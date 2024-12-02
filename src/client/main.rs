mod sender;
mod pages;

use std::cell::RefCell;
use pages::{home::HomePage, file::FilePage, setting::SettingPage};

use std::net::Ipv4Addr;

use gdk::Display;
use gtk::prelude::*;
use gtk::{gdk, Application, ApplicationWindow, CssProvider, Stack, Box, StackSwitcher, Orientation};

use std::env;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::string::ToString;
use lazy_static::lazy_static;

use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref ROOT_DIR: String = env::var("PROJECT_ROOT").unwrap_or_else(|_| env::current_dir().unwrap().to_str().unwrap().to_string());
    static ref PULL_DIR: String = Path::new(&*ROOT_DIR).join("pull/").to_str().unwrap().to_string();
    static ref PUSH_DIR: String = Path::new(&*ROOT_DIR).join("push/").to_str().unwrap().to_string();
    static ref LINK_FILE: String = Path::new(&*ROOT_DIR).join("assets/links.txt").to_str().unwrap().to_string();
    // static ref USER_DOMAIN: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
    static ref USER_DOMAIN: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}

// const ROOT_DIR: String = env::var("PROJECT_ROOT").unwrap_or_else(|_| env::current_dir().unwrap().to_str().unwrap().to_string());
// const PULL_DIR: &str = Path::new(&ROOT_DIR).join("pull").to_str().unwrap();
// const PUSH_DIR: &str = Path::new(&ROOT_DIR).join("push").to_str().unwrap();

// const PULL_DIR: &str = "/Users/aidengage/dev/senior/cate/pull/";
// const PULL_DIR: String = ROOT_DIR + "pull";

// const PUSH_DIR: &str = "/Users/aidengage/dev/senior/cate/push/";
// const PUSH_DIR: String = ROOT_DIR + "push";

// const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
// const ADDR: Ipv4Addr = Ipv4Addr::new(172, 17, 0, 2);
const ADDR: Ipv4Addr = Ipv4Addr::new(74,130,78,72);

// docker localhost address, dont need to specify here, keep 127, 0, 0, 1 when sending to local docker
const PORT: u16 = 8000;
const APP_ID: &str = "com.aidengage.carbon";



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
        // window.set_default_size(800, 600);
        window.set_default_size(350, 200);

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
        let setting = SettingPage::new(&self.page_stack);

        // Add pages to stack
        // self.page_stack.add_named(&home.vbox_home, Some("home-page"));
        self.page_stack.add_named(&home.overlay, Some("home-page"));
        self.page_stack.add_named(&file.vbox_files, Some("file-page"));
        self.page_stack.add_named(&setting.vbox_settings, Some("setting-page"));
    }
}

fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| apply_css());
    app.connect_activate(create_ui);
    app.run();
}

fn create_ui(app: &Application) {
    // Create and initialize the app
    let app = App::new(app);
    app.init();
    app.window.present();
}

fn apply_css() {

    let styling = CssProvider::new();
    styling.load_from_string(include_str!("./client.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Couldn't get default display"),
        &styling,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}