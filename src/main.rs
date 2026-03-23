use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use gtk4::gio::Resource;

#[tokio::main]
async fn main() {
    let resource = Resource::load("target/debug/compiled_resources.gresource")
        .expect("Failed to load resources");
    gtk4::gio::resources_register(&resource);
    let app = Application::builder()
        .application_id("org.gtk.test")
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
