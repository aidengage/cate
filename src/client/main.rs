mod sender;

use std::net::Ipv4Addr;
use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{gdk, glib, Application, ApplicationWindow, DropTarget, Label};

const PULL_DIR: &str = "/Users/aidengage/dev/senior/cate/pull/";
const DISCARD: &str = "/Users/aidengage/dev/senior/cate/push/";
const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
// const ADDR: Ipv4Addr = Ipv4Addr::new(74,130,78,72);
// const ADDR: Ipv4Addr = Ipv4Addr::new(192,168,1,104);
const PORT: u16 = 8000;
const APP_ID: &str = "cate";

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Cate")
        .default_width(320)
        .default_height(180)
        .build();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let label = Label::builder()
        .label("Drop Files Here")
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    vbox.append(&label);

    let drop_target = DropTarget::new(gdk::FileList::static_type(), gdk::DragAction::COPY);

    drop_target.connect_drop(move |_, value, _, _| {
        // println!("value: {:?}", value);
        // Extract the dropped files
        if let Ok(file_list) = value.get::<gdk::FileList>() {
            for file in file_list.files() {
                let path: PathBuf = file.path().expect("Couldn't get file path");
                println!("Dropped file: {:?}", path);
                // You can process the file here
                sender::move_file(path.to_str().unwrap().to_string());
                // println!("{}", path.to_str().unwrap());
                sender::send_file().unwrap();
            }
        }
        // Return true to indicate the drop was handled
        true
    });

    vbox.add_controller(drop_target);

    window.set_child(Some(&vbox));

    window.present();
}

fn main() -> glib::ExitCode {
    println!("Hello, world!");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}