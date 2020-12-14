use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::sync::Arc;
use std::time::SystemTime;

use chrono::{Datelike, DateTime, Local, NaiveDate, Timelike};

use druid::{AppLauncher, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle, LifeCycleCtx, PaintCtx, Size, theme, UpdateCtx, Widget, WidgetExt, WidgetPod, WindowDesc, DateWidgetData, CalendarData, CurrentTimeData, LensExt};
use druid::widget::{BackgroundBrush, Container, Flex, Label, Calendar, CurrentTimeWidget, Align};
use druid_shell::kurbo::{Circle, Point};
use druid_shell::piet::{Color, LinearGradient, RenderContext, UnitPoint};
use druid_shell::piet::kurbo::Rect;
use chrono::format::Numeric::Day;

fn ui_builder() -> impl Widget<DateWidgetData> {
    let mut c1 = Flex::column();

    let current_time_widget = CurrentTimeWidget::new().lens(DateWidgetData::current_time).align_left();
    let current_day_label = Label::dynamic(|d: &CalendarData, env: &Env| {
        let d = NaiveDate::from_ymd(d.current_year, d.current_month_of_year, d.current_day_of_month);
        //format!("{}", d.format("%A, %-d %B, %C%y"))
        d.format("%A, %-d %B, %C%y").to_string()
    }).lens(DateWidgetData::day_and_month).align_left();
    let calendar_widget = Calendar::new().lens(DateWidgetData::day_and_month).align_left();

    c1.add_child(current_time_widget);
    c1.add_child(current_day_label);
    c1.add_spacer(10.);
    c1.add_child(calendar_widget);

    c1//.debug_paint_layout()
}

fn main() {
    let main_window = WindowDesc::new(ui_builder).title("Calendar");
    let today = Local::now();

    let day_and_month = CalendarData {
        current_day_of_month: today.day(),
        current_day_of_week: today.weekday().number_from_sunday(),
        current_month_of_year: today.month(),
        current_year: today.year(),
    };

    let current_time = CurrentTimeData {
        current_hour_of_day: today.hour(),
        current_minute_of_hour: today.minute(),
        current_second_of_minute: today.second(),
        twelve_hour_format: false,
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