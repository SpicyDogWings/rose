use adw::prelude::*;
use adw::{ActionRow, HeaderBar};
use gtk4::{Box, ListBox, Orientation, SelectionMode};

pub struct MainUi {}

impl MainUi {
    pub fn new() -> Self {
        MainUi {}
    }

    pub fn build_ui(&self) -> Box {
        let elements = vec![
            String::from("Item 1"),
            String::from("Item 2"),
            String::from("Item 3"),
        ];
        let rows = self.build_row(elements);
        let list = self.build_list(rows);
        // row.connect_activated(|_| {
        //     eprintln!("Clicked!");
        // });
        let content = Box::new(Orientation::Vertical, 0);
        content.append(&HeaderBar::new());
        content.append(&list);
        content
    }

    fn build_row(&self, elements: Vec<String>) -> Vec<ActionRow> {
        let mut rows = Vec::new();
        for element in elements {
            let row = ActionRow::builder()
                .activatable(true)
                .title(&element)
                .build();
            rows.push(row);
        }
        rows
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
