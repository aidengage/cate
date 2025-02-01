use std::fs::File;
use std::io::{Read, Write};
use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::thread;

use crate::{ADDR, PORT, UPLOAD_DIR};

// vec_to_file takes a vector of u8 data and a String file name
// and IF the vector is NOT empty, create a new file with the
// corresponding file name and naming format
fn vec_to_file(vec: Vec<u8>, file_name: String) {
    // function does nothing and exits if the file vector is empty
    // else it creates a file at the UPLOAD_DIR string location and writes the vectors contents to the file
    if vec.len() == 0 {
        return;
    } else {
        let mut file =
            File::create(UPLOAD_DIR.to_string() + remove_spaces(file_name).as_str()).unwrap();
        file.write_all(&vec).unwrap();
    }
}

// receive_file runs and listens on the ADDR and PORT designated
// in the server main file
//
// need to figure out multi threading i think?
// the program freezes whenever i upload a file
pub fn receive_file() {
    // opens the specified port on the system/server as a tcplistener
    let listener = TcpListener::bind(SocketAddrV4::new(ADDR, PORT)).unwrap();
    println!("{:?}", listener);

    // loops through the streams actively connecting to the listener
    // spawning a thread for each connection
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

// handle_client takes a tcpstream and waits to receive a file/information
// does a bunch of stuff im about to document
pub fn handle_client(mut stream: TcpStream) {
    // buffer of 8 u8 values initialized to 0 to represent the byte
    // length of the name of the file the server is receiving from the user
    // convertable to u64
    let mut name_length_buffer = [0u8; 8];
    // issue when this receives nothing
    // when client shutdown down connection without sending anything
    // ^^ this might be fixed i actually need to test this somehow

    // takes the tcpstream and reads the first set of bytes into the name_length_buffer
    stream.read_exact(&mut name_length_buffer).unwrap();

    // stores the information in the name length buffer in a human printable format
    // u64 data by converting from bytes
    let name_len = u64::from_be_bytes(name_length_buffer);

    // creates the name_buffer from the name_len variable to represent the
    // byte data coming to receiving the name of the file from the client
    let mut name_buffer = vec![0u8; name_len as usize];

    // tcpstream from input and reads the next set of data into the name_buffer
    stream.read_exact(&mut name_buffer).unwrap();

    // convert the name_buffer byte data into human readable file_name String
    let file_name = String::from_utf8(name_buffer).unwrap();
    println!("name: {}", file_name);

    // creates length_buffer to represent the length of the entire file in byte data
    // used to set the buffer size to intake the actual file's contents
    let mut length_buffer = vec![0; 8];

    // no idea when this would happen, might mess with this later
    // might only happen when connection is cut or timed out?
    if let Err(error) = stream.read_exact(&mut length_buffer) {
        println!("failed to read length: {}", error);
    }

    // converts length_buffer to file_size u64 variable for human readability
    // and to generate the file buffers later
    let file_size: u64 = u64::from_be_bytes(length_buffer.try_into().unwrap());
    println!("file size in bytes: {}", file_size);

    // main buffer to place the contents of the files the server receives
    // initialized to an empty vector
    let mut buffer = Vec::new();

    // temporary buffer to take single bytes at a time and place them onto
    // the end of the buffer above
    let mut temp_buffer = [0; 1024];

    // loops until the bytes_read variable is zero, indicating the end of the file
    loop {
        // read from tcpstream into the temporary one byte buffer
        // try placing this in the else statement below on a later date when i am going through the code
        let bytes_read = stream.read(&mut temp_buffer).unwrap();
        print!("|");

        if bytes_read == 0 {
            println!("\nfile received, sending link");
            break;
        } else {
            // let bytes_read = stream.read(&mut temp_buffer).unwrap();
            // print!("|");
        }

        // appends the contents of the temporary buffer onto the main buffer
        // through the entire bytes_read marker, should be 1024 ~~> 1B
        buffer.extend_from_slice(&temp_buffer[..bytes_read]);
    }

    // takes the contents of the main file buffer and the file_name
    // and converts it into a file on your system, should be placed at
    // your docker location of apache webserver
    vec_to_file(buffer, file_name.to_string());

    // creates a clone of the tcpstream input paramter of the function to send a link back
    let tcp_clone = stream.try_clone().unwrap();

    // runs the generate_half_link function based on the file_name variable
    // and stores it in the link variable
    let link = generate_half_link(file_name);

    // runs send_link function with the tcp_clone to send the link back to the, already established,
    // client/server connection
    send_link(tcp_clone, link);
}

///////////////////////////////////
//      send back to client      //
///////////////////////////////////

// send_link takes the tcpstream and link String variables and sends that
// link back to the client at the end of the tcpstream
fn send_link(mut stream: TcpStream, link: String) {
    // must get length of the string before sending the entire string
    let link_length = link.len() as u64;
    println!("message: {:?}", link);

    // send the link_length in bytes first
    stream
        .write_all(&link_length.to_be_bytes())
        // schizo debug message
        .expect("bang bang bang bang bang bang bang bang");

    // send the link string as bytes next
    stream
        .write_all(link.as_bytes())
        .expect("could not send file");
}

// processing funciton to replace the spaces (' ') with underscores ('_')
// need to be compatible with url formatting
fn remove_spaces(file_name: String) -> String {
    // creates empty string as a stack to push chars onto the end of the string
    let mut processed_string = String::new();

    // loop through the chars in file_name
    for char in file_name.chars() {
        if char == ' ' {
            processed_string.push('_');
        } else {
            processed_string.push(char);
        }
    }

    // returns the processed_string at the end of the function
    processed_string
}

// generates the first half of the link containing the path of the file in the webserver location
fn generate_half_link(file_name: String) -> String {
    // runs the remove_spaces function on file_name to return a processed_name with no spaces
    let processed_name = remove_spaces(file_name);

    // files are stored as /files/ + processed_name
    let link = "/files/".to_owned() + processed_name.as_str();
    println!("link: {}", link);

    // return link
    link
}
