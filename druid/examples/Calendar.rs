use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::sync::Arc;
use std::time::SystemTime;

use chrono::{Datelike, DateTime, Local, NaiveDate};

use druid::{AppLauncher, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle, LifeCycleCtx, PaintCtx, Size, theme, UpdateCtx, Widget, WidgetExt, WidgetPod, WindowDesc};
use druid::widget::{BackgroundBrush, Container, Flex, Label};
use druid_shell::kurbo::{Circle, Point};
use druid_shell::piet::{Color, LinearGradient, RenderContext, UnitPoint};
use druid_shell::piet::kurbo::Rect;
use chrono::format::Numeric::Day;

const DEFAULT_DAY_WIDGET_SIZE: Size = Size::new(25.0, 25.0);
const DEFAULT_GRID_SPACING: f64 = 5.0;

const DAYS_OF_WEEK: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// Date widget consisting of all the data I would possibly need.
#[derive(Clone, Data, Lens)]
struct DateWidgetData {
    day_and_month: CalendarData,
    /// 00..23
    current_hour_of_day: u32,
    /// 00..59
    current_minute_of_hour: u32,
    /// 00..59
    current_second_of_hour: u32,
    // key like mmddyy -> list of today's task
    //current_day_tasks: Arc<HashMap<String, Vec<Label<String>>>>,
}

// do I need this?
#[derive(Clone, Data)]
struct CalendarData {
    /// 1..28/29/30/31
    current_day_of_month: u32,
    // this will be used to highlight current day.
    /// 1..12
    current_month_of_year: u32,
    // this will be used to show all days in current month.
    /// 2020, 2021, etc.
    current_year: i32,
}

struct Calendar {
    days_widget: Vec<WidgetPod<String, Container<String>>>,
    //su, mo, tu, etc.
    // date of month cannot be a const. it changes per month
    dates_of_month_widget: Vec<WidgetPod<String, Container<String>>>, // this will be used to highlight.
}

impl Calendar {
    fn new() -> Calendar {
        Calendar {
            days_widget: Calendar::get_days_of_week(),
            dates_of_month_widget: Vec::new(),
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

    fn get_dates_of_month(current_year: i32, current_month_of_year: u32) -> Vec<WidgetPod<String, Container<String>>> {
        let days_in_a_month = Calendar::get_number_of_days_in_a_month(current_year, current_month_of_year);
        let mut date_of_month = Vec::with_capacity(days_in_a_month as usize);

        for current_date in 1..days_in_a_month.add(1) {
            let date_widget = Container::new(Label::new(current_date.to_string()));
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
        for (day, mut day_widget) in DAYS_OF_WEEK.iter().zip(self.days_widget.iter_mut()) {
            day_widget.event(ctx, event, &mut String::from(*day), env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &CalendarData, env: &Env) {
        println!("lifecycle {:?}", event);

        match event {
            LifeCycle::WidgetAdded => {
                for (day, mut day_widget) in DAYS_OF_WEEK.iter().zip(self.days_widget.iter_mut()) {
                    day_widget.lifecycle(ctx, event, &String::from(*day), env);
                }

                let mut dates_of_month_widget = Calendar::get_dates_of_month(data.current_year, data.current_month_of_year);
                for (i, mut date_widget) in dates_of_month_widget.drain(0..dates_of_month_widget.len()).enumerate() {
                    let date = i + 1;
                    date_widget.lifecycle(ctx, event, &date.to_string(), env);
                    self.dates_of_month_widget.push(date_widget);
                }
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &CalendarData, data: &CalendarData, env: &Env) {
        //println!("update");
        for (day, mut day_widget) in DAYS_OF_WEEK.iter().zip(self.days_widget.iter_mut()) {
            day_widget.update(ctx, &String::from(*day), env);
        }
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

        let first_date_of_current_month = NaiveDate::from_ymd(data.current_year, data.current_month_of_year, 1);
        let mut day_from_sunday = first_date_of_current_month.weekday().num_days_from_sunday();
        y_position += DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING;
        x_position = DEFAULT_GRID_SPACING + (DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING) * day_from_sunday as f64;

        let mut date: i32 = 1;

        for date_widget in self.dates_of_month_widget.iter_mut() {
            if day_from_sunday == 7 {
                day_from_sunday = 0;
                y_position += DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING;
                x_position = DEFAULT_GRID_SPACING;
            }
            let rect = Rect::new(
                x_position,
                y_position,
                x_position + DEFAULT_DAY_WIDGET_SIZE.width,
                y_position + DEFAULT_DAY_WIDGET_SIZE.height,
            );
            date_widget.layout(ctx, &BoxConstraints::new(DEFAULT_DAY_WIDGET_SIZE, DEFAULT_DAY_WIDGET_SIZE), &date.to_string(), env);
            date_widget.set_layout_rect(ctx, &date.to_string(), env, rect);
            x_position += DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING;
            day_from_sunday = day_from_sunday.checked_add(1).unwrap();
            date = date.checked_add(1).unwrap();
        }

        Size {
            width: (DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING) * DAYS_OF_WEEK.len() as f64,
            height:  (DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING) * 5.,
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &CalendarData, env: &Env) {
        //println!("paint");
        for (day, mut day_widget) in DAYS_OF_WEEK.iter().zip(self.days_widget.iter_mut()) {
            day_widget.paint(ctx, &String::from(*day), env);
        }

        for (i, date_widget) in self.dates_of_month_widget.iter_mut().enumerate() {
            let date = i + 1;
            date_widget.paint(ctx, &date.to_string(), env);
        }
    }
}

fn ui_builder() -> impl Widget<DateWidgetData> {
    Calendar::new().lens((DateWidgetData::day_and_month))
}

fn main() {
    let main_window = WindowDesc::new(ui_builder).title("Calendar");
    let today = Local::now();

    let day_and_month = CalendarData {
        current_day_of_month: today.day(),
        current_month_of_year: today.month(),
        current_year: today.year(),
    };
    // Set our initial data
    let data = DateWidgetData {
        day_and_month,
        current_hour_of_day: 0,
        current_minute_of_hour: 0,
        current_second_of_hour: 0,
        //: Arc::new(Default::default())
    };
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}