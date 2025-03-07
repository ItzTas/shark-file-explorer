use filesystem::manager::list;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk4::{self as gtk, Box, Label};
// use ui::app::AppUI;

mod filesystem;
// mod ui;

// struct Config {
//     ui: AppUI,
// }

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.shark.FileExplorer")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        let vbox = Box::new(gtk::Orientation::Vertical, 5);

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
