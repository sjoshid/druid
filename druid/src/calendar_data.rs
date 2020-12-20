use druid::{Data, Lens};
use crate::Size;
use chrono::Weekday;
use im::Vector;

pub const DEFAULT_DAY_WIDGET_SIZE: Size = Size::new(35.0, 35.0);
pub const DEFAULT_GRID_SPACING: f64 = 5.0;

pub const DAYS_OF_WEEK: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// Date widget consisting of all the data I would possibly need.
#[derive(Clone, Data, Lens)]
pub struct DateWidgetData {
    pub day_and_month: CalendarData,
    pub current_time: CurrentTimeData,
    // key like mmddyy -> list of today's task
    //current_day_tasks: Arc<HashMap<String, Vec<Label<String>>>>,
}

#[derive(Clone, Data)]
pub struct CurrentTimeData {
    /// 00..23
    pub current_hour_of_day: u32,
    /// 00..59
    pub current_minute_of_hour: u32,
    /// 00..59
    pub current_second_of_minute: u32,
    //12 or 24 hr format
    pub twelve_hour_format: bool,
}

// do I need this?
#[derive(Clone, Data, Lens)]
pub struct CalendarData {
    /// 1..28/29/30/31
    pub current_day_of_month: u32,
    pub current_day_of_week: u32,
    // this will be used to highlight current day.
    /// 1..12
    pub current_month_of_year: u32,
    // this will be used to show all days in current month.
    /// 2020, 2021, etc.
    pub current_year: i32,
    //pub current_mont_data: CurrentMonthData,
    pub all_dates: Vector<u32>,
}

pub struct CurrentMonthData {
    pub days_of_month: Vector<u32>,
    pub index_of_first_day: u32,
    pub index_of_last_day: u32,
}