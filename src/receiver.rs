use std::fs::File;
use std::net::{Shutdown, SocketAddrV4, TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};

use crate::{ADDR, PORT, UPLOAD_DIR};

fn vec_to_file(vec: Vec<u8>, file_name: String) {
    if vec.len() == 0 {
        return;
    } else {
        // let mut file = File::create(UPLOAD_DIR.to_string() + file_name.as_str()).unwrap();
        // let mut file = File::create(UPLOAD_DIR.to_string() + "file.cpp").unwrap();
        // let mut file = File::create(UPLOAD_DIR.to_string() + "cate2.zip").unwrap();
        let mut file = File::create(UPLOAD_DIR.to_string() + &file_name).unwrap();
        // let mut file = File::create(UPLOAD_DIR.to_string() + "temp.txt").unwrap();
        // let mut file = File::create(UPLOAD_DIR.to_string() + "newcpp.txt").unwrap();
        // let mut file = File::create(UPLOAD_DIR.to_string() + "fabric-api.jar").unwrap();
        // let mut file = File::create(UPLOAD_DIR.to_string() + "fire.zip").unwrap();
        file.write_all(&vec).unwrap();
    }
}

pub fn receive_file(/*mut file: &Vec<u8>*/) {
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

pub fn handle_client(mut stream: TcpStream, /* size: u64 */ ) {

    let mut name_length_buffer = [0u8; 8];
    stream.read_exact(&mut name_length_buffer).unwrap();
    println!("name buffer: {:?}", name_length_buffer);
    let name_len = u64::from_be_bytes(name_length_buffer);

    let mut name_buffer = vec![0u8; name_len as usize];
    stream.read_exact(&mut name_buffer).unwrap();
    let file_name = String::from_utf8(name_buffer).unwrap();
    println!("name: {}", file_name);

    let mut length_buffer = vec![0; 8];
    // loop {
        if let Err(error) = stream.read_exact(&mut length_buffer) {
            println!("failed to read length: {}", error);
            // break;
        }
        let length: Vec<u8> = length_buffer.to_vec();
        // let file_size: u64 = u64::from_be_bytes(length_buffer.try_into().unwrap());
        // println!("file size: {}", file_size);
        // println!("size var: {}", size);
        println!("length array: {:?}", length);
        // break;
    // }

    // let mut buffer = [0; 1024];
    let mut buffer = Vec::new();
    let mut temp_buffer = [0; 1024];

    println!("received:");
    loop {
        // Read from the stream into the buffer
        let bytes_read = stream.read(&mut temp_buffer).unwrap();
        print!("|");

        if bytes_read == 0 {
            // Connection closed or end of stream
            println!("\nClient disconnected.");
            break;
        }

        // Process the chunk of data (for demonstration, we append to the data vector)
        buffer.extend_from_slice(&temp_buffer[..bytes_read]);
    }

    println!("buffer size {}", buffer.len());
    // vec_to_file(buffer, UPLOAD_DIR.to_string());
    vec_to_file(buffer, file_name.to_string());
}