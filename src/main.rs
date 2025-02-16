use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk4::{self as gtk, Label};
use ui::app::AppUI;

mod filesystem;
mod ui;

struct Config {
    ui: AppUI,
}

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

        let label = Label::new(Some("Ola jessica como foi seu dia"));
        window.set_child(Some(&label));
        window.present();
    });

    app.run()
}
