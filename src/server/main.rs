// slint
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use std::error::Error;
// slint::include_modules!();

mod receiver;

use std::io::{Read, Write};
use std::string::ToString;
use std::str;
use std::net::{Ipv4Addr};

const UPLOAD_DIR: &str = "/Users/aidengage/dev/senior/cate/file-uploaded/";
// const UPLOAD_DIR: &str = "/home/cate/cate/upload/";
const ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST;
// const ADDR: Ipv4Addr = Ipv4Addr::new(192, 168, 1, 104);

const PORT: u16 = 8000;

// fn main() -> Result<(), Box<dyn Error>> {
//     let ui = AppWindow::new()?;
//
//     ui.on_request_increase_value({
//         let ui_handle = ui.as_weak();
//         move || {
//             let ui = ui_handle.unwrap();
//             ui.set_counter(ui.get_counter() + 1);
//         }
//     });
//
//     ui.run()?;
//
//     Ok(())
//     // receiver::receive_file();
// }

fn main () {
    receiver::receive_file();
}