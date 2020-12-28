use crate::calendar_data::{DAYS_OF_WEEK, DEFAULT_DAY_WIDGET_SIZE, DEFAULT_GRID_SPACING};
use crate::widget::{BackgroundBrush, Container, Label, Painter};
use crate::{theme, BoxConstraints, CalendarData, Env, Event, EventCtx, LayoutCtx, LensExt, LifeCycle, LifeCycleCtx, PaintCtx, Rect, RenderContext, Size, UpdateCtx, Widget, WidgetExt, WidgetPod, MouseButton, DateDetails};
use chrono::{Datelike, NaiveDate, Date};
use druid_shell::piet::Color;
use std::ops::Add;


pub struct CalendarDateWidget {
	days_widget: Vec<WidgetPod<String, Container<String>>>,
	//su, mo, tu, etc.
	// date of month cannot be a const. it changes per month
	dates_of_month_widget: Vec<WidgetPod<DateDetails, CustomDateWrapper>>, // this will be used to highlight.
}

impl CalendarDateWidget {
	pub fn new() -> CalendarDateWidget {
		CalendarDateWidget {
			days_widget: Vec::with_capacity(7),
			dates_of_month_widget: Vec::with_capacity(35),
		}
	}

	fn get_days_of_week() -> Vec<WidgetPod<String, Container<String>>> {
		let mut days_widgets = Vec::with_capacity(7);

		for (i, day) in DAYS_OF_WEEK.iter().enumerate() {
			let day = Container::new(Label::new(String::from(*day)).center());
			days_widgets.push(WidgetPod::new(day));
		}

		days_widgets
	}

	fn current_date_painter() -> Painter<DateDetails> {
		let painter = Painter::new(|ctx, _, env| {
			let bounds = ctx.size().to_rect();

			ctx.fill(bounds, &env.get(theme::PRIMARY_DARK));
		});
		painter
	}

	fn outer_date_painter() -> Painter<CalendarData> {
		let painter = Painter::new(|ctx, _, env| {
			let bounds = ctx.size().to_rect();
			//ctx.fill(bounds, &env.get(theme::PRIMARY_DARK));

			/*if ctx.is_hot() {
				//ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
				println!("outer date hot");
			}*/

			if ctx.is_active() {
				println!("outer date active");
				ctx.fill(bounds, &env.get(theme::PRIMARY_LIGHT));
			}
		});
		painter
	}
}

/// Wraps a Label in a Container.
/// I chose Container because it takes a &mut that adds a border. Not sure if this is the right choice.
struct CustomDateWrapper {
	label_in_container: Container<String>,
	draw_border: bool,
	grey_date: bool,
}

impl CustomDateWrapper {
	/*pub fn new() -> Self {
		CustomDateWrapper {
			label_in_container: Container::new(Label::new(String::from(""))),
		}
	}*/

	/*pub fn new(label: Container<String>) -> Self {
		CustomDateWrapper {
			label_in_container: label,
		}
	}*/

	pub fn new(date_details: DateDetails) -> Self {
		CustomDateWrapper {
			//label_in_container: Container::new(Label::dynamic(|date: &u32, _| date.to_string()).lens(DateDetails::date)),
			label_in_container: Container::new(Label::new(date_details.date.to_string())),
			draw_border: date_details.draw_border,
			grey_date: date_details.grey_date,
		}
	}
}

impl Widget<DateDetails> for CustomDateWrapper {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DateDetails, env: &Env) {
		match event {
			Event::MouseDown(mouse_event) => {
				if mouse_event.button == MouseButton::Left {
					ctx.set_active(true);
				}
				//ctx.request_paint();
			}
			Event::MouseUp(mouse_event) => {
				if ctx.is_active() && mouse_event.button == MouseButton::Left {
					ctx.set_active(false);
				}
				//ctx.request_paint();
			}
			_ => {}
		}
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &DateDetails, env: &Env) {
		match event {
			LifeCycle::WidgetAdded => {
				/*let dynamic_date =
					Label::dynamic(|date_details: &DateDetails, _| date_details.date.to_string())
						.center()
						//.background(CalendarDateWidget::current_date_painter())
						//.on_click(|ctx, data: &mut u32, _env| println!("clicked"))
						.lens(CalendarData::all_dates.index(0));
				self.label_in_container = Container::new(dynamic_date.padding(3.));*/
				self.label_in_container.lifecycle(ctx, event, &data.date.to_string(), env);
			}
			_ => {}
		}
	}

	fn update(&mut self, ctx: &mut UpdateCtx, old_data: &DateDetails, data: &DateDetails, env: &Env) {
		self.label_in_container.update(ctx, &old_data.date.to_string(), &data.date.to_string(), env);
	}

	fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &DateDetails, env: &Env) -> Size {
		self.label_in_container.layout(ctx, bc, &data.date.to_string(), env)
	}

	fn paint(&mut self, ctx: &mut PaintCtx, data: &DateDetails, env: &Env) {
		self.label_in_container.paint(ctx, &data.date.to_string(), env);
		let border = data.draw_border;
		if border {
			//println!("draw border for date {:?}", data.date);
			self.label_in_container.set_border(Color::WHITE, 1.);

		} else {
			//println!("remove border for date {:?}", data.date);
			self.label_in_container.set_border(theme::BACKGROUND_LIGHT, 1.);
		}
	}
}

impl Widget<CalendarData> for CalendarDateWidget {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut CalendarData, env: &Env) {
		match event {
			Event::MouseDown(mouse_event) => {
				for (i, dynamic_date) in self.dates_of_month_widget.iter_mut().enumerate() {
					dynamic_date.event(ctx, event, &mut data.all_dates[i], env);
					if dynamic_date.has_active() { // we cant have date_widget.is_active()
						if data.active_date_details_index.is_some() {
							data.inactive_date_details_index = data.active_date_details_index;
						}
						data.active_date_details_index = Some(i);
					}
				}
				if data.active_date_details_index.is_some() {
					let mut active_date = &mut data.all_dates[data.active_date_details_index.unwrap()];
					active_date.draw_border = true;
				}
				if data.inactive_date_details_index.is_some() {
					let mut inactive_date = &mut data.all_dates[data.inactive_date_details_index.unwrap()];
					inactive_date.draw_border = false;
				}
				ctx.request_paint();
			}
			Event::MouseUp(mouse_event) => {
				for (i, dynamic_date) in self.dates_of_month_widget.iter_mut().enumerate() {
					dynamic_date.event(ctx, event, &mut data.all_dates[i], env);
				}
				ctx.request_paint();
			}
			_ => {}
		}
	}

	fn lifecycle(
		&mut self,
		ctx: &mut LifeCycleCtx,
		event: &LifeCycle,
		data: &CalendarData,
		env: &Env,
	) {
		match event {
			LifeCycle::WidgetAdded => {
				self.days_widget = CalendarDateWidget::get_days_of_week();
				for (day, mut day_widget) in DAYS_OF_WEEK.iter().zip(self.days_widget.iter_mut()) {
					day_widget.lifecycle(ctx, event, &String::from(*day), env);
				}

				let mut is_current_month = false;
				for (i, d) in data.all_dates.iter().enumerate() {
					if d.date == 1 {
						is_current_month = true;
					}

					let mut date_widget = WidgetPod::new(CustomDateWrapper::new(d.clone()));
					self.dates_of_month_widget.push(date_widget);
				}

				for (i, dynamic_date) in self.dates_of_month_widget.iter_mut().enumerate() {
					dynamic_date.lifecycle(ctx, event, &data.all_dates[i], env);
				}
			}
			_ => {}
		}
	}

	fn update(
		&mut self,
		ctx: &mut UpdateCtx,
		old_data: &CalendarData,
		data: &CalendarData,
		env: &Env,
	) {
		for (i, dynamic_date) in self.dates_of_month_widget.iter_mut().enumerate() {
			dynamic_date.update(ctx, &data.all_dates[i], env);
		}
	}

	fn layout(
		&mut self,
		ctx: &mut LayoutCtx,
		bc: &BoxConstraints,
		data: &CalendarData,
		env: &Env,
	) -> Size {
		let mut x_position = DEFAULT_GRID_SPACING;
		let mut y_position = DEFAULT_GRID_SPACING;

		for (day, mut day_widget) in DAYS_OF_WEEK.iter().zip(self.days_widget.iter_mut()) {
			let rect = Rect::new(
				x_position,
				y_position,
				x_position + DEFAULT_DAY_WIDGET_SIZE.width,
				y_position + DEFAULT_DAY_WIDGET_SIZE.height,
			);
			day_widget.layout(
				ctx,
				&BoxConstraints::new(DEFAULT_DAY_WIDGET_SIZE, DEFAULT_DAY_WIDGET_SIZE),
				&String::from(*day),
				env,
			);
			day_widget.set_layout_rect(ctx, &String::from(*day), env, rect);
			x_position += DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING;
		}

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
			date_widget.layout(
				ctx,
				&BoxConstraints::new(DEFAULT_DAY_WIDGET_SIZE, DEFAULT_DAY_WIDGET_SIZE),
				&data.all_dates[i],
				env,
			);
			date_widget.set_layout_rect(ctx, &data.all_dates[i], env, rect);
			x_position += DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING;
		}

		Size {
			width: (DEFAULT_DAY_WIDGET_SIZE.width + DEFAULT_GRID_SPACING)
				* DAYS_OF_WEEK.len() as f64,
			height: (DEFAULT_DAY_WIDGET_SIZE.height + DEFAULT_GRID_SPACING) * 6.,
		}
	}

	fn paint(&mut self, ctx: &mut PaintCtx, data: &CalendarData, env: &Env) {
		for (day, mut day_widget) in DAYS_OF_WEEK.iter().zip(self.days_widget.iter_mut()) {
			day_widget.paint(ctx, &String::from(*day), env);
		}
		println!("outer paint");
		for (i, dynamic_date) in self.dates_of_month_widget.iter_mut().enumerate() {
			dynamic_date.paint(ctx, &data.all_dates[i], env);
		}
	}
}
