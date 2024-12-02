use std::{env, fs, io};
use std::error::Error;
use std::fs::File;
use std::fs::metadata;
use std::io::{BufWriter, Read, Write};
use std::net::SocketAddrV4;
use std::net::Shutdown;
use std::net::{TcpStream};
use std::path::Path;
use std::io::Result;
use std::io::ErrorKind;

use std::io::LineWriter;
use std::fs::OpenOptions;

use gtk::prelude::*;

use crate::{PULL_DIR, PUSH_DIR, ADDR, PORT};

// static ROOT_DIR: String = env::var("PROJECT_ROOT").unwrap_or_else(|_| env::current_dir().unwrap().to_str().unwrap().to_string());
// static PULL_DIR: &str = Path::new(&ROOT_DIR).join("pull").to_str().unwrap();
// static PUSH_DIR: &str = Path::new(&ROOT_DIR).join("push").to_str().unwrap();

fn check_file(file_path: &str) -> bool {
    if let Ok(_file) = File::open(file_path) {
        true
    } else {
        false
    }
}

fn dir_to_vec(file_path: String) -> Vec<u8> {
    let clean_path: String = file_path.clone().trim().to_string();
    println!("Cleaning file: {}", clean_path);
    if Path::new(clean_path.as_str()).exists() {
        let file_contents: Vec<u8> = fs::read(clean_path).unwrap();
        file_contents
    } else {
        Vec::new()
    }
}

fn vec_to_file(vec: Vec<u8>, file_name: String) {
    println!("in vec_to_file: {}", file_name);
    println!("PULL_DIR: {}", *PULL_DIR);
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
        let mut file = File::create(PUSH_DIR.to_string() + file_name.as_str()).unwrap();
        // let mut file = File::create(PUSH_DIR.join(file_name.to_string())).unwrap(); // test
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
    println!("in move_file: {}", file_path.clone());
    let file_vector = dir_to_vec(file_path.to_string());
    vec_to_file(file_vector, get_file_name(&file_path));
    // println!("checked file {}", check_file(file_path.as_str()));
    // if check_file(&file_path) {
    //     let file_vector = dir_to_vec(file_path.to_string());
    //     vec_to_file(file_vector, get_file_name(&file_path));
    // } else {
    //     println!("File does not exist");
    // }
}

pub fn send_file() -> Result<()> {
    println!("in send_file");

    // let project_root = env::var("PROJECT_ROOT").unwrap_or_else(|_| env::current_dir().unwrap().to_str().unwrap().to_string());
    // println!("project_root: {}", project_root);
    // let file_path = Path::new(&project_root).join("assets/links.txt");
    // println!("file_path: {:?}", file_path);

    // println!("root dir: {}", *ROOT_DIR);
    // println!("pull dir: {}", *PULL_DIR);
    // println!("push dir: {}", *PUSH_DIR);

    // let mut progress_var = 0;

    // let paths = fs::read_dir("/Users/aidengage/dev/senior/cate/file-for-upload/")?;
    let paths = fs::read_dir(&*PULL_DIR.as_str())?;
    for path in paths {
        // println!("paths print");
        let directory = path?.path().display().to_string();
        println!("directory: {}", directory);
        let file_name = get_file_name(&directory);
        println!("file name: {}", file_name);
        if file_name.as_bytes()[0] as char != '.' {
            println!("if char");
            let name_of_file = file_name.clone();
            println!("name of file: {}", name_of_file);
            if let Ok(mut stream) = TcpStream::connect(SocketAddrV4::new(ADDR, PORT)) {
                println!("if connect");
                println!("Connected to the server on {:?}", stream.peer_addr()?);

                let full_path = PULL_DIR.to_string() + name_of_file.as_str();
                // let full_path = PULL_DIR.join(name_of_file.to_string()); // test

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
                // let message = dir_to_vec(full_path.into_os_string().into_string().unwrap()); // test

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
                        receive_link(stream);
                        // let link = receive_link(stream).0;//.expect("Failed to receive link");
                        // println!("Received link: {}", link);
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

fn receive_link(mut stream: TcpStream) /*-> (String, Result<()>)*/ {
    // if check_connection(stream) {
    // println!("in receive link method");
    // println!("connected back to server after send");
    let mut message_length_buffer = [0u8; 8];
    // println!("message length buffer: {:?}", message_length_buffer);
    // println!("debug 1");
    stream.read_exact(&mut message_length_buffer).expect("length issue");

    // println!("message length buffer: {:?}", message_length_buffer);
    // stream.read_to_end(&mut buffer);
    // println!("debug 2");
    let message_length = u64::from_be_bytes(message_length_buffer);
    // println!("Message length: {}", message_length);
    let mut message_buffer = vec![0u8; message_length as usize];
    // println!("message buffer: {:?}", message_buffer);
    // let mut message_buffer = vec![0u8; 17];
    stream.read_exact(&mut message_buffer).expect("link issue");
    // println!("message buffer: {:?}", message_buffer);

    // let message = String::from_utf8_lossy(&buffer).to_string();
    // let message = String::from_utf8_lossy(&message_buffer).to_string();
    let message = String::from_utf8(message_buffer).unwrap();
    println!("link? {}", message);
    // let mut file = File::create("/Users/aidengage/dev/senior/cate/assets/links.txt").unwrap();
    // file.write(message.as_bytes()).unwrap();
    append_file("/Users/aidengage/dev/senior/cate/assets/links.txt", message.as_str()).expect("failed to write to file");
    // append_file("../../../assets/links.txt", message.as_str()).expect("failed to write to file");

    // line writer
    // let mut line_writer = LineWriter::new(file);
    // line_writer.write_all(&message.as_bytes()).unwrap();
    // line_writer.flush().unwrap();

    // open options
    // let mut append_file = OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open("/Users/aidengage/dev/senior/cate/assets/links.txt");
    // let writer = BufWriter::new(append_file);
    // writeln!(writer, "{}", message.as_str()).expect("message not written");

    // let mut link_file = File::open("/Users/aidengage/dev/senior/cate/assets/links.txt").unwrap();
    // link_file.write(message.as_bytes()).unwrap();



    // println!("message receive: {}", message);
    /*(message, Ok(()))*/
}

fn append_file(file_path: &str, content: &str) -> Result<()> {
    // println!("content: {}", content);
    // open options
    let append_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    let mut writer = BufWriter::new(append_file);
    writeln!(writer, "{}", content).expect("message not written");
    Ok(())
}

// fn check_connection(mut stream: TcpStream) -> Result<()> {
//     match stream.write(&[0]) {
//         Ok(_) => true,
//         Err(e) if e.kind() == ErrorKind::ConnectionReset => {
//             println!("Connection reset by peer");
//             false
//         }
//         Err(_) => false, // Other errors may also indicate a lost connection
//     }
//     Ok(())
// }