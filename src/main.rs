use std::{env, fs};
use std::fs::File;
use std::io::{Read, Write};
use std::io;
use std::path::Path;
use std::string::ToString;

// fn check_file() {
//
// }
const UPLOAD_DIR: &str = "/Users/aidengage/dev/senior/cate/file-uploaded/";

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
        let mut file = File::create(UPLOAD_DIR.to_string() + file_name.as_str()).unwrap();
        file.write_all(&vec).unwrap();
    }
}

fn file_name(file_path: &String) -> String {
    let mut file_name = String::new();
    let mut reverse_file_name = String::new();
    let reverse_path = file_path.chars().rev().collect::<String>();
    for c in reverse_path.chars() {
        if c != '/' {
            reverse_file_name.push(c);
        } else {
            break;
        }
    }
    file_name = reverse_file_name.chars().rev().collect::<String>();
    file_name
}

fn main() {
    let path = "/Users/aidengage/dev/senior/cate/file-for-upload/fabric-api-0.103.0+1.21.1.jar".to_string();
    let file_name = file_name(&path);
    println!("File name: {}", file_name);

    let file_data = dir_to_vec(path);
    vec_to_file(file_data, file_name);
    // let mut file = File::create(UPLOAD_DIR.to_string() + file_name.as_str()).unwrap();
    // file.write_all(&file_data).unwrap();
}