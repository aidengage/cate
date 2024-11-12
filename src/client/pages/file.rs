use gtk::prelude::*;
use gtk::{Label, Stack, Button, Box, Orientation, ListBox, ScrolledWindow, PolicyType, gdk, GestureClick};
use gdk::Clipboard;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr::copy;

pub struct FilePage {
    pub vbox_files: Box,
    pub container: Box,
    pub list_box: ListBox,
}

impl FilePage {
    pub fn new(page_stack: &Stack) -> Self {

        let vbox_files = Box::new(Orientation::Vertical, 0);
        let container = Box::new(Orientation::Vertical, 0);



        let list_box = ListBox::new();
        let file = File::open("/Users/aidengage/dev/senior/cate/assets/links.txt").unwrap();
        let reader = BufReader::new(file);
        // let copy_button = Button::new();

        for lines in reader.lines() {
            // copy_button.set_tooltip_text(Some("Copy file"));

            // let copy_container = Box::new(Orientation::Vertical, 2);
            let label = Label::new(Some(lines.unwrap().as_ref()));
            // label.set_use_markup(true);
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

            // label.connect_activate_link(move |label, _uri| {
            //     let text = label.text();
            //     clipboard.set_text(&text);
            //     println!("text: {}", text);
            //     gtk::glib::Propagation::Stop
            // });


            // copy_button.set_tooltip_text(Some(&label.));
            // copy_button.add_css_class("custom-button");

            // copy_container.append(&label);

            label.add_css_class("button-text");
            // list_box.append(&label);
            // list_box.append(&copy_to_clipboard);
            // list_box.append(&copy_button);
            // copy_button.set_child(Some(&copy_container));
            list_box.append(&label);
        }
        // copy_button.connect_
        // copy_button.connect_clicked(move |_| {
        //     Clipboard::
        // })

        let scrollable_window = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .min_content_width(300)
            .child(&list_box)
            .build();






        let button_back_home = Button::new();

        button_back_home.add_css_class("custom-button");

        let button_text_page2 = Label::builder()
            .label("back to home page")
            .build();

        button_text_page2.add_css_class("button-text");

        let label_page2 = Label::builder()
            .label("File(s)")
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        vbox_files.append(&label_page2);
        vbox_files.append(&scrollable_window);
        vbox_files.append(&button_back_home);

        container.append(&button_text_page2);
        button_back_home.set_child(Some(&container));

        let stack_clone = page_stack.clone();
        button_back_home.connect_clicked(move |_| {
            stack_clone.set_visible_child_name("home-page")
        });
        button_back_home.add_css_class("custom-button");

        Self { vbox_files, container, list_box }
    }

    // Add page-specific methods here
    // pub fn get_entry_text(&self) -> String {
    //     self.entry.text().to_string()
    // }

    pub fn process_links(&self) {

    }
}