mod ui;
mod jj;

use adw::prelude::*;
use adw::{ Application, ApplicationWindow };
use ui::MainUi;

#[tokio::main]
async fn main() {
    let app = Application::builder()
        .application_id("org.gtk.rose")
        .build();
    app.connect_activate(|app| {
      let ui = MainUi::new();
      let content = ui.build_ui();
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Rose")
            .default_width(800)
            .default_height(600)
            .build();
        window.set_content(Some(&content));
        window.present();
    });
    app.run();
}
