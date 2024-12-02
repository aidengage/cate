use gtk::prelude::*;
use gtk::{Box, Stack, Orientation, Button, Label, Entry, EntryBuffer};
use gtk::glib::clone;
use std::rc::Rc;
use std::cell::RefCell;

use std::sync::{Arc, MutexGuard};
use std::sync::Mutex;

use crate::{USER_DOMAIN};
// use crate::mod::domain;

pub struct SettingPage {
    pub vbox_settings: Box,
    pub container: Box,
    pub save_container: Box,
    // pub domain: Rc<RefCell<String>>,
    // pub domain: Arc<Mutex<String>>,
    // pub domain: String,
}

impl SettingPage {
    pub fn new(page_stack: &Stack) -> Self {
        let vbox_settings = Box::new(Orientation::Vertical, 0);
        let container = Box::new(Orientation::Vertical, 0);
        // let domain = USER_DOMAIN.lock().unwrap();
        // refcell allows for multiple accesses
        // let mut domain = Rc::new(RefCell::new(String::new()));
        // let mut domain = Rc::clone(&USER_DOMAIN);
        let save_container = Box::new(Orientation::Vertical, 0);

        let button_home = Button::new();
        button_home.add_css_class("custom-button");
        // button_home.set_size_request(10, 10);
        // button_home.set_hexpand(false);
        // button_home.set_vexpand(false);

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

        let save_button = Button::builder()
            .build();

        let save_label = Label::builder()
            .label("Save")
            .build();
        save_label.add_css_class("button-text");


        save_button.add_css_class("custom-button");
        button_home.add_css_class("custom-button");

        let entry_copy = entry_box.clone();
        // let mut domain = USER_DOMAIN.lock().unwrap();
        // let mut domain_clone = domain.clone();
        save_button.connect_clicked(move |_| {
            let mut domain = USER_DOMAIN.lock().unwrap();
            *domain = entry_copy.buffer().text().parse().unwrap();
            // if entry_copy.buffer().text().to_string() != "" {
            //     *domain_clone.borrow_mut() = entry_copy.buffer().text().to_string();
            // *USER_DOMAIN = Rc::new(RefCell::new(entry_copy.buffer().text().to_string()));
            // *USER_DOMAIN = Arc<Mutex>
            // domain.borrow_mut() = entry_copy.buffer().text().to_string();
            // domain = entry_copy.buffer().text().to_string();
            //     println!("entry box: {}", domain_clone.borrow());
            // *domain_clone = *entry_copy.buffer().text().to_string();
            println!("USER_DOMAIN: {:?}", *USER_DOMAIN);
            println!("domain: {:?}", domain);
                // println!("domain: {}", &domain)
            // } else {
            //     println!("enter your domain");
            // }
            // domain = entry_copy.buffer().text().parse().unwrap();
        });
        vbox_settings.append(&label_setting);
        vbox_settings.append(&entry_box);
        vbox_settings.append(&save_button);
        vbox_settings.append(&button_home);

        // container.append(&save_label);
        // save_button.set_child(Some(&container));

        save_container.append(&save_label);
        save_button.set_child(Some(&save_container));

        container.append(&button_text);
        button_home.set_child(Some(&container));

        let stack_clone = page_stack.clone();
        button_home.connect_clicked(move |_| {
            stack_clone.set_visible_child_name("home-page");
        });
        button_home.add_css_class("custom-button");


        Self { vbox_settings, container, save_container, /*domain*/ }
    }
}