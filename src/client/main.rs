use std::fs;
use std::fs::File;
use std::fs::metadata;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::net::Shutdown;
use std::net::{TcpStream};
use std::path::Path;

const UPLOAD_DIR: &str = "/Users/aidengage/dev/senior/cate/file-uploaded/";
const PULL_DIR: &str = "/Users/aidengage/dev/senior/cate/upload/";
const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
// const ADDR: Ipv4Addr = Ipv4Addr::new(74,130,78,72);
// const ADDR: Ipv4Addr = Ipv4Addr::new(192,168,1,104);
const PORT: u16 = 8000;



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

// hmmmmm
fn vec_to_file(vec: Vec<u8>, file_name: String) {
    if vec.len() == 0 {
        return;
    } else {
        let mut file = File::create(UPLOAD_DIR.to_string() + file_name.as_str()).unwrap();
        file.write_all(&vec).unwrap();
    }
}

fn vec_to_discard(vec: Vec<u8>, file_name: String) {
    if vec.len() == 0 {
        return;
    } else {
        // let mut file = File::create(UPLOAD_DIR.to_string() + file_name.as_str()).unwrap();
        let mut file = File::create("/Users/aidengage/dev/senior/cate/discard/".to_string() + file_name.as_str()).unwrap();
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

// hmmmmm
fn move_file(file_path: &String) {
    if check_file(file_path) {
        let file_vector = dir_to_vec(file_path.to_string());
        // vec_to_file(file_vector, get_file_name(file_path));
        vec_to_discard(file_vector, file_path.to_string());
    } else {
        println!("File does not exist");
    }
}

// old shit


fn main() -> std::io::Result<()> {
    // let file_path = "/Users/aidengage/dev/senior/cate/upload/fabric-api-0.103.0+1.21.1.jar";
    // let file_path = "/Users/aidengage/dev/senior/cate/upload/Fire.zip";
    // let file_path = "/Users/aidengage/dev/senior/cate/upload/newMain.cpp";
    // let file_path = "/Users/aidengage/dev/senior/cate/upload/cate-checkpoint2.zip";
    // let file_path = "/Users/aidengage/dev/senior/cate/upload/temp.txt";

    println!("Hello Client!");

    // let paths = fs::read_dir("/Users/aidengage/dev/senior/cate/file-for-upload/")?;
    let paths = fs::read_dir(PULL_DIR)?;
    let mut directory = String::new();
    let mut file_name = String::new();
    for path in paths {
        directory = path?.path().display().to_string();
        file_name = get_file_name(&directory);
        // println!("name: {}", path.unwrap().path().display());
        println!("Path: {}", directory);
        // println!("Name: {}", file_name);
        // vec_to_discard(dir_to_vec(&directory), file_name);
        // fs::remove_file(directory)?;
    }
    let name_of_file = file_name.clone();
    if let Ok(mut stream) = TcpStream::connect(SocketAddrV4::new(ADDR, PORT)) {
        println!("Connected to the server on {:?}", stream.peer_addr()?);
        let full_path = PULL_DIR.to_string() + name_of_file.as_str();

        let name_vec = file_name.into_bytes();
        let name_len = name_vec.len().to_be_bytes().to_vec();
        stream.write(&name_len)?;
        println!("name vec: {:?}", name_vec);
        stream.write_all(&name_vec)?;
        // stream.shutdown(Shutdown::Both)?;
        // }
        // if let Ok( mut stream) = TcpStream::connect(SocketAddrV4::new(ADDR, PORT)) {

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

        // let message = dir_to_vec(PULL_DIR.to_string() + file_name.as_str());
        let message = dir_to_vec(full_path);
        // let message = dir_to_vec(file_path.to_string());
        // println!("message length: {}", message.len());
        // println!("{:?}", message);
        // let length = message.len();
        // match metadata(PULL_DIR.to_string() + file_name.as_str()) {
        // match metadata(full_path.clone()) {

        // match metadata(&full_path) {
        //     Ok(metadata) => {
        //         file_size = metadata.len();
        //         println!("File size: {}", file_size);
        //     }
        //     Err(error) => {
        //         println!("Error: {}", error);
        //     }
        // }

        let length = vec![message.len() as u8];
        // let length = vec![file_size];
        let size_array = file_size.to_be_bytes().to_vec();
        println!("size array: {:?}", size_array);
        // println!("length: {:?}", file_size);
        // stream.write(&length)?;

        stream.write(&size_array)?;

        // let name_vec = file_name.into_bytes();
        // stream.write_all(&name_vec)?;
        // println!("name_vec: {:?}", name_vec);
        // println!("name: {}", String::from_utf8_lossy(&name_vec));

        // send length to server
        // let _ = stream.write(&length);
        // stream.write(length.to_string().as_bytes())?;

        // println!("Message: {}", message);
        // match message.as_str() {

        match message[..] {

            // "#END#" => stream.shutdown(Shutdown::Both).expect("Shutdown Failed!"),
            [] => stream.shutdown(Shutdown::Both).expect("shutdown call failed"),
            _ => {
                println!("SENT!");
                // let vec = &message.into_bytes();
                // println!("Message: {}", String::from_utf8_lossy(&vec));
                // println!("{:?}", vec);
                // stream.write(&message.into_bytes())?;

                // stream.write(&vec)?;

                stream.write(&message)?;
            }
        }
    } else {
        println!("Couldn't connect to server...");
    }
    // end of paths for loop
    // }

    Ok(())
}