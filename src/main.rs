mod jj;

use gtk4::prelude::*;
use gtk4::{ Application,
  ApplicationWindow,
  Label,
  ScrolledWindow
};

const APP_ID: &str = "org.gtk_rs.rose";

#[tokio::main]
async fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Rose")
            .default_width(800)
            .default_height(600)
            .build();
        let scrolled_jj_log = ScrolledWindow::new();
        window.set_child(Some(&scrolled_jj_log));
        glib::MainContext::default().spawn_local(async move {
            let jj_logs = jj::run_jj_log().await;
            let label = Label::new(Some(&jj_logs));
            label.set_wrap(true);
            label.set_selectable(true);
            scrolled_jj_log.set_child(Some(&label));
            glib::MainContext::default().spawn_local(async move {
              scrolled_jj_log.set_child(Some(&label));
            });
        });
        window.present();
    });
    app.run()
}
