// src/lib.rs

extern crate gtk;
extern crate webkit2gtk;

use gtk::prelude::*;
use webkit2gtk::{WebView, WebViewExt};

pub struct VoidRenderer;

impl VoidRenderer {
    pub fn render(html: &str) {
        // Initialize GTK
        gtk::init().expect("Failed to initialize GTK.");

        // Create a new GTK window
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_title("VoidRenderer");
        window.set_default_size(800, 600);

        // Create a new WebView
        let webview = WebView::new();

        // Load HTML into WebView
        webview.load_html(html, None);

        // Add WebView to the window
        window.add(&webview);

        // Show all widgets
        window.show_all();

        // Run the GTK main loop
        gtk::main();
    }
}

