use std::fs;
use std::fs::File;
use std::fs::metadata;
use std::io::{BufWriter, Read, Write};
use std::net::SocketAddrV4;
use std::net::Shutdown;
use std::net::{TcpStream};
use std::path::Path;
use std::io::Result;
use std::fs::OpenOptions;
use std::mem;

use crate::{PULL_DIR, PUSH_DIR, PORT, LINK_FILE, USER_DOMAIN, USER_IP};

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
        remove_file(PUSH_DIR.to_string() + file_name.as_str());
    }
}

fn remove_spaces(file_name: String) -> String {
    let mut processed_string = String::new();

    for char in file_name.chars() {
        if char != ' ' {
            processed_string.push(char);
        }
    }

    processed_string
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
}

pub fn send_file() -> Result<()> {

    let paths = fs::read_dir(&*PULL_DIR.as_str())?;
    for path in paths {
        let directory = path?.path().display().to_string();
        // println!("directory: {}", directory);
        let file_name = get_file_name(&directory);
        // println!("file name: {}", file_name);
        if file_name.as_bytes()[0] as char != '.' {
            let name_of_file = file_name.clone();
            // println!("name of file: {}", name_of_file);
            let user_ip_clone = USER_IP.clone();
            let ip = user_ip_clone.lock().unwrap();
            if let Ok(mut stream) = TcpStream::connect(SocketAddrV4::new(*ip, PORT)) {
                mem::drop(ip);
                println!("Connected to the server on {:?}", stream.peer_addr()?);

                let full_path = PULL_DIR.to_string() + name_of_file.as_str();

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

fn receive_link(mut stream: TcpStream) {
    let mut message_length_buffer = [0u8; 8];
    stream.read_exact(&mut message_length_buffer).expect("length issue");

    let message_length = u64::from_be_bytes(message_length_buffer);
    let mut message_buffer = vec![0u8; message_length as usize];
    stream.read_exact(&mut message_buffer).expect("link issue");

    let message = String::from_utf8(message_buffer).unwrap();
    println!("link? {}", message);
    let extracted_domain = USER_DOMAIN.lock().unwrap();
    println!("extracted: {:?}", extracted_domain);
    let domain = remove_spaces(extracted_domain.clone());
    println!("domain: {}", domain);
    let link = create_link(domain, message);
    append_file(LINK_FILE.to_string(), link.as_str()).expect("failed to write to file");
}

fn append_file(file_path: String, content: &str) -> Result<()> {
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

fn create_link(domain: String, cat_link: String) -> String {
    let user_ip_clone = USER_IP.clone();
    let ip = user_ip_clone.lock().unwrap();
    let mut link = String::new();
    if domain == "" {
        link.push_str(&*ip.to_string().as_str());
        link.push_str(&cat_link);
    } else {
        link.push_str(&domain);
        link.push_str(&cat_link);
    }
    println!("created link: {}", link);
    link
}