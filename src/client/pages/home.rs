use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{gdk, glib, Application, ApplicationWindow, DropTarget, Label, CssProvider, Stack, Button, Box, Align, Orientation};

use crate::sender;

pub struct HomePage {
    pub vbox_home: Box,
    pub container: Box,
    // pub button_to_page2: Button,
}

impl HomePage {
    pub fn new(page_stack: &Stack) -> Self {

        let vbox_home = Box::new(Orientation::Vertical, 10);
        let container = Box::new(Orientation::Vertical, 0);

        let button_to_page2 = Button::new();
        button_to_page2.add_css_class("custom-button");

        let button_text = Label::builder()
            .label("button to page2")
            .build();

        button_text.add_css_class("button-text");

        let label_page2 = Label::builder()
            .label("carbon?")
            .margin_top(24)
            .margin_bottom(24)
            .margin_start(24)
            .margin_end(24)
            .build();

        vbox_home.append(&label_page2);
        vbox_home.append(&button_to_page2);

        container.append(&button_text);
        button_to_page2.set_child(Some(&container));

        let stack_clone = page_stack.clone();
        button_to_page2.connect_clicked(move |_| {
            stack_clone.set_visible_child_name("file-page");
        });






        // let container = Box::new(Orientation::Vertical, 0);
        // let vbox_home = Box::new(Orientation::Vertical, 0);
        // let button_to_page2 = Button::new();
        let drop = DropTarget::new(gdk::FileList::static_type(), gdk::DragAction::COPY);
        // // let page_stack = Stack::new();
        //
        // let label = Label::builder()
        //     .label("Carbon")
        //     .margin_top(24)
        //     .margin_bottom(24)
        //     .margin_start(24)
        //     .margin_end(24)
        //     .build();
        //
        // let button_text = Label::builder()
        //     .label("to page 2")
        //     .build();
        //
        // button_text.add_css_class("button-text");
        //
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


        //
        // vbox_home.append(&label);
        // vbox_home.append(&button_to_page2);
        // // vbox_home.add_controller(drop);
        //
        // container.append(&button_text);
        // button_to_page2.set_child(Some(&container));
        // // button_to_page2.connect_clicked(move |_| {home_buttons.set_visible_child_name("page2")});
        // button_to_page2.add_css_class("custom-button");
        // // container.append(&vbox_home);
        //
        // let vbox_home = Box::new(Orientation::Vertical, 10);
        // // vbox_home.set_margin_all(20);
        //
        // // let label = Label::new(Some("home page"));
        // // let button = Button::with_label("button");
        //
        // button_to_page2.connect_clicked(|_| {
        //     println!("button clicked!");
        // });

        // vbox_home.append(&label);
        // vbox_home.append(&button_to_page2);

        Self { vbox_home, container }
    }
}