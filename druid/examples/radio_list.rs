use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

use druid::im::{vector, Vector};
use druid::widget::{Button, Flex, Label, MyRadio, Radio, RadioList, Scroll, TextBox, WidgetExt};
use druid::{
    AppLauncher, Data, Lens, LensExt, LocalizedString, Selector, Target, UnitPoint, Widget,
    WidgetId, WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct Directory {
    persons: Vector<String>,
    to_be_added: String,
}

fn ui_builder() -> impl Widget<Directory> {
    let mut root = Flex::column();
    let radio_list_widget_id = WidgetId::reserved(11111);
    root.add_flex_child(
        Scroll::new(
            RadioList::new(|item: &String, _env: &_| Label::new(item.clone()))
                .lens(Directory::persons)
                .with_id(radio_list_widget_id),
        )
        .align_left(),
        1.0,
    );

    let mut add_delete = Flex::row();
    add_delete.add_child(
        Button::new("Add")
            .on_click(|_ctx, persons: &mut Directory, _env| {
                let mut names = &mut persons.persons;
                let to_be_added = persons.to_be_added.clone();
                names.push_back(to_be_added);

                //reset
                persons.to_be_added = String::from("");
            })
            .fix_size(80.0, 20.0)
            .align_vertical(UnitPoint::CENTER),
    );
    add_delete.add_child(TextBox::new().lens(Directory::to_be_added));
    add_delete.add_child(Button::new("Delete").on_click(move |ctx, _data, _env| {
        ctx.submit_command(
            druid::widget::radio_list::DELETE_SELECTED_NAME,
            Target::Widget(radio_list_widget_id),
        )
    }));
    root.add_child(add_delete);
    root.debug_paint_layout()
}

pub fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("list-demo-window-title").with_placeholder("List Demo"));

    let directory = Directory {
        persons: vector![String::from("Sujit"), String::from("Morgan")],
        to_be_added: String::from(""),
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(directory)
        .expect("launch failed");
}
