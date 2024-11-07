mod sender;

use std::net::Ipv4Addr;
use std::path::PathBuf;

use gdk::Display;
use gtk::prelude::*;
use gtk::{gdk, glib, Application, ApplicationWindow, DropTarget, Label, CssProvider, Stack, Button, Box, Align};

const PULL_DIR: &str = "/Users/aidengage/dev/senior/cate/pull/";
const DISCARD: &str = "/Users/aidengage/dev/senior/cate/push/";
// const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const ADDR: Ipv4Addr = Ipv4Addr::new(74,130,78,72);
// const ADDR: Ipv4Addr = Ipv4Addr::new(192,168,1,104);
const PORT: u16 = 8000;
const APP_ID: &str = "cate";

fn build_ui(app: &Application) {
    // creating stacks
    let page_stack = Stack::new();
    let home_buttons = page_stack.clone();
    let page2_buttons = page_stack.clone();

    // create buttons
    let button_to_page2 = Button::new();
    let button_back_home = Button::with_label("back to page 1");

    // create vbox pages
    let vbox_home = Box::new(gtk::Orientation::Vertical, 0);
    let vbox_page2 = Box::new(gtk::Orientation::Vertical, 0);
    let button_container = Box::new(gtk::Orientation::Vertical, 0);
    let p2_button_container = Box::new(gtk::Orientation::Vertical, 0);

    // create drag and drop target
    let drop_target = DropTarget::new(gdk::FileList::static_type(), gdk::DragAction::COPY);

    // building everything
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Cate")
        .default_width(320)
        .default_height(180)
        .build();

    let label = Label::builder()
        .label("Drop Files Here")
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    // label.add_css_class("button-text");

    let button_text = Label::builder()
        .label("to page 2")
        .build();

    button_text.add_css_class("button-text");

    let button_text_page2 = Label::builder()
        .label("back to home page")
        .build();

    button_text_page2.add_css_class("button-text");

    let label_page2 = Label::builder()
        .label("another page")
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    drop_target.connect_drop(move |_, value, _, _| {
        // println!("value: {:?}", value);
        if let Ok(file_list) = value.get::<gdk::FileList>() {
            for file in file_list.files() {
                let path: PathBuf = file.path().expect("Couldn't get file path");
                println!("Dropped file: {:?}", path);
                sender::move_file(path.to_str().unwrap().to_string());
                // println!("{}", path.to_str().unwrap());
                sender::send_file().unwrap();
            }
        }
        // Return true to indicate the drop was handled
        true
    });
    vbox_home.append(&label);
    vbox_home.append(&button_to_page2);
    vbox_home.add_controller(drop_target);
    // vbox_home.append(&button_text);


    button_container.append(&button_text);
    button_to_page2.set_child(Some(&button_container));

    p2_button_container.append(&button_text_page2);
    button_back_home.set_child(Some(&p2_button_container));
    // button_back_home.append(&vbox_page2);
    // button_back_home.append(&button_back_home)?;

    // textbox.append(&button_text);

    // button_text.add_css_class("button-text");
    // textbox.add_css_class("button-text");
    // button_to_page2.append(&button_text).expect("could not add label");

    button_to_page2.connect_clicked(move |_| {home_buttons.set_visible_child_name("page2")});
    button_to_page2.add_css_class("custom-button");

    vbox_page2.append(&label_page2);

    button_back_home.connect_clicked(move |_| {page2_buttons.set_visible_child_name("home")});
    button_back_home.add_css_class("custom-button");

    vbox_page2.append(&button_back_home);

    page_stack.add_named(&vbox_home, Option::from("home"));
    page_stack.add_named(&vbox_page2, Option::from("page2"));

    window.set_child(Some(&page_stack));
    // window.set_child(Some(&button_to_page2));

    window.present();
}

fn load_css() {

    let styling = CssProvider::new();
    styling.load_from_string(include_str!("./client.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Couldn't get default display"),
        &styling,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        // gtk::STYLE_PROVIDER_PRIORITY_USER,
    );
}

fn main() -> glib::ExitCode {
    println!("Hello, world!");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}