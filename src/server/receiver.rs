use std::fs::File;
use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::thread;
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
    // let length: Vec<u8> = length_buffer.to_vec();
    let file_size: u64 = u64::from_be_bytes(length_buffer.try_into().unwrap());
    println!("file size in bytes: {}", file_size);

    // multithreading





    //

    let mut buffer = Vec::new();
    let mut temp_buffer = [0; 1024];
    loop {
        // Read from the stream into the buffer
        let bytes_read = stream.read(&mut temp_buffer).unwrap();
        print!("|");

        if bytes_read == 0 {
            // Connection closed or end of stream
            // progress[0] = 0;
            // stream.write_all([0; 0].as_ref()).unwrap();
            // println!("\nClient disconnected.");
            println!("\nfile received, sending link");
            // let tcp_clone = stream.try_clone().unwrap();
            // send_link(tcp_clone);
            // send_link(stream);
            break;
        } else {
            // send out boolean (single byte representation) to the client
            // client interprets it and progresses the progress bar
            // let tcp_clone = stream.try_clone().unwrap();
            // send_link(tcp_clone);
            // send_link(stream);
            // break;
        }

        // let tcp_clone = stream.try_clone().unwrap();
        // send_link(tcp_clone);
        // Process the chunk of data (for demonstration, we append to the data vector)
        buffer.extend_from_slice(&temp_buffer[..bytes_read]);
    }

    vec_to_file(buffer, file_name.to_string());
    let tcp_clone = stream.try_clone().unwrap();
    send_link(tcp_clone);


    ///////////////////////////////////
    //      send back to client      //
    ///////////////////////////////////

    // let message_back = "hello from server";
    // let message_length = message_back.len() as u64;
    // println!("message length: {:?}", message_length.to_be_bytes());
    // // stream.write_all(&message_length.to_be_bytes()).expect("bang bang bang bang bang bang bang bang");
    // // stream.write_all(message_back.as_bytes()).unwrap();
    // println!("message sent to client: {}", message_back);
}

fn send_link(mut stream: TcpStream) {
    let message_back = "hello from server this is pain";
    let message_length = message_back.len() as u64;
    println!("message length: {:?}", message_length.to_be_bytes());
    println!("message: {:?}", message_back);
    stream.write_all(&message_length.to_be_bytes()).expect("bang bang bang bang bang bang bang bang");
    stream.write_all(message_back.as_bytes()).expect("could not send file");
    // println!("message sent to client: {}", message_back);
}