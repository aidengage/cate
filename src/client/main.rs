mod sender;

use std::net::Ipv4Addr;
use std::path::PathBuf;

use gdk::Display;
use gtk::prelude::*;
use gtk::{gdk, glib, Application, ApplicationWindow, DropTarget, Label, CssProvider, Stack, Button, Box};
use gtk::glib::clone;

const PULL_DIR: &str = "/Users/aidengage/dev/senior/cate/pull/";
const DISCARD: &str = "/Users/aidengage/dev/senior/cate/push/";
// const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const ADDR: Ipv4Addr = Ipv4Addr::new(74,130,78,72);
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

    let page_stack = Stack::new();

    let vbox_home = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let label = Label::builder()
        .label("Drop Files Here")
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    vbox_home.append(&label);

    // i dont know what the fuck i am doing this worked because clone :)
    // had a borrow issue??
    let page_copy = page_stack.clone();
    let button_to_page2 = Button::with_label("page 2");
    // button_to_page2.connect_clicked(clone!(@weak app => move |_| {
    //     page_copy.set_visible_child_name("page2");
    // }));
    button_to_page2.connect_clicked(move |_| {page_copy.set_visible_child_name("page2")});
    vbox_home.append(&button_to_page2);

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

    vbox_home.add_controller(drop_target);

    page_stack.add_named(&vbox_home, Option::from("home"));

    let vbox_page2 = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let label_page2 = Label::builder()
        .label("another page")
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();
    vbox_page2.append(&label_page2);

    let button_back_home = Button::with_label("Back to Page 1");
    button_back_home.connect_clicked(clone!(@weak page_stack => move |_| {
        page_stack.set_visible_child_name("home");
    }));

    vbox_page2.append(&button_back_home);

    page_stack.add_named(&vbox_page2, Option::from("page2"));

    window.set_child(Some(&page_stack));
    // window.set_child(Some(&vbox_home));

    window.present();
}

fn load_css() {
    let styling = CssProvider::new();
    // styling.load_from_path("./client.css");
    styling.load_from_string(include_str!("./client.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Couldn't get default display"),
        &styling,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() -> glib::ExitCode {
    println!("Hello, world!");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}