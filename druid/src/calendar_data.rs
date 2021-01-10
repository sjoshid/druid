use chrono::{Weekday, NaiveDate, Datelike, Local};
use im::Vector;

use druid::{Data, Lens};

use crate::{Selector, Size, WidgetId};

pub const DEFAULT_DAY_WIDGET_SIZE: Size = Size::new(47.0, 47.0);
pub const DEFAULT_GRID_SPACING: f64 = 5.0;

pub const DAYS_OF_WEEK: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

pub const CALENDAR_ID: WidgetId = WidgetId::reserved(0);
pub const SHOW_NEXT_CALENDAR: Selector<CalendarData> =
	Selector::new("calendar.show.next.month");
pub const SHOW_PREVIOUS_CALENDAR: Selector<u32> =
	Selector::new("calendar.show.previous.month");

/// Date widget consisting of all the data I would possibly need.
#[derive(Clone, Data, Lens)]
pub struct DateWidgetData {
	pub day_and_month: CalendarData,
	pub current_day_month_year: (u32, u32, i32),
	pub current_time: CurrentTimeData,
	// key like mmddyy -> list of today's task
	//current_day_tasks: Arc<HashMap<String, Vec<Label<String>>>>,
}

#[derive(Clone, Data, Lens, Debug)]
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
	//pub current_day_of_month: u32,
	// this will be used to highlight current day.
	/// 1..12
	pub current_month_of_year: u32,
	// this will be used to show all days in current month.
	/// 2020, 2021, etc.
	pub current_year: i32,
	//pub current_mont_data: CurrentMonthData,
	pub all_dates: Vector<DateDetails>,
	pub active_date_details_index: Option<usize>,
	pub inactive_date_details_index: Option<usize>,
}

impl CalendarData {
	pub fn new(year: i32, month: u32) -> CalendarData {
		let all_dates = CalendarData::get_all_dates_for_given_month_and_year(year, month);
		let active_date_details_index = CalendarData::get_active_date_details_index(&all_dates, year, month);

		CalendarData {
			current_month_of_year: month,
			current_year: year,
			all_dates,
			active_date_details_index,
			inactive_date_details_index: None,
		}
	}

	pub fn get_calendar_data_for_next_month(&mut self) {
		// get current month and year from data
		let current_month = self.current_month_of_year;
		let current_year = self.current_year;
		// create a valid next month's naivedate from it
		let (y, m) = if current_month == 12 { (current_year + 1, 1) } else { (current_year, current_month + 1) };
		let next_month_naive_date = NaiveDate::from_ymd(y, m, 1);
		let cdata = CalendarData::new(next_month_naive_date.year(), next_month_naive_date.month());

		self.all_dates = cdata.all_dates;
		self.active_date_details_index = cdata.active_date_details_index;
		self.inactive_date_details_index = None;
		self.current_month_of_year = cdata.current_month_of_year;
		self.current_year = cdata.current_year;
	}

	pub fn get_calendar_data_for_previous_month(&mut self) {
		let current_month = self.current_month_of_year;
		let current_year = self.current_year;
		let current_months_first_date = NaiveDate::from_ymd(current_year, current_month, 1);
		// Go to previous day ie last date of previous month
		let previous_months_last_date = current_months_first_date.pred();
		let cdata = CalendarData::new(previous_months_last_date.year(), previous_months_last_date.month());

		self.all_dates = cdata.all_dates;
		self.active_date_details_index = cdata.active_date_details_index;
		self.inactive_date_details_index = None;
		self.current_month_of_year = cdata.current_month_of_year;
		self.current_year = cdata.current_year;
	}

	fn get_active_date_details_index(all_dates: &Vector<DateDetails>, year: i32, month: u32) -> Option<usize> {
		let today = Local::today();

		if today.year() == year && today.month() == month {
			let mut first_date_found = false;

			let todays_naive_date = NaiveDate::from_ymd(today.year(), today.month(), today.day());
			for (i, date_details) in all_dates.iter().enumerate() {
				if date_details.date == 1 {
					first_date_found = true;
				}

				if first_date_found {
					let current_naive_date = NaiveDate::from_ymd(year, month, date_details.date);

					if todays_naive_date == current_naive_date {
						return Some(i);
					}
				}
			}
			return None;
		} else {
			return None;
		}
	}

	fn get_all_dates_for_given_month_and_year(year: i32, month: u32) -> Vector<DateDetails> {
		println!("year {}, month {}", year, month);
		let first_date_of_current_month = NaiveDate::from_ymd(year, month, 1);
		let mut all_dates = Vector::new();

		let days_in_previous_month = CalendarData::get_last_n_days_of_previous_month(
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
		let days_in_current_month: Vector<u32> = (1..=CalendarData::get_number_of_days_in_a_month(
			first_date_of_current_month.year(),
			first_date_of_current_month.month(),
		) as u32)
			.collect();
		let days_in_current_month_len = days_in_current_month.len();
		let today = Local::now();
		let todays_naive_date = NaiveDate::from_ymd(today.year(), today.month(), today.day());

		for (i, date) in days_in_current_month.into_iter().enumerate() {
			let current_naive_date = NaiveDate::from_ymd(year, month, date);
			let date_is_todays = if todays_naive_date == current_naive_date {
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
			(1..=(42 - (days_in_current_month_len + days_in_previous_month_len) as u32)).collect();

		for date in days_in_next_month {
			let date_details = DateDetails {
				date,
				draw_border: false,
				grey_date: true,
				date_is_todays: false,
			};
			all_dates.push_back(date_details);
		}

		println!("all dates {:?}", all_dates);
		all_dates
	}

	fn get_number_of_days_in_a_month(year: i32, month: u32) -> i64 {
		if month == 12 {
			NaiveDate::from_ymd(year + 1, 1, 1)
		} else {
			NaiveDate::from_ymd(year, month + 1, 1)
		}
			.signed_duration_since(NaiveDate::from_ymd(year, month, 1))
			.num_days()
	}

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
}

#[derive(Clone, Data, Lens, Debug)]
pub struct DateDetails {
	pub date: u32,
	pub draw_border: bool,
	pub grey_date: bool,
	pub date_is_todays: bool,
}
