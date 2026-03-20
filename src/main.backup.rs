use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation, Paned};

const APP_ID: &str = "org.gtk_rs.rose";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        // Crear la ventana principal
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Rose")
            .default_width(800)
            .default_height(600)
            .build();

        // --- Panel lateral izquierdo (sidebar) ---
        let sidebar = GtkBox::new(Orientation::Vertical, 0);
        sidebar.set_margin_start(10);
        sidebar.set_margin_end(10);
        sidebar.set_margin_top(10);
        sidebar.set_margin_bottom(10);
        sidebar.append(&Button::with_label("Inicio"));
        sidebar.append(&Button::with_label("Proyectos"));
        sidebar.append(&Button::with_label("Estadísticas"));
        sidebar.set_hexpand(false);
        sidebar.set_width_request(150);

        // --- Área de contenido principal ---
        let content = GtkBox::new(Orientation::Vertical, 6);
        content.append(&Label::new(Some("Bienvenido al Dashboard")));
        let click_button = Button::with_label("¡Haz clic para actualizar!");
        click_button.connect_clicked(|_| {
            println!("Actualizando datos...");
        });
        content.append(&click_button);
        content.set_margin_start(10);
        content.set_margin_end(10);
        content.set_margin_top(10);
        content.set_margin_bottom(10);

        // --- Layout principal con Paned (panel divisible) ---
        let paned = Paned::new(Orientation::Horizontal);
        paned.set_start_child(Some(&sidebar));
        paned.set_end_child(Some(&content));
        paned.set_position(180); // Ancho inicial del sidebar

        // --- Contenedor principal (vertical) ---
        let main_box = GtkBox::new(Orientation::Vertical, 0);
        main_box.append(&paned);

        // Añadir todo a la ventana
        window.set_child(Some(&main_box));

        // Añadir esto antes de presentar la ventana (`window.present()`)
        // let css_provider = gtk4::CssProvider::new();
        // css_provider.load_from_data(
        //     "
        //     #sidebar {
        //         background: #f0f0f0;
        //         padding: 10px;
        //     }
        //     #sidebar button {
        //         margin: 5px;
        //         padding: 8px;
        //         border-radius: 4px;
        //         background: #e0e0e0;
        //         border: 1px solid #ccc;
        //     }
        //     #sidebar button:hover {
        //         background: #d0d0d0;
        //     }
        //     ",
        // );
        // gtk4::style_context_add_provider_for_display(
        //     &gtk4::gdk::Display::default().expect("No display"),
        //     &css_provider,
        //     gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        // );
        // Asignar un ID al sidebar para aplicar el CSS
        // sidebar.set_widget_name("sidebar");

        window.present();
    });

    app.run()
}
