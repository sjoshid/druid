use druid::widget::{
    Button, CrossAxisAlignment, Flex, Label, List, RadioGroup, Scroll, TextBox, WidgetExt,
};
use druid::widget::{Dropdown, DROP};
use druid::{AppLauncher, Color, Data, Env, EventCtx, Lens, UnitPoint, Widget, WindowDesc};
use im::Vector;

#[derive(Data, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Fruit {
    Apple,
    Pear,
    Orange,
}

#[derive(Data, Clone, Lens)]
struct FuzzySearchData {
    word: String,
    words: Vector<String>,
    tolerance: usize,
}

fn main_widget() -> impl Widget<FuzzySearchData> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_spacer(10.)
        .with_child(
            Dropdown::new(
                Flex::row()
                    .with_child(TextBox::new())
                    .with_flex_spacer(1.)
                    .lens(FuzzySearchData::word),
                |_, _| {
                    Scroll::new(List::new(|| {
                        Label::new(|item: &String, _env: &_| format!("List item #{}", item))
                            .align_vertical(UnitPoint::LEFT)
                            .padding(10.0)
                            .expand()
                            .height(50.0)
                            .background(Color::rgb(0.5, 0.5, 0.5))
                    }))
                    .vertical()
                    .lens(FuzzySearchData::words)
                },
            )
            .align_left(),
        )
        .padding(10.)
        .fix_width(250.)
}

pub fn main() {
    let main_window = WindowDesc::new(main_widget())
        .title("Dropdown")
        .window_size((250., 300.));

    // create the initial app state
    let initial_state = FuzzySearchData {
        word: String::new(),
        words: Vector::new(),
        tolerance: 3,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}
