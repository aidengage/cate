mod pages;
mod sender;
use gdk::Display;
use gtk::prelude::*;
use gtk::{
    gdk, Application, ApplicationWindow, Box, CssProvider, Orientation, Stack, StackSwitcher,
};
use lazy_static::lazy_static;
use pages::{file::FilePage, home::HomePage, setting::SettingPage};
use std::fs::File;
use std::net::Ipv4Addr;
use std::path::Path;
use std::string::ToString;
use std::sync::Arc;
use std::sync::Mutex;
use std::{env, fs, io};

lazy_static! {
    static ref ROOT_DIR: String = env::var("PROJECT_ROOT").unwrap_or_else(|_| env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string());
    static ref PULL_DIR: String = Path::new(&*ROOT_DIR)
        .join("pull/")
        .to_str()
        .unwrap()
        .to_string();
    static ref PUSH_DIR: String = Path::new(&*ROOT_DIR)
        .join("push/")
        .to_str()
        .unwrap()
        .to_string();
    static ref LINK_DIR: String = Path::new(&*ROOT_DIR)
        .join("assets/")
        .to_str()
        .unwrap()
        .to_string();
    static ref LINK_FILE: String = Path::new(&*ROOT_DIR)
        .join("assets/links.txt")
        .to_str()
        .unwrap()
        .to_string();
    static ref USER_DOMAIN: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref USER_IP: Arc<Mutex<Ipv4Addr>> = Arc::new(Mutex::new(Ipv4Addr::new(0, 0, 0, 0)));
}

const PORT: u16 = 8000;
const APP_ID: &str = "com.aidengage.carbon";

////////////////////////////////////////////
//         some weird file stuff          //
////////////////////////////////////////////

pub struct Carbon {
    pub window: ApplicationWindow,
    pub page_stack: Stack,
}

impl Carbon {
    pub fn new(app: &Application) -> Self {
        // Create the main window
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Carbon"));
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
        self.page_stack
            .add_named(&setting.vbox_settings, Some("setting-page"));
        self.page_stack.add_named(&home.overlay, Some("home-page"));
        self.page_stack
            .add_named(&file.vbox_files, Some("file-page"));
    }
}

fn main() {
    create_files_dirs().expect("could not create directories and files");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| apply_css());
    app.connect_activate(create_ui);
    app.run();
}

fn create_files_dirs() -> io::Result<()> {
    match fs::create_dir(&*PULL_DIR) {
        Ok(_) => println!("pull dir created"),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            println!("dir already exists");
        }
        Err(e) => return Err(e),
    }
    match fs::create_dir_all(&*PUSH_DIR) {
        Ok(_) => println!("push dir created"),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            println!("dir already exists");
        }
        Err(e) => return Err(e),
    }
    match fs::create_dir(&*LINK_DIR) {
        Ok(_) => {
            println!("link dir created");
            File::create(LINK_FILE.clone())?;
        }
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            println!("dir already exists");
            // File::create(LINK_FILE.clone())?;
        }
        Err(e) => return Err(e),
    }
    Ok(())
}

fn create_ui(app: &Application) {
    // Create and initialize the app
    let app = Carbon::new(app);
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
