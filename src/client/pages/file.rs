use gtk::prelude::*;
use gtk::{Label, Stack, Button, Box, Orientation, ListBox, ScrolledWindow, PolicyType, gdk, GestureClick, glib};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use glib::ControlFlow;
use crate::LINK_FILE;

pub struct FilePage {
    pub vbox_files: Box,
    pub list_box: ListBox,
}

impl FilePage {
    pub fn new(page_stack: &Stack) -> Self {
        let vbox_files = Box::new(Orientation::Vertical, 0);
        let container = Box::new(Orientation::Vertical, 0);
        let list_box = ListBox::new();

        let scrollable_window = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .min_content_width(300)
            .min_content_height(100)
            .child(&list_box)
            .build();

        let button_home = Button::new();
        button_home.add_css_class("custom-button");

        let button_text_page2 = Label::builder()
            .label("back to home page")
            .build();
        button_text_page2.add_css_class("button-text");

        let label_page2 = Label::builder()
            .label("File(s)")
            .margin_top(5)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        vbox_files.append(&label_page2);
        vbox_files.append(&scrollable_window);
        vbox_files.append(&button_home);

        container.append(&button_text_page2);
        button_home.set_child(Some(&container));

        let stack_clone = page_stack.clone();
        button_home.connect_clicked(move |_| {
            stack_clone.set_visible_child_name("home-page")
        });
        button_home.add_css_class("custom-button");

        let file_page = Self { vbox_files, list_box };
        file_page.setup_auto_refresh();
        file_page
    }

    fn setup_auto_refresh(&self) {
        let list_box = self.list_box.clone();
        let file_path = &*LINK_FILE;

        // Refresh every 1000ms (1 second) - adjust this value as needed
        glib::timeout_add_local(Duration::from_millis(1000), move || {
            if let Ok(file) = File::open(&file_path) {
                let reader = BufReader::new(file);
                FilePage::populate_list_box(&list_box, reader);
            }
            // Continue the timeout
            ControlFlow::Continue
        });

    }

    // Helper function to create a label with clipboard functionality
    fn create_link_label(text: &str) -> Label {
        let label = Label::new(Some(text));
        label.set_selectable(false);
        label.set_can_target(true);
        label.set_widget_name("clickable-label");

        let clipboard = gdk::Display::default()
            .expect("could not get default display")
            .clipboard();

        let gesture = GestureClick::new();
        let label_clone = label.clone();
        gesture.connect_released(move |_, _, _, _| {
            clipboard.set_text(label_clone.text().as_str());
        });
        label.add_controller(gesture);
        label.add_css_class("button-text");

        label
    }

    // Helper function to populate the list box
    fn populate_list_box(list_box: &ListBox, reader: BufReader<File>) {
        // Clear existing items
        while let Some(child) = list_box.first_child() {
            list_box.remove(&child);
        }

        // Add new items
        for line in reader.lines() {
            if let Ok(text) = line {
                let label = FilePage::create_link_label(&text);
                list_box.append(&label);
            }
        }
    }
}