/* User Interface */

use std::path::Path;
use std::option::Option;

use gtk::{Application, ApplicationWindow, Builder, PopoverMenu, Stack, 
    HeaderBar, AboutDialog, MenuButton, Picture, TextView};
use gtk::prelude::*;
use gio::{Menu, SimpleAction};
use gdk_pixbuf::Pixbuf;

use glib_macros::clone;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION"); // get package version from Cargo
const LICENSE: &str = env!("CARGO_PKG_LICENSE"); // get license of the project
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION"); // get the description of the project
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS"); // get the authors of the project
const COPYRIGHT_FORMAT: &str = "\u{00A9} 2023 "; // copyright format
const LOGO_PATH: &str = "src/resources/images/logo.png"; // path to the logo
const MENU_UI_PATH: &str = "src/resources/ui/menu.ui";

pub fn build_ui(application: &Application) {
    /* build ui of the application */
    // Create Window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Text Editor 3")
        .default_width(500)
        .default_height(500)
        .build();

    // Load menu ui file
    let menu_builder = Builder::from_file(MENU_UI_PATH);

    // Get Menu object
    let menu_object: Option<Menu> = menu_builder.object("menu");

    // Get file path of the logo image
    let image_logo_path = Path::new(LOGO_PATH);

    // Create pixbuf from file path of the app logo image
    let image_logo_pixbuf = Pixbuf::from_file(&image_logo_path);

    // Get the Pixbuf value of file_pixbuf if the file exists
    let image_logo_pixbuf = image_logo_pixbuf.expect("File Not Found: app logo image not found!");

    // Create picture
    let app_logo = Picture::for_pixbuf(&image_logo_pixbuf);

    // Create header bar
    let header_bar = HeaderBar::new();

    // Create menu button
    let menu_button = MenuButton::new();
    menu_button.set_icon_name("view-list"); // set menu button icon

    // Action items for the menu
    let about_action = SimpleAction::new("about", None);
    let open_action = SimpleAction::new("open", None);
    let save_action = SimpleAction::new("save", None);

    // Get Menu
    let menu = menu_object.unwrap();

    // Create Popover Menu from menu
    let popover_menu = PopoverMenu::from_model(Some(&menu));

    // Create stack
    let stack = Stack::new();
   
    // Create entry
    let text_view = TextView::new();

    /* Connect callbacks */
    about_action.connect_activate(clone!(@strong window =>
        move |_, _| {
            // create about dialog here
            // About Dialog 
            let about_dialog = AboutDialog::builder()
                .transient_for(&window) // the temporary parent of the window 
                .modal(true) // freezes the rest of the app from user input
                .logo(&app_logo.paintable().unwrap())
                .version(APP_VERSION)
                .comments(DESCRIPTION)
                .copyright(format!("{}{}", COPYRIGHT_FORMAT, AUTHORS).as_str())
                .authors(vec![String::from(AUTHORS)])
                .license(LICENSE)
                .build();
            
            // Show the about dialog
            about_dialog.present();
        }));

    open_action.connect_activate(move |_, _| {
        println!("Open file");
    });

    save_action.connect_activate(move |_, _| {
        println!("Save file");
    });

    // Set popover for menu button
    menu_button.set_popover(Some(&popover_menu));

    // Add about button to the header bar
    header_bar.pack_end(&menu_button);

    /* Attach widgets to the Grid */
    stack.add_child(&text_view);

    // Add about action to the application
    application.add_action(&about_action);
    application.add_action(&open_action);
    application.add_action(&save_action);

    // Set the window title bar
    window.set_titlebar(Some(&header_bar));

    // set stack as a child of window
    window.set_child(Some(&stack));

    // Present the window
    window.present();
}

