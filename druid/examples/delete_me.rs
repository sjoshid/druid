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

use druid::kurbo::BezPath;
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{Affine, AppLauncher, Color, FontDescriptor, LocalizedString, Point, Rect, TextLayout, WindowDesc, WidgetPod, WidgetExt, MouseButton, Data, Lens, theme};
use druid::widget::{Label, Container, Flex};
use std::cmp;
use im::Vector;

/// Wraps a Label in a Container.
/// I chose Container because it takes a &mut that adds a border. Not sure if this is the right choice.
struct CustomLabelWrapper {
    label_in_container: Container<String>,
}

impl CustomLabelWrapper {
    pub fn new(text: String) -> Self {
        CustomLabelWrapper {
            label_in_container: Container::new(Label::new(text)),
        }
    }
}

impl Widget<(String, bool)> for CustomLabelWrapper {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut (String, bool), env: &Env) {
        match event {
            Event::MouseDown(mouse_event) => {
                if mouse_event.button == MouseButton::Left {
                    ctx.set_active(true);
                    //ctx.request_paint(); //not needed
                }
            }
            Event::MouseUp(mouse_event) => {
                if ctx.is_active() && mouse_event.button == MouseButton::Left {
                    ctx.set_active(false);
                    if ctx.is_hot() {
                        // do nothing
                    }
                    //ctx.request_paint(); //not needed
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &(String, bool), env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.label_in_container.lifecycle(ctx, event, &(*data).0, env);
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &(String, bool), data: &(String, bool), env: &Env) {
        self.label_in_container.update(ctx, &(*old_data).0, &(*data).0, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &(String, bool), env: &Env) -> Size {
        self.label_in_container.layout(ctx, bc, &(*data).0, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &(String, bool), env: &Env) {
        let border = data.1;
        println!("inner paint ");
        self.label_in_container.paint(ctx, &(*data).0, env);
        if border {
            println!("i {:?}", *data);
            self.label_in_container.set_border(Color::WHITE, 1.);
        } else {
            println!("i {:?}", *data);
            self.label_in_container.set_border(theme::BACKGROUND_LIGHT, 1.);
        }
    }
}

/// A container widget that makes sure only one of the contained widgets has a border.
/// Clicking a different widget removes the border from old widget and add a border to clicked
/// widget.
struct CustomWidgetContainer {
    label: Vec<WidgetPod<(String, bool), CustomLabelWrapper>>,
}

impl CustomWidgetContainer {
    fn new() -> Self {
        CustomWidgetContainer {
            label: vec![],
        }
    }
}

impl Widget<AppState> for CustomWidgetContainer {
    fn event(&mut self, ctx: &mut EventCtx, _event: &Event, data: &mut AppState, _env: &Env) {
        let mut labels = &mut data.labels;
        for (i, l) in self.label.iter_mut().enumerate() {
            let mut active = None;
            let mut inside_label = &mut labels[i];
            l.event(ctx, _event, &mut ((*inside_label).clone(), false), _env);
            if l.is_active() {
                println!("l index {:?}", i);
                active = Some(i);
            }
            if active.is_some() {
                // Make current active index, inactive.
                if data.active_index.is_some() {
                    data.inactive_index = data.active_index;
                }
                // Make clicked widget, active.
                data.active_index = active;
                println!("active index {:?}", data.active_index);
                println!("inactive index {:?}", data.inactive_index);
            }
            ctx.request_paint();
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &AppState,
        _env: &Env,
    ) {
        match event {
            LifeCycle::WidgetAdded => {
                for d in data.labels.iter() {
                    let wrapper = CustomLabelWrapper::new(d.clone());
                    let pod = WidgetPod::new(wrapper);
                    self.label.push(pod);
                }
            }
            _ => {}
        }
        let labels = &data.labels;
        for (i, l) in self.label.iter_mut().enumerate() {
            let inside_label = &labels[i];
            l.lifecycle(_ctx, event, &(inside_label.clone(), false), _env);
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, data: &AppState, _env: &Env) {
        let labels = &data.labels;
        for (i, l) in self.label.iter_mut().enumerate() {
            let inside_label = &labels[i];
            l.update(_ctx, &(inside_label.clone(), false), _env);
        }
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        let mut height = 0.;
        let mut width: f64 = 0.;
        let x_position = 0.;
        let mut y_position = 0.;
        let labels = &data.labels;

        for (i, l) in self.label.iter_mut().enumerate() {
            let inside_label = &labels[i];
            let size = l.layout(layout_ctx, bc, &(inside_label.clone(), false), env);
            let rect = Rect::new(
                x_position,
                y_position,
                x_position + size.width,
                y_position + size.height,
            );

            l.set_layout_rect(layout_ctx, &(inside_label.clone(), false), env, rect);
            y_position += size.height;
            height += size.height;
            width = width.max(size.width);
        };

        Size { width, height }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        let labels = &data.labels;
        println!("outer paint ");
        for (i, l) in self.label.iter_mut().enumerate() {
            let inside_label = &labels[i];
            if data.active_index.is_some() && i == data.active_index.unwrap() {
                //pass true to paint
                println!("o {:?}", (inside_label.clone(), true));
                l.paint(ctx, &(inside_label.clone(), true), env);
            } else {
                if data.inactive_index.is_some() && i == data.inactive_index.unwrap() {
                    //pass false to paint
                    println!("o {:?}", (inside_label.clone(), false));
                    l.paint(ctx, &(inside_label.clone(), false), env);
                } else {
                    //for anything else pass false
                    println!("o {:?}", (inside_label.clone(), false));
                    l.paint(ctx, &(inside_label.clone(), false), env);
                }
            }
        }
    }
}

#[derive(Clone, Data, Lens, Debug)]
pub struct AppState {
    active_index: Option<usize>,
    inactive_index: Option<usize>,
    labels: Vector<String>,
}

pub fn main() {
    let main_window = WindowDesc::new(ui_builder).title("Testing something..");
    let mut labels = Vector::new();
    labels.push_back(String::from("First"));
    labels.push_back(String::from("Middle"));
    labels.push_back(String::from("Last"));
    let active_index = None;
    let inactive_index = None;

    let app_state = AppState {
        active_index,
        inactive_index,
        labels,
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(app_state)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<AppState> {
    let mut c1 = Flex::column();

    let custom_widget = CustomWidgetContainer::new();

    c1.add_child(custom_widget);
    c1//.debug_paint_layout()
}
