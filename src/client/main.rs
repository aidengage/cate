//slint
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use slint;

use std::error::Error;

slint::include_modules!();
use std::fs;
use std::fs::File;
use std::fs::metadata;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::net::Shutdown;
use std::net::{TcpStream};
use std::path::Path;

// const UPLOAD_DIR: &str = "/Users/aidengage/dev/senior/cate/file-uploaded/";
const PULL_DIR: &str = "/Users/aidengage/dev/senior/cate/upload/";
const DISCARD: &str = "/Users/aidengage/dev/senior/cate/discard/";
// const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const ADDR: Ipv4Addr = Ipv4Addr::new(74,130,78,72);
// const ADDR: Ipv4Addr = Ipv4Addr::new(192,168,1,104);
const PORT: u16 = 8000;



// fn check_file(file_path: &str) -> bool {
//     if let Ok(_file) = File::open(file_path) {
//         true
//     } else {
//         false
//     }
// }

fn dir_to_vec(file_path: String) -> Vec<u8> {
    let clean_path: String = file_path.clone().trim().to_string();
    if Path::new(clean_path.as_str()).exists() {
        let file_contents: Vec<u8> = fs::read(clean_path).unwrap();
        file_contents
    } else {
        Vec::new()
    }
}

// fn vec_to_file(vec: Vec<u8>, file_name: String) {
//     if vec.len() == 0 {
//         return;
//     } else {
//         let mut file = File::create(UPLOAD_DIR.to_string() + file_name.as_str()).unwrap();
//         file.write_all(&vec).unwrap();
//     }
// }

fn vec_to_discard(vec: Vec<u8>, file_name: String) {
    if vec.len() == 0 {
        return;
    } else {
        // let mut file = File::create(UPLOAD_DIR.to_string() + file_name.as_str()).unwrap();
        // let mut file = File::create("/Users/aidengage/dev/senior/cate/discard/".to_string() + file_name.as_str()).unwrap();
        let mut file = File::create(DISCARD.to_string() + file_name.as_str()).unwrap();
        file.write_all(&vec).unwrap();
    }
}

fn remove_file(file_path: String) {
    fs::remove_file(file_path).unwrap();
}

fn get_file_name(file_path: &String) -> String {
    let mut reverse_file_name = String::new();

    let reverse_path = file_path.chars().rev().collect::<String>();
    for c in reverse_path.chars() {
        if c != '/' {
            reverse_file_name.push(c);
        } else {
            break;
        }
    }
    let file_name = reverse_file_name.chars().rev().collect::<String>();
    file_name
}

// hmmmmm
// fn move_file(file_path: String) {
//     if check_file(&file_path) {
//         let file_vector = dir_to_vec(file_path.to_string());
//         // vec_to_file(file_vector, get_file_name(file_path));
//         vec_to_discard(file_vector, file_path.to_string());
//     } else {
//         println!("File does not exist");
//     }
// }

fn send_file() -> std::io::Result<()> {
    println!("Hello Client!");

    // let paths = fs::read_dir("/Users/aidengage/dev/senior/cate/file-for-upload/")?;
    let paths = fs::read_dir(PULL_DIR)?;
    // let mut directory = String::new();
    // let mut file_name = String::new();
    for path in paths {
        let directory = path?.path().display().to_string();
        let file_name = get_file_name(&directory);
        // file_name = get_file_name(&path?.path().display().to_string());
        if file_name.as_bytes()[0] as char != '.' {
            // println!("Path: {}", directory);

            // }
            let name_of_file = file_name.clone();
            if let Ok(mut stream) = TcpStream::connect(SocketAddrV4::new(ADDR, PORT)) {
                println!("Connected to the server on {:?}", stream.peer_addr()?);

                let full_path = PULL_DIR.to_string() + name_of_file.as_str();
                let name_vec = file_name.into_bytes();
                let name_len = name_vec.len().to_be_bytes().to_vec();

                stream.write(&name_len)?;
                // println!("name vec: {:?}", name_vec);
                stream.write_all(&name_vec)?;

                let mut file_size = 0;
                match metadata(&full_path) {
                    Ok(metadata) => {
                        file_size = metadata.len();
                        println!("File size: {}", file_size);
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }

                let message = dir_to_vec(full_path.clone());

                // let length = vec![message.len() as u8];
                let size_array = file_size.to_be_bytes().to_vec();

                // println!("size array: {:?}", size_array);

                stream.write(&size_array)?;

                match message[..] {

                    // "#END#" => stream.shutdown(Shutdown::Both).expect("Shutdown Failed!"),
                    [] => stream.shutdown(Shutdown::Both).expect("shutdown call failed"),
                    _ => {
                        println!("SENT!");
                        stream.write(&message)?;
                        vec_to_discard(message, name_of_file.clone());
                        remove_file(full_path.to_string());
                        // move_file(DISCARD.to_string() + file_name.as_str());
                    }
                }
            } else {
                println!("Couldn't connect to server...");
            }
        }
    }

    Ok(())
}

// use slint::WindowSize;
// use slint::PhysicalSize;
// use slint::LogicalSize;
// use slint::Weak;

// pub struct LogicalSize {
//     pub width: f32,
//     pub height: f32,
// }
//
// pub struct PhysicalSize {
//     pub width: u32,
//     pub height: u32,
// }

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    // let window = ui.window();
    // window.set_title("Cate");
    // let app: App = App::new().unwrap();
    // let weak: Weak<App> = app.as_weak();
    // let size: PhysicalSize = PhysicalSize::new(1920, 1080);
    // let logical_size = LogicalSize::new(800, 600);

    // ui.window().set_size(PhysicalSize::new(1280, 720));
    // app.window().set_size(PhysicalSize::new(1280, 720));
    println!("window size: {:?}", ui.window().size());
    ui.on_request_increase_value({
        // let ui_handle = ui.as_weak();
        move || {
            // let ui = ui_handle.unwrap();
            send_file().unwrap();
        }
    });

    ui.run()?;
    // app.run()?;

    Ok(())
}


// slint::slint! {
//     import {Button, VerticalBox, HorizontalBox} from "std-widgets.slint";
//     export component App inherits Window {
//         width: 1280px;
//         height: 720px;
//
//         in property <bool> data_exists;
//         callback upload_file_clicked <=> upload_btn.clicked;
//
//         VerticalBox {
//             Text {
//                 text: data_exists ? "Found existing data, loading..." : "Please upload an MBOX or JSON file."; }
//             HorizontalBox {
//                 upload_btn := Button { text: "Upload"; }
//             }
//         }
//     }
// }


// fn main() {
//     send_file().unwrap();
// }