use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::sync::Arc;
use std::time::SystemTime;

use chrono::{Datelike, DateTime, Local, NaiveDate, Timelike};

use druid::{AppLauncher, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle, LifeCycleCtx, PaintCtx, Size, theme, UpdateCtx, Widget, WidgetExt, WidgetPod, WindowDesc, DateWidgetData, CalendarData, CurrentTimeData};
use druid::widget::{BackgroundBrush, Container, Flex, Label, Calendar, CurrentTimeWidget};
use druid_shell::kurbo::{Circle, Point};
use druid_shell::piet::{Color, LinearGradient, RenderContext, UnitPoint};
use druid_shell::piet::kurbo::Rect;
use chrono::format::Numeric::Day;

fn ui_builder() -> impl Widget<DateWidgetData> {
    let mut c1 = Flex::column();

    let current_time_widget = CurrentTimeWidget::new().lens(DateWidgetData::current_time);

    let calendar_widget = Calendar::new().lens(DateWidgetData::day_and_month);

    c1.add_child(current_time_widget);
    c1.add_child(calendar_widget);

    c1.debug_paint_layout()
}

fn main() {
    let main_window = WindowDesc::new(ui_builder).title("Calendar");
    let today = Local::now();

    let day_and_month = CalendarData {
        current_day_of_month: today.day(),
        current_month_of_year: today.month(),
        current_year: today.year(),
    };

    let current_time = CurrentTimeData {
        current_hour_of_day: today.hour(),
        current_minute_of_hour: today.minute(),
        current_second_of_hour: today.second(),
        twelve_hour_format: true,
        time_separator: ':',
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