use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::sync::Arc;
use std::time::SystemTime;

use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

use chrono::format::Numeric::Day;
use druid::widget::{Align, BackgroundBrush, CalendarDateWidget, Container, CurrentTimeWidget, Flex, Label, Checkbox};
use druid::{
    theme, AppLauncher, BoxConstraints, CalendarData, DateDetails, CurrentTimeData, Data,
    DateWidgetData, Env, Event, EventCtx, LayoutCtx, Lens, LensExt, LifeCycle, LifeCycleCtx,
    PaintCtx, Size, UpdateCtx, Widget, WidgetExt, WidgetPod, WindowDesc,
};
use druid_shell::kurbo::{Circle, Point};
use druid_shell::piet::kurbo::Rect;
use druid_shell::piet::{Color, LinearGradient, RenderContext, UnitPoint};
use im::Vector;
use std::str::FromStr;

fn ui_builder() -> impl Widget<DateWidgetData> {
    let mut c1 = Flex::column();

    let current_time_widget = CurrentTimeWidget::new()
        .lens(DateWidgetData::current_time)
        .align_left();
    let current_day_label = Label::dynamic(|d: &CalendarData, env: &Env| {
        let d = NaiveDate::from_ymd(
            d.current_year,
            d.current_month_of_year,
            d.current_day_of_month,
        );
        //format!("{}", d.format("%A, %-d %B, %C%y"))
        d.format("%A, %-d %B, %C%y").to_string()
    })
        .lens(DateWidgetData::day_and_month)
        .align_left();
    let time_format = Checkbox::new("12 hr format")
        .lens(DateWidgetData::current_time.then(CurrentTimeData::twelve_hour_format));

    let calendar_widget = CalendarDateWidget::new()
        .lens(DateWidgetData::day_and_month)
        .align_left();

    let mut time_flex = Flex::row()
        .with_child(current_time_widget).with_child(time_format)
        .align_left();
    c1.add_child(time_flex);
    c1.add_child(current_day_label);
    c1.add_spacer(10.);
    c1.add_child(calendar_widget);

    c1//.debug_paint_layout()
    //c1.debug_widget_id()
}

fn main() {
    let main_window = WindowDesc::new(ui_builder).title("Calendar");
    let today = Local::now();
    let first_date_of_current_month = NaiveDate::from_ymd(today.year(), today.month(), 1);
    let mut all_dates = Vector::new();

    let days_in_previous_month = get_last_n_days_of_previous_month(
        first_date_of_current_month.year(),
        first_date_of_current_month.month(),
        first_date_of_current_month.weekday().number_from_sunday(),
    );
    let days_in_previous_month_len = days_in_previous_month.len();
    for date in days_in_previous_month {
        let date_details = DateDetails {
            date,
            draw_border: false,
            grey_date: true,
            date_is_todays: false,
        };
        all_dates.push_back(date_details);
    }
    let days_in_current_month: Vector<u32> = (1..=get_number_of_days_in_a_month(
        first_date_of_current_month.year(),
        first_date_of_current_month.month(),
    ) as u32)
        .collect();
    let days_in_current_month_len = days_in_current_month.len();
    let todays_date = today.day();
    for date in days_in_current_month {
        let date_is_todays = if date == todays_date {
            true
        } else {
            false
        };

        let date_details = DateDetails {
            date,
            draw_border: date_is_todays,
            grey_date: false,
            date_is_todays,
        };
        all_dates.push_back(date_details);
    }
    let days_in_next_month: Vector<u32> =
        (1..=(35 - (days_in_current_month_len + days_in_previous_month_len) as u32)).collect();

    for date in days_in_next_month {
        let date_details = DateDetails {
            date,
            draw_border: false,
            grey_date: true,
            date_is_todays: false,
        };
        all_dates.push_back(date_details);
    }

    let day_and_month = CalendarData {
        current_day_of_month: today.day(),
        current_day_of_week: today.weekday().number_from_sunday(),
        current_month_of_year: today.month(),
        current_year: today.year(),
        all_dates,
        active_date_details_index: None,
        inactive_date_details_index: None,
    };

    let current_time = CurrentTimeData {
        current_hour_of_day: today.hour(),
        current_minute_of_hour: today.minute(),
        current_second_of_minute: today.second(),
        twelve_hour_format: true,
    };

    // Set our initial data
    let data = DateWidgetData {
        day_and_month,
        current_time,
    };
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

pub fn get_number_of_days_in_a_month(year: i32, month: u32) -> i64 {
    if month == 12 {
        NaiveDate::from_ymd(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd(year, month + 1, 1)
    }
        .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
        .num_days()
}

/*fn main() {
    let first_date_of_current_month = NaiveDate::from_ymd(2020, 12, 1);
    let days_in_previous_month = get_last_n_days_of_previous_month(first_date_of_current_month.year(), first_date_of_current_month.month(), first_date_of_current_month.weekday().number_from_sunday());
    let days_in_current_month: Vector<u32> = (1..=Calendar::get_number_of_days_in_a_month(first_date_of_current_month.year(), first_date_of_current_month.month()) as u32).collect();
    let days_in_next_month: Vector<u32> = (1..=(35 - (days_in_current_month.len() + days_in_previous_month.len()) as u32)).collect();

    println!("{:?}", days_in_previous_month.add(days_in_current_month).add(days_in_next_month));

    let first_date_of_current_month = NaiveDate::from_ymd(2020, 11, 1);
    let days_in_previous_month = get_last_n_days_of_previous_month(first_date_of_current_month.year(), first_date_of_current_month.month(), first_date_of_current_month.weekday().number_from_sunday());
    let days_in_current_month: Vector<u32> = (1..=Calendar::get_number_of_days_in_a_month(first_date_of_current_month.year(), first_date_of_current_month.month()) as u32).collect();
    let days_in_next_month: Vector<u32> = (1..=(35 - (days_in_current_month.len() + days_in_previous_month.len()) as u32)).collect();

    println!("{:?}", days_in_previous_month.add(days_in_current_month).add(days_in_next_month));

    let first_date_of_current_month = NaiveDate::from_ymd(2020, 10, 1);
    let days_in_previous_month = get_last_n_days_of_previous_month(first_date_of_current_month.year(), first_date_of_current_month.month(), first_date_of_current_month.weekday().number_from_sunday());
    let days_in_current_month: Vector<u32> = (1..=Calendar::get_number_of_days_in_a_month(first_date_of_current_month.year(), first_date_of_current_month.month()) as u32).collect();
    let days_in_next_month: Vector<u32> = (1..=(35 - (days_in_current_month.len() + days_in_previous_month.len()) as u32)).collect();

    println!("{:?}", days_in_previous_month.add(days_in_current_month).add(days_in_next_month));

    let first_date_of_current_month = NaiveDate::from_ymd(2020, 3, 1);
    let days_in_previous_month = get_last_n_days_of_previous_month(first_date_of_current_month.year(), first_date_of_current_month.month(), first_date_of_current_month.weekday().number_from_sunday());
    let days_in_current_month: Vector<u32> = (1..=Calendar::get_number_of_days_in_a_month(first_date_of_current_month.year(), first_date_of_current_month.month()) as u32).collect();
    let days_in_next_month: Vector<u32> = (1..=(35 - (days_in_current_month.len() + days_in_previous_month.len()) as u32)).collect();

    println!("{:?}", days_in_previous_month.add(days_in_current_month).add(days_in_next_month));

    let first_date_of_current_month = NaiveDate::from_ymd(2020, 2, 1);
    let days_in_previous_month = get_last_n_days_of_previous_month(first_date_of_current_month.year(), first_date_of_current_month.month(), first_date_of_current_month.weekday().number_from_sunday());
    let days_in_current_month: Vector<u32> = (1..=Calendar::get_number_of_days_in_a_month(first_date_of_current_month.year(), first_date_of_current_month.month()) as u32).collect();
    let days_in_next_month: Vector<u32> = (1..=(35 - (days_in_current_month.len() + days_in_previous_month.len()) as u32)).collect();

    println!("{:?}", days_in_previous_month.add(days_in_current_month).add(days_in_next_month));
}*/

fn get_last_n_days_of_previous_month(
    current_year: i32,
    current_month: u32,
    first_of_current_month_from_sunday: u32,
) -> Vector<u32> {
    let first_day_of_current_month = NaiveDate::from_ymd(current_year, current_month, 1);
    let last_date_of_previous_month = first_day_of_current_month.pred();
    let date_of_last_day_of_previous_month = last_date_of_previous_month.day();

    let last_n_days: Vector<u32> = (date_of_last_day_of_previous_month
        - first_of_current_month_from_sunday
        ..=date_of_last_day_of_previous_month)
        .skip(2)
        .collect();
    last_n_days
}
