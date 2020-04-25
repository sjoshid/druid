// Copyright 2019 The xi-editor Authors.
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

//! An example of a timer.

use std::time::Duration;

use druid::{
    AppLauncher, BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    LocalizedString, PaintCtx, RenderContext, Size, TimerToken, UpdateCtx, Widget, WindowDesc,
};
use druid::kurbo::Line;
use druid::widget::{Flex, WidgetExt};

struct TimerWidget {
    timer_id: TimerToken,
    on: bool,
}

impl TimerWidget {
    fn new() -> TimerWidget {
        TimerWidget {
            timer_id: TimerToken::INVALID,
            on: false,
        }
    }
}

impl Widget<u32> for TimerWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut u32, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                self.on = !self.on;
                ctx.request_paint();
                let deadline = Duration::from_millis(1000);
                self.timer_id = ctx.request_timer(deadline);
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    self.on = !self.on;
                    ctx.request_paint();
                    println!("Received/triggering by id: {:?}", ctx.widget_id());
                    let deadline = Duration::from_millis(1000);
                    self.timer_id = ctx.request_timer(deadline);
                } else {
                    println!("Ignored by id: {:?}", ctx.widget_id());
                }
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &u32, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &u32, _data: &u32, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &u32,
        _env: &Env,
    ) -> Size {
        bc.constrain((100.0, 100.0))
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, _data: &u32, _env: &Env) {
        if self.on {
            paint_ctx.stroke(Line::new((10.0, 10.0), (10.0, 50.0)), &Color::WHITE, 1.0);
        }
    }
}

fn build_widget() -> impl Widget<u32> {
    let t1 = TimerWidget::new().debug_widget();
    let t2 = TimerWidget::new().debug_widget();
    let t3 = TimerWidget::new().debug_widget();
    let t4 = TimerWidget::new().debug_widget();
    //let label = Label::new(|data: &String, _env: &_| format!("value: {}", data));

    let column1 = Flex::column()
        .with_child(t1)
        .with_child(t2)
        .with_child(t3)
        .with_child(t4);

    let t5 = TimerWidget::new().debug_widget();
    let t6 = TimerWidget::new().debug_widget();
    let t7 = TimerWidget::new().debug_widget();
    let t8 = TimerWidget::new().debug_widget();
    //let label = Label::new(|data: &String, _env: &_| format!("value: {}", data));

    let column2 = Flex::column()
        .with_child(t5)
        .with_child(t6)
        .with_child(t7)
        .with_child(t8);

    let t9 = TimerWidget::new().debug_widget();
    let t10 = TimerWidget::new().debug_widget();
    let t11 = TimerWidget::new().debug_widget();
    let t12 = TimerWidget::new().debug_widget();
    //let label = Label::new(|data: &String, _env: &_| format!("value: {}", data));

    let column3 = Flex::column()
        .with_child(t9)
        .with_child(t10)
        .with_child(t11)
        .with_child(t12);

    let root = Flex::row().with_child(column1).with_child(column2)
        .with_child(column3);

    root
}

fn main() {
    let window = WindowDesc::new(build_widget)
        .title(LocalizedString::new("timer-demo-window-title").with_placeholder("Tick Tock"));

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(0u32)
        .expect("launch failed");
}
