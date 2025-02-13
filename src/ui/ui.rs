use gtk4::{gio::Application, Application, ApplicationWindow};

pub fn build_app() {
    let app = Application::new(Some(""), Default::default());
}

fn build_main_window(app: &Application) {
    let window = ApplicationWindow::builder().application(app);
}
