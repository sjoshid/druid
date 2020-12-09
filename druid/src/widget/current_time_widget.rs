use druid::{WidgetPod, Widget, EventCtx, LifeCycle, PaintCtx, BoxConstraints, LifeCycleCtx, Size, LayoutCtx, Event, Env, UpdateCtx, KeyOrValue, FontDescriptor, Color, CurrentTimeData};
use druid::widget::{Container, Label};
use std::ops::Add;
use crate::calendar_data::{DEFAULT_DAY_WIDGET_SIZE, DEFAULT_GRID_SPACING, DAYS_OF_WEEK};
use crate::{Rect, FontFamily};
use std::sync::Arc;

pub struct CurrentTimeWidget {
    hour_label: WidgetPod<CurrentTimeData, Container<CurrentTimeData>>,
    minute_label: WidgetPod<CurrentTimeData, Container<CurrentTimeData>>,
    second_label: WidgetPod<CurrentTimeData, Container<CurrentTimeData>>,
}

impl CurrentTimeWidget {
    pub fn new() -> Self {
        let hour = Container::new(Label::dynamic(|c: &CurrentTimeData, _| c.current_hour_of_day.to_string()).with_font(FontDescriptor::new(FontFamily::SERIF).with_size(32.0)));
        let hour_label = WidgetPod::new(hour);

        let minute = Container::new(Label::dynamic(|c: &CurrentTimeData, _| c.current_minute_of_hour.to_string()));
        let minute_label = WidgetPod::new(minute);

        let second = Container::new(Label::dynamic(|c: &CurrentTimeData, _| c.current_second_of_hour.to_string()));
        let second_label = WidgetPod::new(second);

        CurrentTimeWidget {
            hour_label,
            minute_label,
            second_label,
        }
    }

    /*pub fn with_font(mut self, font: impl Into<KeyOrValue<FontDescriptor>>) -> Self {
        let with_new_font = self.current_time_label.with_font(font);
        CurrentTimeWidget {
            current_time_label: with_new_font
        }
    }

    pub fn with_text_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        let with_new_color = self.current_time_label.with_text_color(font);
        CurrentTimeWidget {
            current_time_label: with_new_color
        }
    }*/
}

impl Widget<CurrentTimeData> for CurrentTimeWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut CurrentTimeData, env: &Env) {
        //update here
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &CurrentTimeData, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.hour_label.lifecycle(ctx, event, &data, env);
                self.minute_label.lifecycle(ctx, event, &data, env);
                self.second_label.lifecycle(ctx, event, &data, env);
                println!("lifecycle - {:?}", data.current_second_of_hour);
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &CurrentTimeData, data: &CurrentTimeData, env: &Env) {
        println!("old {:?} new {:?}", old_data.current_second_of_hour, data.current_second_of_hour);
        if old_data.current_hour_of_day != data.current_hour_of_day {
            self.hour_label.update(ctx, &data, env);
            //ctx.request_paint();
        }
        if old_data.current_minute_of_hour != data.current_minute_of_hour {
            self.minute_label.update(ctx, &data, env);
            //ctx.request_paint();
        }
        if old_data.current_second_of_hour != data.current_second_of_hour {
            self.second_label.update(ctx, &data, env);
            //ctx.request_paint();
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &CurrentTimeData, env: &Env) -> Size {
        //println!("{:?}", data.current_second_of_hour);
        let mut x_position = DEFAULT_GRID_SPACING;
        let mut y_position = DEFAULT_GRID_SPACING;

        let hour_rect = Rect::new(
            x_position,
            y_position,
            x_position + DEFAULT_DAY_WIDGET_SIZE.width,
            y_position + DEFAULT_DAY_WIDGET_SIZE.height,
        );

        self.hour_label.layout(ctx, bc, &data, env);
        self.hour_label.set_layout_rect(ctx, &data, env, hour_rect);
        x_position += DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING;

        let min_rect = Rect::new(
            x_position,
            y_position,
            x_position + DEFAULT_DAY_WIDGET_SIZE.width,
            y_position + DEFAULT_DAY_WIDGET_SIZE.height,
        );

        self.minute_label.layout(ctx, bc, &data, env);
        self.minute_label.set_layout_rect(ctx, &data, env, min_rect);
        x_position += DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING;

        let sec_rect = Rect::new(
            x_position,
            y_position,
            x_position + DEFAULT_DAY_WIDGET_SIZE.width,
            y_position + DEFAULT_DAY_WIDGET_SIZE.height,
        );

        self.second_label.layout(ctx, bc, &data, env);
        self.second_label.set_layout_rect(ctx, &data, env, sec_rect);

        Size {
            width: (DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING) * DAYS_OF_WEEK.len() as f64,
            height: (DEFAULT_DAY_WIDGET_SIZE.height + DEFAULT_GRID_SPACING) * 2.,
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &CurrentTimeData, env: &Env) {
        println!("paint {:?}", data.current_second_of_hour);
        self.hour_label.paint(ctx, &data, env);
        self.minute_label.paint(ctx, &data, env);
        self.second_label.paint(ctx, &data, env);
        //self.sample_label.paint(ctx, &data, env);
    }
}