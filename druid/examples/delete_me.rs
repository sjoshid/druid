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

//! An example of a custom drawing widget.
//! We draw an image, some text, a shape, and a curve.

use druid::kurbo::BezPath;
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{Affine, AppLauncher, Color, FontDescriptor, LocalizedString, Point, Rect, TextLayout, WindowDesc, WidgetPod, WidgetExt, MouseButton, Data, Lens};
use druid::widget::{Label, Container, Flex};
use std::cmp;
use im::Vector;

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

impl Widget<(String, usize, usize)> for CustomLabelWrapper {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut (String, usize, usize), env: &Env) {
        match event {
            Event::MouseDown(mouse_event) => {
                if mouse_event.button == MouseButton::Left && !self.label_in_container.border_is_some() {
                    self.label_in_container.set_border(Color::WHITE, 1.);
                    ctx.set_active(true);
                    ctx.request_paint();
                }
            }
            Event::MouseUp(mouse_event) => {
                if ctx.is_active() && mouse_event.button == MouseButton::Left {
                    ctx.set_active(false);
                    if ctx.is_hot() {
                        //
                    }
                    ctx.request_paint();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &(String, usize, usize), env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.label_in_container.lifecycle(ctx, event, &(*data).0, env);
            }
            LifeCycle::FocusChanged(is_focused) => {
                println!("Focused {}", is_focused);
            }
            LifeCycle::HotChanged(is_hot) => {
                //println!("is_hot {}", is_hot);
                /*if *is_hot {
                    self.label.set_border(Color::WHITE, 1.);
                } else {
                    self.label.set_border(Color::rgb(0., 100., 0.), 1.);
                }*/
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &(String, usize, usize), data: &(String, usize, usize), env: &Env) {
        self.label_in_container.update(ctx, &(*old_data).0, &(*data).0, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &(String, usize, usize), env: &Env) -> Size {
        self.label_in_container.layout(ctx, bc, &(*data).0, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &(String, usize, usize), env: &Env) {
        println!("painting..");
        self.label_in_container.paint(ctx, &(*data).0, env);
    }
}

struct CustomWidget {
    label: Vec<WidgetPod<(String, usize, usize), CustomLabelWrapper>>,
}

impl CustomWidget {
    fn new() -> Self {
        CustomWidget {
            label: vec![],
        }
    }
}

impl Widget<AppState> for CustomWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, data: &mut AppState, _env: &Env) {
        let mut labels = &mut data.labels;
        for (i, l) in self.label.iter_mut().enumerate() {
            let mut active = None;
            let mut inside_label = &mut labels[i];
            l.event(_ctx, _event, &mut ((*inside_label).clone(), i, i), _env);
            if l.is_active() {
                println!("l index {:?}", i);
                active = Some(i);
            }
            if active.is_some() {
                if data.active_index.is_some() {
                    data.inactive_index = data.active_index;
                }
                data.active_index = active;
                println!("active index {:?}", data.active_index);
                println!("inactive index {:?}", data.inactive_index);
            }
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
            l.lifecycle(_ctx, event, &(inside_label.clone(), i, i), _env);
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, data: &AppState, _env: &Env) {
        let labels = &data.labels;
        for (i, l) in self.label.iter_mut().enumerate() {
            let inside_label = &labels[i];
            l.update(_ctx, &(inside_label.clone(), i, i), _env);
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
            let size = l.layout(layout_ctx, bc, &(inside_label.clone(), i, i), env);
            let rect = Rect::new(
                x_position,
                y_position,
                x_position + size.width,
                y_position + size.height,
            );

            l.set_layout_rect(layout_ctx, &(inside_label.clone(), i, i), env, rect);
            y_position += size.height;
            height += size.height;
            width = width.max(size.width);
        };

        Size { width, height }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        let labels = &data.labels;
        for (i, l) in self.label.iter_mut().enumerate() {
            let inside_label = &labels[i];
            l.paint(ctx, &(inside_label.clone(), i, i), env);
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    active_index: Option<usize>,
    inactive_index: Option<usize>,
    labels: Vector<String>,
}

pub fn main() {
    let main_window = WindowDesc::new(ui_builder).title("Testing something..");
    let mut labels = Vector::new();
    labels.push_back(String::from("Sujit"));
    labels.push_back(String::from("Joshi"));
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

    let custom_widget = CustomWidget::new();

    c1.add_child(custom_widget);
    c1//.debug_paint_layout()
}
