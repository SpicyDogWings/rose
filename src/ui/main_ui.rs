use adw::{ActionRow, HeaderBar};
use gtk4::{Box, ListBox, Orientation, SelectionMode};
use adw::prelude::*;

pub struct MainUi {}

impl MainUi {
    pub fn build_ui() -> Box {
        let row = ActionRow::builder()
            .activatable(true)
            .title("Click me")
            .build();
        row.connect_activated(|_| {
            eprintln!("Clicked!");
        });
        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(SelectionMode::None)
            .css_classes(vec![String::from("boxed-list")])
            .build();
        list.append(&row);
        let content = Box::new(Orientation::Vertical, 0);
        content.append(&HeaderBar::new());
        content.append(&list);
        content
    }
}
