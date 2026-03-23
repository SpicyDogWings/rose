use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

#[tokio::main]
async fn main() {
    let app = Application::builder()
        .application_id("org.gtk.rose")
        .build();
    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Rose")
            .default_width(800)
            .default_height(600)
            .build();
        let button = gtk4::Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Button clicked!");
        });
        window.set_child(Some(&button));
        window.show();
    });
    app.run();
}
