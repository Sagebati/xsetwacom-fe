


extern crate gio;
extern crate gtk;

mod fe;
mod xsetwacom;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    ApplicationWindow, CellRendererText, Label, ListStore, Orientation, TreeView, TreeViewColumn,
    WindowPosition,
};

use std::env::args;


fn main() {
    /*let application = gtk::Application::new("com.github.simple_treeview",
                                            gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());*/
    fe::gtk_fe::init();

}

