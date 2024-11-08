mod sender;
mod pages;
use pages::{home::HomePage, file::FilePage};

use std::net::Ipv4Addr;
use std::path::PathBuf;

use gdk::Display;
use gtk::prelude::*;
use gtk::{gdk, glib, Application, ApplicationWindow, DropTarget, Label, CssProvider, Stack, Button, Box, Align, StackSwitcher, Orientation, StackTransitionType};

const PULL_DIR: &str = "/Users/aidengage/dev/senior/cate/pull/";
const DISCARD: &str = "/Users/aidengage/dev/senior/cate/push/";
// const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const ADDR: Ipv4Addr = Ipv4Addr::new(74,130,78,72);
// const ADDR: Ipv4Addr = Ipv4Addr::new(192,168,1,104);
const PORT: u16 = 8000;
const APP_ID: &str = "carbon";

// fn build_ui2(app: &Application) {
//     let vbox_home = Box::new(gtk::Orientation::Vertical, 0);
//
//     let window = ApplicationWindow::builder()
//         .application(app)
//         .title("Carbon")
//         .default_width(320)
//         .default_height(180)
//         .build();
//
//     let label = Label::builder()
//         .label("Carbon")
//         .margin_top(24)
//         .margin_bottom(24)
//         .margin_start(24)
//         .margin_end(24)
//         .build();
//
//     let button_text = Label::builder()
//         .label("to page 2")
//         .build();
//
//     button_text.add_css_class("button-text");
//
//     let drop_target = DropTarget::new(gdk::FileList::static_type(), gdk::DragAction::COPY);
//
//     let page_stack = Stack::new();
//
//     vbox_home.append(&label);
//     // vbox_home.append(&button_to_page2);
//     vbox_home.add_controller(drop_target);
//
//
//
//
//     let vbox_page2 = Box::new(gtk::Orientation::Vertical, 0);
//     let button_back_home = Button::with_label("back to page 1");
//
//     let button_text_page2 = Label::builder()
//         .label("back to home page")
//         .build();
//
//     button_text_page2.add_css_class("button-text");
//
//     let label_page2 = Label::builder()
//         .label("another page")
//         .margin_top(24)
//         .margin_bottom(24)
//         .margin_start(24)
//         .margin_end(24)
//         .build();
//
//
//
//
//
//
//
//
//     // creating stacks
//     let page_stack = Stack::new();
//     let home_buttons = page_stack.clone();
//     let page2_buttons = page_stack.clone();
//
//     // create buttons
//
//
//     // create vbox pages
//
//     let button_container = Box::new(gtk::Orientation::Vertical, 0);
//     let p2_button_container = Box::new(gtk::Orientation::Vertical, 0);
//
//     // create drag and drop target
//
//
//     // building everything
//
//
//
//
//     // label.add_css_class("button-text");
//
//
//
//
//
//
//     // vbox_home.append(&button_text);
//
//
//     button_container.append(&button_text);
//     button_to_page2.set_child(Some(&button_container));
//
//     p2_button_container.append(&button_text_page2);
//     button_back_home.set_child(Some(&p2_button_container));
//     // button_back_home.append(&vbox_page2);
//     // button_back_home.append(&button_back_home)?;
//
//     // textbox.append(&button_text);
//
//     // button_text.add_css_class("button-text");
//     // textbox.add_css_class("button-text");
//     // button_to_page2.append(&button_text).expect("could not add label");
//
//     button_to_page2.connect_clicked(move |_| {home_buttons.set_visible_child_name("page2")});
//     button_to_page2.add_css_class("custom-button");
//
//     vbox_page2.append(&label_page2);
//     vbox_page2.append(&button_back_home);
//
//     button_back_home.connect_clicked(move |_| {page2_buttons.set_visible_child_name("home")});
//     button_back_home.add_css_class("custom-button");
//
//
//     page_stack.add_named(&vbox_home, Option::from("home"));
//     page_stack.add_named(&vbox_page2, Option::from("files"));
//
//     window.set_child(Some(&page_stack));
//     // window.set_child(Some(&button_to_page2));
//
//     window.present();
// }

fn load_css() {

    let styling = CssProvider::new();
    styling.load_from_string(include_str!("./client.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Couldn't get default display"),
        &styling,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        // gtk::STYLE_PROVIDER_PRIORITY_USER,
    );
}

fn main2() -> glib::ExitCode {
    println!("Hello, world!");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}


////////////////////////////////////////////
//         some weird file stuff          //
////////////////////////////////////////////

pub struct App {
    pub window: ApplicationWindow,
    pub page_stack: Stack,
}

impl App {
    pub fn new(app: &Application) -> Self {
        // Create the main window
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Carbon"));
        window.set_default_size(320, 180);

        // Create the stack for managing pages
        let page_stack = Stack::new();
        // stack.set_transition_type(StackTransitionType::SlideLeftRight);

        // Create a stack switcher (tabs/navigation)
        let stack_switcher = StackSwitcher::new();
        stack_switcher.set_stack(Some(&page_stack));

        // Create the main vertical box
        let main_box = Box::new(Orientation::Vertical, 10);
        main_box.append(&stack_switcher);
        main_box.append(&page_stack);

        // Set up the window
        window.set_child(Some(&main_box));

        Self { window, page_stack }
    }

    pub fn init(&self) {
        // Initialize pages
        let home = HomePage::new(&self.page_stack);
        let file = FilePage::new(&self.page_stack);


        // Add pages to stack
        // self.page_stack.add_titled(&home.vbox_home, Some("home-page"), "Home");
        // self.page_stack.add_titled(&file.vbox_files, Some("file-page"), "Files");
        // self.page_stack.add_child(&home.button_to_page2);
        // self.stack.set_visible_child_name("home");
        // self.stack.set_visible_child_name("file");
        // self.stack.set_child_visible(Some(&home));
        // self.stack.set_child_visible(Some(&file));

        self.page_stack.add_named(&home.vbox_home, Some("home-page"));
        self.page_stack.add_named(&file.vbox_files, Some("file-page"));

        // self.page_stack.add_named(&home.button_to_page2, Option::from("home"));
    }
}

fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    // Create and initialize the app
    let app = App::new(app);
    app.init();
    app.window.present();
}
