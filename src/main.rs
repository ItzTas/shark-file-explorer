use filesystem::manager::list;
use global_config::config::GlobalConfig;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk4::{self as gtk, Box, Label};

mod filesystem;
mod global_config;
mod ui;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.shark.FileExplorer")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .build();

        let vbox = Box::new(gtk::Orientation::Vertical, 5);

        let _global_config = match GlobalConfig::new(String::from("."), &window, &app) {
            Err(e) => {
                println!("Error creatig global config: {}", e);
                std::process::exit(1)
            }
            Ok(config) => config,
        };

        match list(".") {
            Err(e) => print!("Error: {}\n", e),
            Ok(file_items) => {
                for file_item in file_items {
                    print!("{}", file_item.get_name());
                    let label = Label::new(Some(file_item.get_name()));
                    vbox.append(&label);
                }
            }
        }

        window.set_child(Some(&vbox));
        window.present();
    });

    app.run()
}
