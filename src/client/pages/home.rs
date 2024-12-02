use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{gdk, DropTarget, Label, Stack, Button, Box, Orientation, CssProvider, Align, Justification, Overlay};
use crate::pages::file::FilePage;
use crate::{sender, USER_DOMAIN};

pub struct HomePage {
    pub vbox_home: Box,
    pub container: Box,
    pub overlay: Overlay,
    // pub page_stack: Stack,
}

impl HomePage {
    pub fn new(page_stack: &Stack) -> Self {

        let vbox_home = Box::new(Orientation::Vertical, 10);
        let background = Box::new(Orientation::Horizontal, 10);
        // let nav_bar = Self::generate_nav();


        let container = Box::new(Orientation::Vertical, 0);
        let nav_bar = Box::new(Orientation::Horizontal, 0);

        let button_settings = Button::new();
        let button_files = Button::new();
        let button_domain = Button::builder()
            .label("Domain")
            .build();

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
        button_settings.set_size_request(50, 50);
        button_settings.set_hexpand(false); // Disable horizontal expansion
        button_settings.set_vexpand(false); // Disable vertical expansion

        button_settings.set_halign(Align::Start); // or Center, End depending on where you want it
        button_settings.set_valign(Align::Center);

        button_files.set_size_request(50, 50);
        button_files.set_hexpand(false); // Disable horizontal expansion
        button_files.set_vexpand(false); // Disable vertical expansion

        button_files.set_halign(Align::Start); // or Center, End depending on where you want it
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

        let stack_setting = page_stack.clone();
        button_settings.connect_clicked(move |_| {
            stack_setting.set_visible_child_name("setting-page");
        });

        let stack_file = page_stack.clone();
        button_files.connect_clicked(move |_| {
            stack_file.set_visible_child_name("file-page");
        });

        let label = Label::builder()
            .label("CARBON")
            .justify(Justification::Center)
            // .halign(Align::Center)
            // .valign(Align::Center)
            // .margin_top(10)
            // .margin_bottom(10)
            // .margin_start(10)
            // .margin_end(10)
            .build();
        background.set_halign(Align::Center);
        background.set_valign(Align::Center);
        background.set_vexpand(true);
        background.set_hexpand(true);

        vbox_home.append(&nav_bar);
        background.append(&label);
        // vbox_home.append(&background);

        let overlay = Overlay::new();
        overlay.set_child(Some(&background));
        overlay.add_overlay(&vbox_home);


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

        // debug button to print domain name from setting page
        button_domain.connect_clicked(move |_| {
            let mut domain = USER_DOMAIN.lock().unwrap();
            // println!("domain: {:?}", *USER_DOMAIN);
            println!("domain: {}", *domain);
        });
        button_domain.add_css_class("custom-button");

        vbox_home.add_controller(drop);
        vbox_home.append(&button_domain);

        Self { vbox_home, container, overlay }
    }

    fn generate_nav() -> gtk::Box {
        let container = Box::new(Orientation::Vertical, 0);
        let nav_bar = Box::new(Orientation::Horizontal, 0);

        let button_settings = Button::new();
        let button_files = Button::new();

        button_settings.add_css_class("custom-button");
        button_files.add_css_class("custom-button");

        // container.set_width_request(100);
        // container.set_halign(Align::Center);
        button_settings.set_size_request(10, 10);
        button_settings.set_hexpand(false); // Disable horizontal expansion
        button_settings.set_vexpand(false); // Disable vertical expansion

        button_settings.set_halign(Align::Start); // or Center, End depending on where you want it
        button_settings.set_valign(Align::Center);

        button_files.set_size_request(10, 10);
        button_files.set_hexpand(false); // Disable horizontal expansion
        button_files.set_vexpand(false); // Disable vertical expansion

        button_files.set_halign(Align::End); // or Center, End depending on where you want it
        button_files.set_valign(Align::Center);

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
        nav_bar.append(&button_settings);
        nav_bar.append(&button_files);

        nav_bar
    }
}