use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{gdk, DropTarget, Label, Stack, Button, Box, Orientation};

use crate::sender;

pub struct HomePage {
    pub vbox_home: Box,
    pub container: Box,
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
}