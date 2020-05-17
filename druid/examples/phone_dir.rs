use std::sync::Arc;

use druid::{
    AppLauncher, Color, Data, Lens, LocalizedString, UnitPoint, Widget, WidgetExt, WindowDesc,
};
use druid::lens::{self, LensExt};
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll};

#[derive(Clone, Data, Lens)]
struct PersonDetails {
    first_name: String,
    last_name: String,
    phone_number: String,
}

#[derive(Clone, Data, Lens)]
struct Directory {
    persons: Arc<Vec<PersonDetails>>,
    selected_person: u64,
}

pub fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("list-demo-window-title").with_placeholder("List Demo"));
    let mut names = Vec::new();
    let person1 = PersonDetails {
        last_name: String::from("Joshi"),
        first_name: String::from("Sujit"),
        phone_number: String::from("1212"),
    };
    names.push(person1);
    let person2 = PersonDetails {
        last_name: String::from("Joshi"),
        first_name: String::from("Kapil"),
        phone_number: String::from("345345"),
    };
    names.push(person2);
    let person3 = PersonDetails {
        last_name: String::from("Joshi"),
        first_name: String::from("Kapil"),
        phone_number: String::from("345345"),
    };
    names.push(person3);
    let person4 = PersonDetails {
        last_name: String::from("Joshi"),
        first_name: String::from("Kapil"),
        phone_number: String::from("345345"),
    };
    names.push(person4);
    let person5 = PersonDetails {
        last_name: String::from("Joshi"),
        first_name: String::from("Kapil"),
        phone_number: String::from("345345"),
    };
    names.push(person5);
    let person6 = PersonDetails {
        last_name: String::from("Joshi"),
        first_name: String::from("Kapil"),
        phone_number: String::from("345345"),
    };
    names.push(person6);
    let person7 = PersonDetails {
        last_name: String::from("Joshi"),
        first_name: String::from("Kapil"),
        phone_number: String::from("345345"),
    };
    names.push(person7);

    let directory = Directory {
        persons: Arc::new(names),
        selected_person: 0,
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(directory)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<Directory> {
    let mut root = Flex::column();

    let mut person_list = Flex::row();

    // Build a simple list
    person_list.add_flex_child(
        Scroll::new(List::new(|| {
            Label::new(|item: &PersonDetails, _env: &_| format!("{}, {}", item.last_name, item.first_name))
                .align_vertical(UnitPoint::LEFT)
                .padding(10.0)
                .expand()
                .height(50.0)
                .background(Color::rgb(0.5, 0.5, 0.5))
        }))
        .vertical()
        .lens(Directory::persons),
        1.0,
    );

    let mut manage_persons = Flex::row();
    manage_persons.add_flex_child(
        Button::new("Add")
        .on_click(|_, data: &mut Directory, _| {

        }),
        1.0
    );

    root.add_flex_child(person_list,5.0);
    root.add_flex_child(manage_persons,1.0);
    root.debug_paint_layout()
}
