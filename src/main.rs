use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::string::ToString;

use std::thread;
use std::str;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::net::{TcpListener, TcpStream, Shutdown};

const UPLOAD_DIR: &str = "/Users/aidengage/dev/senior/cate/file-uploaded/";
const ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST;
const PORT: u16 = 8000;
const EXTRA_BYTES: u32 = 15;


// static EXCLUDED_CHARS: Vec<char> = Vec::
    // ['/', '\', '(', ')', '<', '>', ':', '"', '|', '*', '?'];

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
        // println!("{}", file_name); // broken
        // println!("{}", UPLOAD_DIR);
        // let mut file = File::create(UPLOAD_DIR.to_string() + file_name.as_str()).unwrap();
        let mut file = File::create(UPLOAD_DIR.to_string() + "file.cpp").unwrap();
        file.write_all(&vec).unwrap();
    }
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

fn move_file(file_path: &String) {
    if check_file(file_path) {
        let file_vector = dir_to_vec(file_path.to_string());
        vec_to_file(file_vector, get_file_name(file_path));
    } else {
        println!("File does not exist");
    }
}

// fn ret_val(file: Vec<u8>) -> Vec<u8> {
//     file
// }

fn receive_file(/*mut file: &Vec<u8>*/) {
    let listener = TcpListener::bind(SocketAddrV4::new(ADDR, PORT)).unwrap();
    println!("{:?}", listener);
    // file = &Vec::new();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(error) => {
                println!("failed to accept connection: {}", error);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, /*mut file: &Vec<u8>*/) {
    let mut length_buffer = [0; 4];

    // let mut buffer = [0; 512];
    let mut received_bytes: Vec<u8> = Vec::new();
    loop {
        if let Err(error) = stream.read(&mut length_buffer) {
        // if let Err(error) = stream.read_exact(&mut received_bytes) {
            println!("failed to read length: {}", error);
            break;
        }
        let length = u32::from_be_bytes(length_buffer);
        println!("expecting {} bytes from the client: ", length /* + EXTRA_BYTES */);

        let mut buffer = vec![0; length as usize];

        // let mut bytes_received = 0u32;
        let mut num_bytes_read = 0;
        break;
        /*
        while num_bytes_read != length {

            match stream.read(&mut buffer) {
                Ok(size) if size > 0 => {
                    let received = str::from_utf8(&buffer[..size]).unwrap_or("");
                    received_bytes = buffer[..size].to_vec();
                    stream.write(&buffer[..size]).unwrap();
                    num_bytes_read = received_bytes.len() as u32;

                    // println!("file received");
                    // println!("received: {}", received);

                    // let received_vec: Vec<u8> = received.trim().as_bytes().to_vec();

                    if received.trim() == "#END#" {
                        println!("Exiting client");
                        stream.shutdown(Shutdown::Both).unwrap();

                        // break Vec::new();
                        break;
                    }

                    vec_to_file(received_bytes, UPLOAD_DIR.to_string());
                    break;
                    // return received
                    // file = &received_vec;
                    // return received_vec;
                }
                Ok(_) => {
                    println!("connection closed by client");
                    // break Vec::new();
                    break;
                }
                Err(error) => {
                    println!("failed to read from client: {}", error);
                    // break Vec::new();
                    break;
                }
            }
        }*/
    }
}


fn main() {
    // println!("{}", check_file("/Users/aidengage/dev/senior/cate/file-for-upload/fabric-api-0.103.0+1.21.1.jar"));
    // println!("{}", check_file("/Users/aidengage/dev/senior/cate/file-for-upload/whatiasdohe.txt"));
    // check_file("/Users/aidengage/dev/senior/cate/file-for-upload/fabric-api-0.103.0+1.21.1.jar");
    // check_file("/Users/aidengage/dev/senior/cate/file-for-upload/whatiasdohe.txt");

    // let path = "/Users/aidengage/dev/senior/cate/file-for-upload/fabric-api-0.103.0+1.21.1.jar".to_string();
    // move_file(&path);

    // let mut file: Vec<u8>;
    // receive_file(&file);
    receive_file();
}