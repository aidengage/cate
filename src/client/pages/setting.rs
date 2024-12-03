use gtk::prelude::*;
use gtk::{Box, Stack, Orientation, Button, Label, Entry, EntryBuffer};

use crate::{USER_DOMAIN};

pub struct SettingPage {
    pub vbox_settings: Box,
    pub container: Box,
    pub save_container: Box,
}

impl SettingPage {
    pub fn new(page_stack: &Stack) -> Self {
        let vbox_settings = Box::new(Orientation::Vertical, 0);
        let container = Box::new(Orientation::Vertical, 0);
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
        save_button.connect_clicked(move |_| {
            let mut domain = USER_DOMAIN.lock().unwrap();
            *domain = entry_copy.buffer().text().parse().unwrap();
            println!("USER_DOMAIN: {:?}", *USER_DOMAIN);
            println!("domain: {:?}", domain);
        });
        vbox_settings.append(&label_setting);
        vbox_settings.append(&entry_box);
        vbox_settings.append(&save_button);
        vbox_settings.append(&button_home);

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