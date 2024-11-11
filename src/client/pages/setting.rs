use gtk::prelude::*;
use gtk::{Box, Stack, Orientation, Button};

pub struct SettingPage {
    pub vbox_settings: Box,
    pub container: Box,
}

impl SettingPage {
    pub fn new(page_stack: &Stack) -> Self {
        let vbox_settings = Box::new(Orientation::Vertical, 0);
        let container = Box::new(Orientation::Horizontal, 0);


        let button_home = Button::new();
        button_home.add_css_class("custom-button");
        button_home.set_size_request(50, 50);
        button_home.set_hexpand(false);
        button_home.set_vexpand(false);

        let stack_clone = page_stack.clone();
        button_home.connect_clicked(move |_| {
            stack_clone.set_visible_child_name("home-page");
        });
        button_home.add_css_class("custom-button");
        vbox_settings.append(&button_home);

        Self { vbox_settings, container }
    }
}