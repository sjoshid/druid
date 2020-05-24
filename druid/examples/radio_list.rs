use druid::widget::{Flex, Label, MyRadio, RadioList, Scroll, WidgetExt};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc};
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
struct Directory {
    persons: Arc<Vec<String>>,
    selected_person: u64,
}

fn ui_builder() -> impl Widget<Directory> {
    let mut root = Flex::column();
    root.add_flex_child(
        Scroll::new(RadioList::new(|item: &String, _env: &_| {
            Label::new(item.clone())
        }))
        .lens(Directory::persons),
        1.0,
    );
    root
}

pub fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("list-demo-window-title").with_placeholder("List Demo"));

    let directory = Directory {
        persons: Arc::new(vec![String::from("Sujit"), String::from("Morgan")]),
        selected_person: 0,
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(directory)
        .expect("launch failed");
}
