use gtk::prelude::*;
use gtk::{Box, Stack, Orientation, Button, Label, Entry, EntryBuffer};
use gtk::glib::clone;
use std::rc::Rc;
use std::cell::RefCell;

pub struct SettingPage {
    pub vbox_settings: Box,
    pub container: Box,
    pub domain: Rc<RefCell<String>>,
}

impl SettingPage {
    pub fn new(page_stack: &Stack) -> Self {
        let vbox_settings = Box::new(Orientation::Vertical, 0);
        let container = Box::new(Orientation::Horizontal, 0);
        // let mut domain = String::new();
        let mut domain = Rc::new(RefCell::new(String::new()));

        let button_home = Button::new();
        button_home.add_css_class("custom-button");
        button_home.set_size_request(50, 50);
        button_home.set_hexpand(false);
        button_home.set_vexpand(false);

        let button_text = Label::builder()
            .label("Home")
            .build();
        button_text.add_css_class("button-text");

        let label_setting = Label::builder()
            .label("Setting(s)")
            .margin_top(5)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        let entry_buffer = EntryBuffer::builder().build();
        let entry_box = Entry::builder()
            .buffer(&entry_buffer)
            .placeholder_text("Enter Your Domain")
            .build();

        // entry_buffer.connect_notify()
        let save_button = Button::builder()
            .label("Save")
            .build();

        // let save_label = Label::builder()
        //     .label("Save")
        //     .build();
        // save_label.add_css_class("button-text");

        save_button.add_css_class("custom-button");

        let entry_copy = entry_box.clone();
        let domain_clone = domain.clone();
        save_button.connect_clicked(move |_| {
            // domain = entry_copy.buffer().text().parse().unwrap();
            *domain_clone.borrow_mut() = entry_copy.buffer().text().to_string();
            println!("entry box: {}", domain_clone.borrow());
        });
        vbox_settings.append(&label_setting);
        vbox_settings.append(&entry_box);
        vbox_settings.append(&save_button);
        vbox_settings.append(&button_home);

        // container.append(&save_label);
        // save_button.set_child(Some(&container));

        container.append(&button_text);
        button_home.set_child(Some(&container));

        let stack_clone = page_stack.clone();
        button_home.connect_clicked(move |_| {
            stack_clone.set_visible_child_name("home-page");
        });
        button_home.add_css_class("custom-button");


        Self { vbox_settings, container, domain }
    }
}