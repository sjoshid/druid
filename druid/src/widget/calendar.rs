use crate::{WidgetPod, Size, LifeCycleCtx, EventCtx, Widget, Event, Env, LifeCycle, UpdateCtx, BoxConstraints, LayoutCtx, Rect, PaintCtx, CalendarData, LensExt, WidgetExt};
use crate::widget::{Container, Label, BackgroundBrush};
use druid_shell::piet::Color;
use chrono::{NaiveDate, Datelike};
use std::ops::Add;
use crate::calendar_data::{DAYS_OF_WEEK, DEFAULT_DAY_WIDGET_SIZE, DEFAULT_GRID_SPACING};

pub struct Calendar {
    days_widget: Vec<WidgetPod<String, Container<String>>>,
    //su, mo, tu, etc.
    // date of month cannot be a const. it changes per month
    dates_of_month_widget: Vec<WidgetPod<CalendarData, Container<CalendarData>>>, // this will be used to highlight.
}

impl Calendar {
    pub fn new() -> Calendar {
        Calendar {
            days_widget: Calendar::get_days_of_week(),
            dates_of_month_widget: Calendar::get_dates_of_month_widgets(),
        }
    }

    fn get_days_of_week() -> Vec<WidgetPod<String, Container<String>>> {
        let mut days_widgets = Vec::with_capacity(7);

        for (i, day) in DAYS_OF_WEEK.iter().enumerate() {
            let day = Container::new(Label::new(String::from(*day)));
            let day = day.background(BackgroundBrush::Color(Color::rgb8(i as u8, i as u8, 55)));
            let day = day.border(Color::WHITE, 1.0);
            days_widgets.push(WidgetPod::new(day));
        }

        days_widgets
    }

    fn get_dates_of_month_widgets() -> Vec<WidgetPod<CalendarData, Container<CalendarData>>> {
        let mut date_of_month = Vec::with_capacity(35);

        for current_date in 0..35 {
            let dynamic_date = Label::dynamic(|date_of_month: &u32, _| {
                date_of_month.to_string()
            }).lens(CalendarData::all_dates.index(current_date));
            let date_widget = Container::new(dynamic_date);
            let date_widget = date_widget.border(Color::WHITE, 1.0);
            let date_widget = WidgetPod::new(date_widget);

            date_of_month.push(date_widget);
        }

        date_of_month
    }

    pub fn get_number_of_days_in_a_month(year: i32, month: u32) -> i64 {
        if month == 12 {
            NaiveDate::from_ymd(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd(year, month + 1, 1)
        }.signed_duration_since(NaiveDate::from_ymd(year, month, 1))
            .num_days()
    }
}

impl Widget<CalendarData> for Calendar {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut CalendarData, env: &Env) {
        //println!("event");
        // day_widgets will never be updated. so ignore calling event on them
        /*for (day, mut day_widget) in DAYS_OF_WEEK.iter().zip(self.days_widget.iter_mut()) {
            day_widget.event(ctx, event, &mut String::from(*day), env);
        }*/
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &CalendarData, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                for (day, mut day_widget) in DAYS_OF_WEEK.iter().zip(self.days_widget.iter_mut()) {
                    day_widget.lifecycle(ctx, event, &String::from(*day), env);
                }

                println!("{:?}", data.all_dates);
                for dynamic_date in self.dates_of_month_widget.iter_mut() {
                    dynamic_date.lifecycle(ctx, event, &data, env);
                }

                /*let mut dates_of_month_widget = Calendar::get_dates_of_month(data.current_year, data.current_month_of_year);
                for (i, mut date_widget) in dates_of_month_widget.drain(0..dates_of_month_widget.len()).enumerate() {
                    let date = i + 1;
                    date_widget.lifecycle(ctx, event, &date.to_string(), env);
                    self.dates_of_month_widget.push(date_widget);
                }*/
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &CalendarData, data: &CalendarData, env: &Env) {
        //println!("update");
        // day_widgets will never be updated.
        /*for (day, mut day_widget) in DAYS_OF_WEEK.iter().zip(self.days_widget.iter_mut()) {
            day_widget.update(ctx, &String::from(*day), env);
        }*/
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &CalendarData, env: &Env) -> Size {
        let mut x_position = DEFAULT_GRID_SPACING;
        let mut y_position = DEFAULT_GRID_SPACING;

        for (day, mut day_widget) in DAYS_OF_WEEK.iter().zip(self.days_widget.iter_mut()) {
            let rect = Rect::new(
                x_position,
                y_position,
                x_position + DEFAULT_DAY_WIDGET_SIZE.width,
                y_position + DEFAULT_DAY_WIDGET_SIZE.height,
            );
            day_widget.layout(ctx, &BoxConstraints::new(DEFAULT_DAY_WIDGET_SIZE, DEFAULT_DAY_WIDGET_SIZE), &String::from(*day), env);
            day_widget.set_layout_rect(ctx, &String::from(*day), env, rect);
            x_position += DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING;
        }

        /*let first_date_of_current_month = NaiveDate::from_ymd(data.current_year, data.current_month_of_year, 1);
        let mut day_from_sunday = first_date_of_current_month.weekday().num_days_from_sunday();
        y_position += DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING;
        x_position = DEFAULT_GRID_SPACING + (DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING) * day_from_sunday as f64;*/

        let mut date: i32 = 1;

        for (i, date_widget) in self.dates_of_month_widget.iter_mut().enumerate() {
            if i % 7 == 0 {
                y_position += DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING;
                x_position = DEFAULT_GRID_SPACING;
            }
            let rect = Rect::new(
                x_position,
                y_position,
                x_position + DEFAULT_DAY_WIDGET_SIZE.width,
                y_position + DEFAULT_DAY_WIDGET_SIZE.height,
            );
            date_widget.layout(ctx, &BoxConstraints::new(DEFAULT_DAY_WIDGET_SIZE, DEFAULT_DAY_WIDGET_SIZE), &data, env);
            date_widget.set_layout_rect(ctx, &data, env, rect);
            x_position += DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING;
            /*day_from_sunday = day_from_sunday.checked_add(1).unwrap();
            date = date.checked_add(1).unwrap();*/
        }

        Size {
            width: (DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING) * DAYS_OF_WEEK.len() as f64,
            height:  (DEFAULT_DAY_WIDGET_SIZE.height + DEFAULT_GRID_SPACING) * 6.,
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &CalendarData, env: &Env) {
        //println!("paint");
        for (day, mut day_widget) in DAYS_OF_WEEK.iter().zip(self.days_widget.iter_mut()) {
            day_widget.paint(ctx, &String::from(*day), env);
        }

        for date_widget in self.dates_of_month_widget.iter_mut() {
            date_widget.paint(ctx, &data, env);
        }
    }
}