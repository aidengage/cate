use std::net::Ipv4Addr;
use gtk::prelude::*;
use gtk::{Box, Stack, Orientation, Button, Label, Entry, EntryBuffer};

use crate::{USER_DOMAIN, USER_IP};

pub struct SettingPage {
    pub vbox_settings: Box,
}

impl SettingPage {
    pub fn new(page_stack: &Stack) -> Self {
        let vbox_settings = Box::new(Orientation::Vertical, 0);
        let save_container = Box::new(Orientation::Vertical, 0);

        let button_home = Button::new();
        button_home.add_css_class("custom-button");

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

        let domain_buffer = EntryBuffer::builder().build();
        let domain_entry = Entry::builder()
            .buffer(&domain_buffer)
            .placeholder_text("Enter Your Domain")
            .build();

        let ip_buffer = EntryBuffer::builder().build();
        let ip_entry = Entry::builder()
            .buffer(&ip_buffer)
            .placeholder_text("Enter Your IP")
            .build();

        let save_button = Button::builder()
            .build();

        let save_label = Label::builder()
            .label("Save")
            .build();
        save_label.add_css_class("button-text");

        save_button.add_css_class("custom-button");
        button_home.add_css_class("custom-button");

        let domain_entry_copy = domain_entry.clone();
        let ip_entry_copy = ip_entry.clone();

        let stack_clone = page_stack.clone();
        save_button.connect_clicked(move |_| {
            let mut domain = USER_DOMAIN.lock().unwrap();
            let mut ip = USER_IP.lock().unwrap();

            if let Ok(parsed_ip) = ip_entry_copy.buffer().text().parse::<Ipv4Addr>() {
                if parsed_ip != Ipv4Addr::new(0, 0, 0, 0) {
                    *ip = ip_entry_copy.buffer().text().parse().unwrap();
                }
            }

            *domain = domain_entry_copy.buffer().text().parse().unwrap();

            // println!("USER_DOMAIN: {:?}", *USER_DOMAIN);
            println!("domain: {:?}", domain);
            println!("ip: {:?}", ip);

            stack_clone.set_visible_child_name("home-page");
        });
        vbox_settings.append(&label_setting);
        vbox_settings.append(&domain_entry);
        vbox_settings.append(&ip_entry);
        vbox_settings.append(&save_button);
        save_container.append(&save_label);
        save_button.set_child(Some(&save_container));

        Self { vbox_settings }
    }
}