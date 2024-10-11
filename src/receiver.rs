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
        // let mut file = File::create(UPLOAD_DIR.to_string() + "newcpp.txt").unwrap();
        let mut file = File::create(UPLOAD_DIR.to_string() + "fabric-api.jar").unwrap();
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
    let mut length_buffer = vec![0; 8];
    loop {
        if let Err(error) = stream.read_exact(&mut length_buffer) {
            println!("failed to read length: {}", error);
            break;
        }
        let length: Vec<u8> = length_buffer.to_vec();
        // let file_size: u64 = u64::from_be_bytes(length_buffer.try_into().unwrap());
        // println!("file size: {}", file_size);
        // println!("size var: {}", size);
        println!("length array: {:?}", length);
        break;
    }

    // let mut buffer = [0; 1024];
    let mut buffer = Vec::new();
    let mut temp_buffer = [0; 1024];
    let mut received_bytes: Vec<u8> = Vec::new();


    // loop {
    //     let bytes_read = stream.read(&mut temp_buffer).unwrap();
    //     if bytes_read == 0 {
    //         break;
    //     }
    //     buffer.extend_from_slice(&temp_buffer[0..bytes_read]);
    // }

    let mut bytes_read = 0;

    loop {
        // Read from the stream into the buffer
        let bytes_read = stream.read(&mut temp_buffer).unwrap();

        if bytes_read == 0 {
            // Connection closed or end of stream
            println!("Client disconnected.");
            break;
        }

        // Process the chunk of data (for demonstration, we append to the data vector)
        buffer.extend_from_slice(&temp_buffer[..bytes_read]);

        // println!("Received {} bytes", bytes_read);
        // Optionally, you can print the data chunk received (e.g., if it's text)
        // println!("Chunk: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
    }

    println!("buffer size {}", buffer.len());
    vec_to_file(buffer, UPLOAD_DIR.to_string());

    /*

    loop {
        // if let Err(error) = stream.read(&mut length_buffer) {
        //     println!("failed to read length: {}", error);
        //     break;
        // }

        // let length: Vec<u8> = length_buffer.to_vec();
        // println!("length: {:?}", length);
        // println!("num bytes read: {}", num_bytes_read);

        // let bytes_read = stream.read(&mut temp_buffer).unwrap();
        // if bytes_read == 0 {
        //     break;
        // }
        // buffer.extend_from_slice(&temp_buffer[0..bytes_read]);

        match stream.read(&mut temp_buffer) {
            // size is actually the size of the file no fucking way
            Ok(size) if size > bytes_read => {

                let received = std::str::from_utf8(&temp_buffer[..size]).unwrap_or("");
                // println!("received: {}", received);
                received_bytes = temp_buffer[..size].to_vec();
                stream.write(&temp_buffer[..size]).unwrap();
                bytes_read += received_bytes.len();
                // println!("bytes read: {}", bytes_read);
                // bytes_read = received_bytes.len() as u32;
                // println!("size: {:?}", size);
                // println!("temp_buffer size: {}", temp_buffer.len());
                buffer.extend_from_slice(&temp_buffer[..size]);
                // println!("buffer size: {}", buffer.len());

                if received.trim() == "#END#" {
                    println!("Exiting client");
                    stream.shutdown(Shutdown::Both).unwrap();

                    break;
                }

                println!("vector to a file");
                // vec_to_file(received_bytes, UPLOAD_DIR.to_string());
                vec_to_file(buffer, UPLOAD_DIR.to_string());
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

     */
}