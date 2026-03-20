mod jj;

use gtk4::prelude::*;
use gtk4::{ Application,
  CssProvider,
  Label,
  ListBox,
  ListBoxRow,
  ScrolledWindow,
  Button
};
use adw::{ ApplicationWindow, prelude::AdwApplicationWindowExt };

const APP_ID: &str = "org.gtk_rs.rose";

#[tokio::main]
async fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        // Load CSS styles
        let provider = CssProvider::new();
        provider.load_from_path("src/style.css");

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Rose")
            .default_width(800)
            .default_height(600)
            .build();

        // AdwApplicationWindow has built-in close button functionality
        // We can add a custom close button to the content area
        let close_button = Button::with_label("Close");
        let window_weak = window.downgrade();
        close_button.connect_clicked(move |_| {
            if let Some(window) = window_weak.upgrade() {
                window.close();
            }
        });

        // Apply CSS styles to the window
        gtk4::style_context_add_provider_for_display(
            &gtk4::prelude::RootExt::display(&window),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Create main vertical box to hold close button and scrolled window
        let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
        
        // Add close button at the top
        main_box.append(&close_button);
        
        // Add scrolled window with list below
        let scrolled_jj_log = ScrolledWindow::new();
        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk4::SelectionMode::None);
        scrolled_jj_log.set_child(Some(&list_box));
        scrolled_jj_log.set_vexpand(true);
        main_box.append(&scrolled_jj_log);
        
        window.set_content(Some(&main_box));

        glib::MainContext::default().spawn_local(async move {
            let commits = jj::run_jj_log().await;
            for commit in commits {
                let current_marker = if commit.is_current { " @" } else { "" };
                let commit_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
                let commit_id_label = Label::new(Some(&format!("{}{}", commit.commit_id, current_marker)));
                commit_id_label.set_width_chars(15);
                commit_id_label.set_xalign(0.0);
                commit_id_label.set_selectable(true);

                // Description label (right side, expands)
                let desc_label = Label::new(Some(&commit.description));
                desc_label.set_wrap(true);
                desc_label.set_selectable(true);
                desc_label.set_xalign(0.0);
                desc_label.set_hexpand(true);

                // Pack labels into the box
                commit_box.append(&commit_id_label);
                commit_box.append(&desc_label);

                // Create row and add the box
                let row = ListBoxRow::new();
                row.set_child(Some(&commit_box));
                list_box.append(&row);
            }
        });
        window.present();
    });
    app.run()
}
