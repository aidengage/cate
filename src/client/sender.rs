use std::fs;
use std::fs::File;
use std::fs::metadata;
use std::io::{Read, Write};
use std::net::SocketAddrV4;
use std::net::Shutdown;
use std::net::{TcpStream};
use std::path::Path;
use std::io::Result;

use gtk::prelude::*;

use crate::{PULL_DIR, DISCARD, ADDR, PORT};

fn check_file(file_path: &str) -> bool {
    if let Ok(_file) = File::open(file_path) {
        true
    } else {
        false
    }
}

fn dir_to_vec(file_path: String) -> Vec<u8> {
    let clean_path: String = file_path.clone().trim().to_string();
    if Path::new(clean_path.as_str()).exists() {
        let file_contents: Vec<u8> = fs::read(clean_path).unwrap();
        file_contents
    } else {
        Vec::new()
    }
}

fn vec_to_file(vec: Vec<u8>, file_name: String) {
    if vec.len() == 0 {
        return;
    } else {
        let mut file = File::create(PULL_DIR.to_string() + file_name.as_str()).unwrap();
        file.write_all(&vec).unwrap();
    }
}

fn vec_to_discard(vec: Vec<u8>, file_name: String) {
    if vec.len() == 0 {
        return;
    } else {
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
pub fn move_file(file_path: String) {
    if check_file(&file_path) {
        let file_vector = dir_to_vec(file_path.to_string());
        vec_to_file(file_vector, get_file_name(&file_path));
    } else {
        println!("File does not exist");
    }
}

pub fn send_file() -> Result<()> {
    println!("Hello Client!");
    // let mut progress_var = 0;

    // let paths = fs::read_dir("/Users/aidengage/dev/senior/cate/file-for-upload/")?;
    let paths = fs::read_dir(PULL_DIR)?;
    for path in paths {
        println!("paths print");
        let directory = path?.path().display().to_string();
        let file_name = get_file_name(&directory);
        if file_name.as_bytes()[0] as char != '.' {
            println!("if char");
            let name_of_file = file_name.clone();
            println!("name of file: {}", name_of_file);
            if let Ok(mut stream) = TcpStream::connect(SocketAddrV4::new(ADDR, PORT)) {
                println!("if connect");
                println!("Connected to the server on {:?}", stream.peer_addr()?);

                let full_path = PULL_DIR.to_string() + name_of_file.as_str();

                let mut file_size = 0;
                match metadata(&full_path) {
                    Ok(metadata) => {
                        file_size = metadata.len();
                        // if file_size > isize::MAX as u64 {
                        //     println!("one of your files is too large (over {})", isize::MAX as u64);
                        //     // shutdown causes issue when sending nothing
                        //     stream.shutdown(Shutdown::Both).expect("shutdown call failed");
                        //     continue;
                        // }
                        println!("File size: {}", file_size);
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }

                let name_vec = file_name.into_bytes();
                let name_len = name_vec.len().to_be_bytes().to_vec();

                stream.write(&name_len)?;
                // println!("name vec: {:?}", name_vec);
                stream.write_all(&name_vec)?;
                let message = dir_to_vec(full_path.clone());

                let size_array = file_size.to_be_bytes().to_vec();

                stream.write(&size_array)?;

                match message[..] {

                    // "#END#" => stream.shutdown(Shutdown::Both).expect("Shutdown Failed!"),
                    [] => stream.shutdown(Shutdown::Both).expect("shutdown call failed"),
                    _ => {
                        println!("SENT!");
                        stream.write(&message)?;
                        stream.shutdown(Shutdown::Write)?;

                        vec_to_discard(message, name_of_file.clone());
                        remove_file(full_path.to_string());
                        receive_link(stream).expect("Failed to receive link");
                    }
                }
            } else {
                println!("Couldn't connect to server...");
            }
        }
    }
    Ok(())
}

/////////////////////////////////
//     receive from server     //
/////////////////////////////////

fn receive_link(mut stream: TcpStream) -> Result<()> {
    // if let Ok(mut stream) = TcpStream::connect(SocketAddrV4::new(ADDR, PORT)) {
    println!("connected back to server after send");
    let mut message_length_buffer = [0u8; 8];
    // println!("message length buffer: {:?}", message_length_buffer);
    // message_length_buffer = [0, 0, 0, 0, 0, 0, 0, 17];
    // println!("debug 1");
    stream.read_exact(&mut message_length_buffer)?;

    // println!("message length buffer: {:?}", message_length_buffer);
    // stream.read_to_end(&mut buffer);
    // println!("debug 2");
    let message_length = u64::from_be_bytes(message_length_buffer);
    // println!("Message length: {}", message_length);
    let mut message_buffer = vec![0u8; message_length as usize];
    // println!("message buffer: {:?}", message_buffer);
    // let mut message_buffer = vec![0u8; 17];
    stream.read_exact(&mut message_buffer).unwrap();
    // println!("message buffer: {:?}", message_buffer);

    // let message = String::from_utf8_lossy(&buffer).to_string();
    // let message = String::from_utf8_lossy(&message_buffer).to_string();
    let message = String::from_utf8(message_buffer).unwrap();
    println!("message receive: {}", message);

    Ok(())
}