use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

use druid::im::{vector, Vector};
use druid::widget::{Button, Flex, Label, MyRadio, RadioList, Scroll, WidgetExt};
use druid::{AppLauncher, Data, Lens, LensExt, LocalizedString, UnitPoint, Widget, WindowDesc};

#[derive(Clone, Data, Lens)]
struct Directory {
    persons: Vector<String>,
}

fn ui_builder() -> impl Widget<Directory> {
    let mut root = Flex::column();
    root.add_flex_child(
        Scroll::new(RadioList::new(|item: &String, _env: &_| {
            Label::new(item.clone())
        }))
        .align_left()
        .lens(Directory::persons),
        1.0,
    );
    root.add_child(
        Button::new("Add")
            .on_click(|_ctx, persons: &mut Vector<String>, _env| {
                persons.push_back(String::from("Nekko"));
                /*let persons_vec = persons.borrow_mut();
                persons_vec.push(String::from("Nekko"));*/
                println!("size is {}", persons.len())
            })
            .lens(Directory::persons)
            .fix_size(80.0, 20.0)
            .align_vertical(UnitPoint::CENTER),
    );
    root.debug_paint_layout()
}

pub fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("list-demo-window-title").with_placeholder("List Demo"));

    let directory = Directory {
        persons: vector![String::from("Sujit"), String::from("Morgan")],
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(directory)
        .expect("launch failed");
}
