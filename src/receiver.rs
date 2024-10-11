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
        // let mut file = File::create(UPLOAD_DIR.to_string() + "temp.txt").unwrap();
        let mut file = File::create(UPLOAD_DIR.to_string() + "newcpp.txt").unwrap();
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



pub fn handle_client(mut stream: TcpStream, /*mut file: &Vec<u8>*/) {
    let mut length_buffer = vec![0; 8];
    // const file_size: u64 = 0;
    loop {
        if let Err(error) = stream.read_exact(&mut length_buffer) {
            println!("failed to read length: {}", error);
            break;
        }
        let length: Vec<u8> = length_buffer.to_vec();
        let file_size: u64 = u64::from_be_bytes(length_buffer.try_into().unwrap());
        // file_size = u64::from_be_bytes(length_buffer.try_into().unwrap());
        println!("file size: {}", file_size);
        println!("length: {:?}", length);
        break;

        // match stream.read(&mut length_buffer) {
        //     Ok(size) => {
        //         println!("size: {}", size);
        //         break;
        //     }
        //     Err(error) => {
        //         println!("failed to read from client: {}", error);
        //         break;
        //     }
        // }
    }

    let mut buffer = [0; 1024];
    let mut received_bytes: Vec<u8> = Vec::new();
    loop {
        // if let Err(error) = stream.read(&mut length_buffer) {
        //     println!("failed to read length: {}", error);
        //     break;
        // }

        // let length: Vec<u8> = length_buffer.to_vec();
        // println!("length: {:?}", length);
        let mut num_bytes_read = 0;
        // println!("num bytes read: {}", num_bytes_read);


        match stream.read(&mut buffer) {
            // size is actually the size of the file no fucking way
            Ok(size) if size > 0 => {
                let received = std::str::from_utf8(&buffer[..size]).unwrap_or("");
                received_bytes = buffer[..size].to_vec();
                stream.write(&buffer[..size]).unwrap();
                num_bytes_read = received_bytes.len() as u32;
                println!("size: {:?}", size);

                if received.trim() == "#END#" {
                    println!("Exiting client");
                    stream.shutdown(Shutdown::Both).unwrap();

                    break;
                }

                println!("vector to a file");
                vec_to_file(received_bytes, UPLOAD_DIR.to_string());
                break;
            }
            Ok(_) => {
                println!("connection closed by client");
                break;
            }
            Err(error) => {
                println!("failed to read from client: {}", error);
                break;
            }
        }
    }
}