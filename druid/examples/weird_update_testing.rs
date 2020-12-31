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

use std::{cmp, time, thread};

use druid::{Affine, AppLauncher, Color, Data, FontDescriptor, Lens, LocalizedString, MouseButton, Point, Rect, TextLayout, theme, WidgetExt, WidgetPod, WindowDesc};
use druid::kurbo::BezPath;
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::{Container, Flex, Label, List};
use druid::widget::prelude::*;

#[derive(Clone, Data, Lens, Debug)]
struct MyData {
	data_for_widget_a: u32,
	data_for_widget_b: u32
}

fn ui_builder() -> impl Widget<MyData> {
	let mut c1 = Flex::column();

	let label_1 = Label::dynamic(|data: &u32, env: &Env| data.to_string()).lens(MyData::data_for_widget_a);
	let label_2 = Label::new(|data: &u32, env: &Env| data.to_string()).lens(MyData::data_for_widget_b);

	let my_list = List::new(|| Label::new())
		.horizontal()
		.with_spacing(10.0)
		.lens(AppState::thumbnails);

	c1.add_child(label_1);
	c1.add_child(label_2);
	c1//.debug_paint_layout()
}


pub fn main() {
	let main_window = WindowDesc::new(ui_builder).title("Testing something..");

	let mut app_state = MyData {
		data_for_widget_a: 0,
		data_for_widget_b: 0,
	};

	AppLauncher::with_window(main_window)
		.use_simple_logger()
		.launch(app_state)
		.expect("launch failed");
}
