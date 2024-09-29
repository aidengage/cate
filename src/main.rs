use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::string::ToString;

const UPLOAD_DIR: &str = "/Users/aidengage/dev/senior/cate/file-uploaded/";
fn check_file(file_path: &str) -> bool {
    if let Ok(_file) = File::open(file_path) {
        true
    } else {
        false
    }
}

// fn check_file(file_path: &str) {
//     let file_path_result = Path::new(file_path).try_exists();
//     let checked_path = match file_path_result {
//         Ok(path) => path,
//         Err(error) => panic!("{}: file does not exist", error),
//     };
// }

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

fn main() {
    // println!("{}", check_file("/Users/aidengage/dev/senior/cate/file-for-upload/fabric-api-0.103.0+1.21.1.jar"));
    // println!("{}", check_file("/Users/aidengage/dev/senior/cate/file-for-upload/whatiasdohe.txt"));
    // check_file("/Users/aidengage/dev/senior/cate/file-for-upload/fabric-api-0.103.0+1.21.1.jar");
    // check_file("/Users/aidengage/dev/senior/cate/file-for-upload/whatiasdohe.txt");

    let path = "/Users/aidengage/dev/senior/cate/file-for-upload/fabric-api-0.103.0+1.21.1.jar".to_string();
    move_file(&path);

    // let file_name = get_file_name(&path);
    // println!("File name: {}", file_name);
    // let file_data = dir_to_vec(path);
    // vec_to_file(file_data, file_name);

    // let mut file = File::create(UPLOAD_DIR.to_string() + file_name.as_str()).unwrap();
    // file.write_all(&file_data).unwrap();
}