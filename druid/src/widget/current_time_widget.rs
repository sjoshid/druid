use crate::calendar_data::{DAYS_OF_WEEK, DEFAULT_DAY_WIDGET_SIZE, DEFAULT_GRID_SPACING};
use crate::{FontFamily, Rect};
use chrono::{Local, Timelike};
use druid::widget::{Container, Label};
use druid::{
    BoxConstraints, Color, CurrentTimeData, Env, Event, EventCtx, FontDescriptor, KeyOrValue,
    LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, TimerToken, UpdateCtx, Widget, WidgetPod,
};
use std::ops::Add;
use std::sync::Arc;
use std::time::Duration;

pub struct CurrentTimeWidget {
    time_label: WidgetPod<CurrentTimeData, Container<CurrentTimeData>>,
    am_pm_label: WidgetPod<CurrentTimeData, Container<CurrentTimeData>>,
    timer_id: TimerToken,
}

impl CurrentTimeWidget {
    pub fn new() -> Self {
        let hour = Container::new(
            Label::dynamic(CurrentTimeWidget::create_time_label())
                .with_font(FontDescriptor::new(FontFamily::SERIF).with_size(34.0)),
        );
        let hour_label = WidgetPod::new(hour);

        let am_pm = Container::new(
            Label::dynamic(CurrentTimeWidget::am_pm_label())
                .with_font(FontDescriptor::new(FontFamily::SERIF).with_size(8.0)),
        );
        let am_pm_label = WidgetPod::new(am_pm);

        CurrentTimeWidget {
            time_label: hour_label,
            am_pm_label,
            timer_id: TimerToken::INVALID,
        }
    }

    fn create_time_label() -> impl Fn(&CurrentTimeData, &Env) -> String + 'static {
        |c: &CurrentTimeData, _| {
            if c.twelve_hour_format {
                let mut hour = c.current_hour_of_day % 12;
                if hour == 0 {
                    hour = 12;
                }
                format!(
                    "{:0>2}:{:0>2}:{:0>2}",
                    hour, c.current_minute_of_hour, c.current_second_of_minute
                )
            } else {
                format!(
                    "{:0>2}:{:0>2}:{:0>2}",
                    c.current_hour_of_day, c.current_minute_of_hour, c.current_second_of_minute
                )
            }
        }
    }

    fn am_pm_label() -> impl Fn(&CurrentTimeData, &Env) -> String + 'static {
        |c: &CurrentTimeData, _| {
            if c.twelve_hour_format {
                if c.current_hour_of_day < 12 {
                    String::from("AM")
                } else {
                    String::from("PM")
                }
            } else {
                String::from("")
            }
        }
    }
}

impl Widget<CurrentTimeData> for CurrentTimeWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut CurrentTimeData, env: &Env) {
        //update here
        match event {
            Event::WindowConnected => {
                let deadline = Duration::from_millis(1000);
                self.timer_id = ctx.request_timer(deadline);
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    let deadline = Duration::from_millis(1000); // one sec
                    let today = Local::now();
                    data.current_hour_of_day = today.hour();
                    data.current_minute_of_hour = today.minute();
                    data.current_second_of_minute = today.second();

                    self.timer_id = ctx.request_timer(deadline);
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &CurrentTimeData,
        env: &Env,
    ) {
        match event {
            LifeCycle::WidgetAdded => {
                self.time_label.lifecycle(ctx, event, &data, env);
                self.am_pm_label.lifecycle(ctx, event, &data, env);
            }
            _ => {}
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &CurrentTimeData,
        data: &CurrentTimeData,
        env: &Env,
    ) {
        self.time_label.update(ctx, &data, env);
        self.am_pm_label.update(ctx, &data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &CurrentTimeData,
        env: &Env,
    ) -> Size {
        let time_label_size = self.time_label.layout(ctx, bc, &data, env);

        let mut x_position = DEFAULT_GRID_SPACING;
        let mut y_position = DEFAULT_GRID_SPACING;

        let time_rect = Rect::new(
            x_position,
            y_position,
            x_position + time_label_size.width,
            y_position + time_label_size.height,
        );
        self.time_label.set_layout_rect(ctx, &data, env, time_rect);
        x_position += x_position + time_label_size.width + DEFAULT_GRID_SPACING;

        let am_pm_label = self.am_pm_label.layout(ctx, bc, &data, env);
        y_position += time_label_size.height - am_pm_label.height - 5.;

        let am_pm_rect = Rect::new(
            x_position,
            y_position,
            x_position + am_pm_label.width,
            y_position + am_pm_label.height,
        );
        self.am_pm_label
            .set_layout_rect(ctx, &data, env, am_pm_rect);

        Size {
            width: (DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING)
                * DAYS_OF_WEEK.len() as f64,
            height: time_label_size.height,
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &CurrentTimeData, env: &Env) {
        self.time_label.paint(ctx, &data, env);
        self.am_pm_label.paint(ctx, &data, env);
    }
}
