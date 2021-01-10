use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;

use chrono::{Datelike, DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use chrono::format::Numeric::Day;
use im::Vector;

use druid::{AppLauncher, BoxConstraints, CALENDAR_ID, CalendarData, Command, CurrentTimeData, Data, DateDetails, DateWidgetData, Env, Event, EventCtx, LayoutCtx, Lens, LensExt, LifeCycle, LifeCycleCtx, PaintCtx, Selector, SHOW_NEXT_CALENDAR, SHOW_PREVIOUS_CALENDAR, Size, Target, theme, UpdateCtx, Widget, WidgetExt, WidgetId, WidgetPod, WindowDesc};
use druid::widget::{Align, BackgroundBrush, Button, CalendarDateWidget, Checkbox, Container, CurrentTimeWidget, Flex, Label};
use druid_shell::kurbo::{Circle, Point};
use druid_shell::piet::{Color, LinearGradient, RenderContext, UnitPoint};
use druid_shell::piet::kurbo::Rect;

fn ui_builder() -> impl Widget<DateWidgetData> {
	let mut c1 = Flex::column();

	let current_time_widget = CurrentTimeWidget::new()
		.lens(DateWidgetData::current_time)
		.align_left();
	let current_day_label = Label::dynamic(|d: &(u32, u32, i32), env: &Env| {
		let d = NaiveDate::from_ymd(
			d.2,
			d.1,
			d.0,
		);
		//format!("{}", d.format("%A, %-d %B, %C%y"))
		d.format("%A, %-d %B, %C%y").to_string()
	})
		.lens(DateWidgetData::current_day_month_year)
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

	let next_month_button = Button::new("Next Month").on_click(move |ctx, data: &mut CalendarData, _| {
		// get current month and year from data
		let current_month = data.current_month_of_year;
		let current_year = data.current_year;
		// create a valid next month's naivedate from it
		let (y, m) = if current_month == 12 { (current_year + 1, 1) } else { (current_year, current_month + 1) };
		let next_month_naive_date = NaiveDate::from_ymd(y, m, 1);
		// Get dates for that month
		let next_month_calendar_data = CalendarData::new(next_month_naive_date.year(), next_month_naive_date.month());
		// change calendar data
		*data = next_month_calendar_data;
	})
		.lens(DateWidgetData::day_and_month);

	let previous_month_button = Button::new("Previous Month").on_click(move |ctx, data: &mut CalendarData, _| {
		// get current month and year from data
		let current_month = data.current_month_of_year;
		let current_year = data.current_year;
		let current_months_first_date = NaiveDate::from_ymd(current_year, current_month, 1);
		// Go to previous day ie last date of previous month
		let previous_months_last_date = current_months_first_date.pred();
		// Get dates for that month
		let previous_month_calendar_data = CalendarData::new(previous_months_last_date.year(), previous_months_last_date.month());
		// change calendar data
		*data = previous_month_calendar_data;
	})
		.lens(DateWidgetData::day_and_month);

	let mut calendar_flex = Flex::row()
		.with_child(calendar_widget.with_id(CALENDAR_ID))
		.with_child(next_month_button)
		.with_child(previous_month_button)
		.align_left();

	c1.add_child(calendar_flex);

	c1//.debug_paint_layout()
	//c1.debug_widget_id()
}

fn main() {
	let main_window = WindowDesc::new(ui_builder).title("Calendar");
	let today = Local::now();
	let calendar_data = CalendarData::new(today.year(), today.month());

	let current_time = CurrentTimeData {
		current_hour_of_day: today.hour(),
		current_minute_of_hour: today.minute(),
		current_second_of_minute: today.second(),
		twelve_hour_format: true,
	};

	// Set our initial data
	let data = DateWidgetData {
		day_and_month: calendar_data,
		current_day_month_year: (today.day(), today.month(), today.year()),
		current_time,
	};
	AppLauncher::with_window(main_window)
		.use_simple_logger()
		.launch(data)
		.expect("launch failed");
}
