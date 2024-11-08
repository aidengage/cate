use gtk::prelude::*;
use gtk::{gdk, glib, Application, ApplicationWindow, DropTarget, Label, CssProvider, Stack, Button, Box, Align, Orientation, Entry};

pub struct FilePage {
    pub vbox_files: Box,
    pub container: Box,
    // entry: Entry,
}

impl FilePage {
    pub fn new(page_stack: &Stack) -> Self {

        let vbox_files = Box::new(Orientation::Vertical, 10);
        let container = Box::new(Orientation::Vertical, 0);

        let button_back_home = Button::new();
        button_back_home.add_css_class("custom-button");

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

        vbox_files.append(&label_page2);
        vbox_files.append(&button_back_home);

        container.append(&button_text_page2);
        button_back_home.set_child(Some(&container));

        let stack_clone = page_stack.clone();
        button_back_home.connect_clicked(move |_| {
            stack_clone.set_visible_child_name("home-page")
        });








        // Create main container for the page


        // Add your page content
        // let label = Label::new(Some("Welcome to Page Two"));
        // let entry = Entry::new();
        // let button = Button::with_label("Submit");

        // Connect signals
        // let entry_clone = entry.clone();
        // button.connect_clicked(move |_| {
        //     println!("Submitted text: {}", entry_clone.text());
        // });

        // Add widgets to the page
        // vbox_files.append(&label);
        // vbox_files.append(&entry);
        // vbox_files.append(&button);

        Self { vbox_files, container }
    }

    // Add page-specific methods here
    // pub fn get_entry_text(&self) -> String {
    //     self.entry.text().to_string()
    // }
}