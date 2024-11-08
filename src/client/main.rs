mod sender;
mod pages;
use pages::{home::HomePage, file::FilePage};

use std::net::Ipv4Addr;

use gdk::Display;
use gtk::prelude::*;
use gtk::{gdk, Application, ApplicationWindow, CssProvider, Stack, Box, StackSwitcher, Orientation};

const PULL_DIR: &str = "/Users/aidengage/dev/senior/cate/pull/";
const DISCARD: &str = "/Users/aidengage/dev/senior/cate/push/";
// const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const ADDR: Ipv4Addr = Ipv4Addr::new(74,130,78,72);
// const ADDR: Ipv4Addr = Ipv4Addr::new(192,168,1,104);
const PORT: u16 = 8000;
const APP_ID: &str = "carbon";



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
        self.page_stack.add_named(&home.vbox_home, Some("home-page"));
        self.page_stack.add_named(&file.vbox_files, Some("file-page"));
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

fn load_css() {

    let styling = CssProvider::new();
    styling.load_from_string(include_str!("./client.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Couldn't get default display"),
        &styling,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}