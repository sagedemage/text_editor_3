/* Setup the CSS to improve the app appearance */

use gdk::Display;
use gtk::{CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};

// resources/styles.css
pub fn load_css_file(css_file_path: &str) {
    /* Load the CSS file to change the 
     * appearance of the app */
    // CssProvider
    let provider = CssProvider::new();

    // Load the CSS file
    provider.load_from_path(css_file_path);

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

