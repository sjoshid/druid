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

use std::{cmp, thread, time};
use std::num::NonZeroU64;

use druid::{Affine, AppLauncher, Color, Data, FontDescriptor, Lens, LocalizedString, MouseButton, Point, Rect, TextLayout, theme, WidgetExt, WidgetPod, WindowDesc};
use druid::kurbo::BezPath;
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::{Checkbox, Container, Flex, Label, List};
use druid::widget::prelude::*;

#[derive(Clone, Data, Lens, Debug)]
struct MyData {
	value: bool,
}

fn ui_builder() -> impl Widget<MyData> {
	let mut c1 = Flex::column();

	let label_1 = Label::dynamic(|data: &bool, env: &Env| data.to_string()).with_id(WidgetId::next()).lens(MyData::value);
	let label_2 = Label::new(String::from("static label 1"));
	let label_3 = Label::new(String::from("static label 2"));
	let my_cb = Checkbox::new("click me").with_id(WidgetId::next()).lens(MyData::value);

	//c1.add_child(label_1);
	c1.add_child(label_2);
	c1.add_child(label_3);
	c1.add_child(my_cb);
	c1
}


pub fn main() {
	let main_window = WindowDesc::new(ui_builder).title("Testing something..");

	let mut app_state = MyData {
		value: true,
	};

	AppLauncher::with_window(main_window)
		.use_simple_logger()
		.launch(app_state)
		.expect("launch failed");
}
