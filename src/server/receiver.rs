use std::fs::File;
use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::{fs, thread};
use std::io::{Read, Write};

use crate::{ADDR, PORT, UPLOAD_DIR};

fn vec_to_file(vec: Vec<u8>, file_name: String) {
    if vec.len() == 0 {
        return;
    } else {
        let mut file = File::create(UPLOAD_DIR.to_string() + file_name.as_str()).unwrap();
        file.write_all(&vec).unwrap();
    }
}

pub fn receive_file(/*mut file: &Vec<u8>*/) {
    let listener = TcpListener::bind(SocketAddrV4::new(ADDR, PORT)).unwrap();
    println!("{:?}", listener);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(error) => {
                println!("failed to accept connection: {}", error);
            }
        }
    }
}

pub fn handle_client(mut stream: TcpStream) {

    let mut name_length_buffer = [0u8; 8];
    // issue when this receives nothing
    // when client shutdown down connection without sending anything
    stream.read_exact(&mut name_length_buffer).unwrap();
    let name_len = u64::from_be_bytes(name_length_buffer);

    let mut name_buffer = vec![0u8; name_len as usize];
    stream.read_exact(&mut name_buffer).unwrap();
    let file_name = String::from_utf8(name_buffer).unwrap();
    println!("name: {}", file_name);

    let mut length_buffer = vec![0; 8];
    if let Err(error) = stream.read_exact(&mut length_buffer) {
        println!("failed to read length: {}", error);
    }
    let file_size: u64 = u64::from_be_bytes(length_buffer.try_into().unwrap());
    println!("file size in bytes: {}", file_size);


    let mut buffer = Vec::new();
    let mut temp_buffer = [0; 1024];
    loop {
        // Read from the stream into the buffer
        let bytes_read = stream.read(&mut temp_buffer).unwrap();
        print!("|");

        if bytes_read == 0 {
            println!("\nfile received, sending link");
            break;
        } else {
        }

        // Process the chunk of data (for demonstration, we append to the data vector)
        buffer.extend_from_slice(&temp_buffer[..bytes_read]);
    }

    vec_to_file(buffer, file_name.to_string());
    let tcp_clone = stream.try_clone().unwrap();
    let link = generate_link(file_name);
    send_link(tcp_clone, link);
}

///////////////////////////////////
//      send back to client      //
///////////////////////////////////

fn send_link(mut stream: TcpStream, link: String) {
    // let message_back = "hello from server this is pain";
    // let message_length = message_back.len() as u64;
    // println!("message length: {:?}", message_length.to_be_bytes());
    // println!("message: {:?}", message_back);
    // stream.write_all(&message_length.to_be_bytes()).expect("bang bang bang bang bang bang bang bang");
    // stream.write_all(message_back.as_bytes()).expect("could not send file");
    let link_length = link.len() as u64;
    println!("message: {:?}", link);
    stream.write_all(&link_length.to_be_bytes()).expect("bang bang bang bang bang bang bang bang");
    stream.write_all(link.as_bytes()).expect("could not send file");
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

fn generate_link(file_name: String) -> String {
    // let paths = fs::read_dir(UPLOAD_DIR);
    // for path in paths {
    //     let directory = path?.path().display().to_string();
    //     let file_name = get_file_name(&directory);
    //     if file_name.as_bytes()[0] as char != '.' {
    //
    //     }
    // }

    let link = ADDR.to_string() + "/files/" + file_name.as_str();
    println!("link: {}", link);
    link
    // Ok(())
}