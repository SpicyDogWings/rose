use adw::prelude::*;
use adw::{ActionRow, HeaderBar};
use gtk4::{Box, ListBox, Orientation, SelectionMode};

pub struct MainUi {}

impl MainUi {
    pub fn build_ui(&self) -> Box {
        let row = ActionRow::builder()
            .activatable(true)
            .title("Click me")
            .build();
        row.connect_activated(|_| {
            eprintln!("Clicked!");
        });
        let list = self.build_list(vec![row]);
        // let list = ListBox::builder()
        //     .margin_top(32)
        //     .margin_end(32)
        //     .margin_bottom(32)
        //     .margin_start(32)
        //     .selection_mode(SelectionMode::None)
        //     .css_classes(vec![String::from("boxed-list")])
        //     .build();
        // list.append(&row);
        let content = Box::new(Orientation::Vertical, 0);
        content.append(&HeaderBar::new());
        content.append(&list);
        content
    }

    fn build_list(&self, rows: Vec<ActionRow>) -> ListBox {
        let center_margin = 32;
        let list = ListBox::builder()
            .margin_top(center_margin)
            .margin_end(center_margin)
            .margin_bottom(center_margin)
            .margin_start(center_margin)
            .selection_mode(SelectionMode::None)
            .css_classes(vec![String::from("boxed-list")])
            .build();
        for row in rows {
            list.append(&row);
        }
        list
    }
}
