// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use druid::{
	AppLauncher, Data, Env, FontDescriptor, FontFamily, Lens, LocalizedString, UnitPoint, Widget,
	WidgetExt, WindowDesc,
};
use druid::widget::{Flex, Label, TextBox, Button};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct HelloState {
	event_title: String,
	event_start_time: String,
	event_end_time: String,
	event_location: String,
}

pub fn main() {
	// describe the main window
	let main_window = WindowDesc::new(build_root_widget)
		.title(WINDOW_TITLE)
		.window_size((400.0, 400.0));

	// create the initial app state
	let initial_state = HelloState {
		event_title: String::from("Event title"),
		event_start_time: String::from("Event start time"),
		event_end_time: String::from("Event end time"),
		event_location: String::from("Event location"),
	};

	// start the application
	AppLauncher::with_window(main_window)
		.launch(initial_state)
		.expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
	// let today_label = Label::dynamic(|data: &(u32, u32, i32), _| "Today".to_string()).lens(DateWidgetData::current_day_month_year);
	let today_label = Label::new(String::from("Today"));
	let close_button = Button::new("Close");

	let event_textbox = TextBox::new().with_placeholder("Add an event or reminder").lens(HelloState::event_title);
	let from_label = Label::new("From");
	let to_label = Label::new("To");

	let event_start_time_input = TextBox::new().lens(HelloState::event_start_time);
	let event_end_time_input = TextBox::new().lens(HelloState::event_end_time);

	let location_textbox = TextBox::new().with_placeholder("Add Location").lens(HelloState::event_location);

	let save_button = Button::new("Save");
	let cancel_button = Button::new("Cancel");

	let mut today_label_flex = Flex::row();
	today_label_flex.add_child(today_label.align_left());
	let mut close_button_flex = Flex::row();
	close_button_flex.add_child(close_button.align_right());

	let mut top_row = Flex::row();
	top_row.add_child(today_label_flex);
	top_row.add_spacer(400.);
	top_row.add_child(close_button_flex);

	let mut from_to_row = Flex::row();
	from_to_row.add_child(from_label);
	from_to_row.add_child(event_start_time_input);
	from_to_row.add_child(to_label);
	from_to_row.add_child(event_end_time_input);

	let mut last_row = Flex::row();
	last_row.add_child(save_button.align_right());
	last_row.add_child(cancel_button.align_right());

	let mut event_details = Flex::column();
	event_details.add_child(top_row.align_vertical(UnitPoint::CENTER));

	let mut second_row_flex = Flex::row();
	let icon_label_flex = Flex::row().with_child(Label::new(String::from("ICON")));
	second_row_flex.add_child(icon_label_flex.center().align_left());
	let event_textbox_flex = Flex::row().with_child(event_textbox.padding(10.).center());
	second_row_flex.add_child(event_textbox_flex);

	event_details.add_child(second_row_flex);

	let mut third_row_flex = Flex::row();
	let icon_label = Label::new(String::from("ICON"));
	third_row_flex.add_child(icon_label.center());
	third_row_flex.add_child(from_to_row.padding(10.).align_left());

	event_details.add_child(third_row_flex);

	let mut fourth_row_flex = Flex::row();
	let icon_label = Label::new(String::from("ICON"));
	fourth_row_flex.add_child(icon_label.center());
	fourth_row_flex.add_child(location_textbox.padding(10.).center());

	event_details.add_child(fourth_row_flex);

	event_details.add_child(last_row.align_right());
	event_details//.debug_paint_layout()
}
