use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{gdk, DropTarget, Label, Stack, Button, Box, Orientation, CssProvider, Align};

use crate::sender;

pub struct HomePage {
    pub vbox_home: Box,
    pub container: Box,
    // pub page_stack: Stack,
}

impl HomePage {
    pub fn new(page_stack: &Stack) -> Self {

        let vbox_home = Box::new(Orientation::Vertical, 10);
        // let nav_bar = Self::generate_nav();


        let container = Box::new(Orientation::Vertical, 0);
        let nav_bar = Box::new(Orientation::Horizontal, 0);

        let button_settings = Button::new();
        let button_files = Button::new();

        let settings_icon = gtk::Image::from_file("assets/settings.png");
        settings_icon.add_css_class("nav-icon");
        button_settings.set_child(Some(&settings_icon));

        let files_icon = gtk::Image::from_file("assets/files.png");
        files_icon.add_css_class("nav-icon");
        button_files.set_child(Some(&files_icon));

        button_settings.add_css_class("custom-button");
        button_files.add_css_class("custom-button");

        // container.set_width_request(100);
        // container.set_halign(Align::Center);
        button_settings.set_size_request(60, 60);
        button_settings.set_hexpand(false); // Disable horizontal expansion
        button_settings.set_vexpand(false); // Disable vertical expansion

        button_settings.set_halign(Align::Start); // or Center, End depending on where you want it
        button_settings.set_valign(Align::Center);

        button_files.set_size_request(60, 60);
        button_files.set_hexpand(false); // Disable horizontal expansion
        button_files.set_vexpand(false); // Disable vertical expansion

        button_files.set_halign(Align::End); // or Center, End depending on where you want it
        button_files.set_valign(Align::Center);

        // nav_bar.set_width_request(180);
        // nav_bar.set_halign(Align::Center);
        nav_bar.set_halign(Align::Start);

        let button_text = Label::builder()
            .label("S")
            // .margin_top(10)
            // .margin_bottom(10)
            // .margin_start(10)
            // .margin_end(10)
            .build();

        button_text.add_css_class("button-text");


        // vbox_home.append(&button_to_page2);


        container.append(&button_text);
        // button_to_page2.set_child(Some(&container));
        nav_bar.append(&button_settings);
        nav_bar.append(&button_files);

        let stack_clone = page_stack.clone();
        button_settings.connect_clicked(move |_| {
            stack_clone.set_visible_child_name("file-page");
        });



        let fixed = gtk::Fixed::new();
        let grid = gtk::Grid::new();


        let provider = CssProvider::new();

        // button_to_page2.set_hexpand(true);



        // button_to_page2.set_size_request(100, 50);
        // fixed.set_size_request(100, 100);

        // fixed.put(&button_to_page2, 0f64, 0f64);
        // grid.attach(&button_to_page2, 0, 0, 1, 1);

        let label_page2 = Label::builder()
            .label("CARBON")
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();


        vbox_home.append(&nav_bar);
        vbox_home.append(&label_page2);



        let drop = DropTarget::new(gdk::FileList::static_type(), gdk::DragAction::COPY);

        drop.connect_drop(move |_, value, _, _| {
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

        vbox_home.add_controller(drop);

        Self { vbox_home, container }
    }

    fn generate_nav() -> gtk::Box {
        let container = Box::new(Orientation::Vertical, 0);
        let nav_bar = Box::new(Orientation::Horizontal, 0);

        let button_to_page2 = Button::new();
        let button_right = Button::new();

        button_to_page2.add_css_class("custom-button");
        button_right.add_css_class("custom-button");

        // container.set_width_request(100);
        // container.set_halign(Align::Center);
        button_to_page2.set_size_request(10, 10);
        button_to_page2.set_hexpand(false); // Disable horizontal expansion
        button_to_page2.set_vexpand(false); // Disable vertical expansion

        button_to_page2.set_halign(gtk::Align::Start); // or Center, End depending on where you want it
        button_to_page2.set_valign(gtk::Align::Center);

        button_right.set_size_request(10, 10);
        button_right.set_hexpand(false); // Disable horizontal expansion
        button_right.set_vexpand(false); // Disable vertical expansion

        button_right.set_halign(gtk::Align::End); // or Center, End depending on where you want it
        button_right.set_valign(gtk::Align::Center);

        // nav_bar.set_width_request(180);
        nav_bar.set_halign(Align::Center);

        let button_text = Label::builder()
            .label("S")
            // .margin_top(10)
            // .margin_bottom(10)
            // .margin_start(10)
            // .margin_end(10)
            .build();

        button_text.add_css_class("button-text");


        // vbox_home.append(&button_to_page2);


        container.append(&button_text);
        // button_to_page2.set_child(Some(&container));
        nav_bar.append(&button_to_page2);
        nav_bar.append(&button_right);

        // let stack_clone = self::page_stack.clone();
        // button_to_page2.connect_clicked(move |_| {
        //     stack_clone.set_visible_child_name("file-page");
        // });

        nav_bar
    }
}