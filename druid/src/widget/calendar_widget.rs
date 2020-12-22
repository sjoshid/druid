use crate::calendar_data::{DAYS_OF_WEEK, DEFAULT_DAY_WIDGET_SIZE, DEFAULT_GRID_SPACING};
use crate::widget::{BackgroundBrush, Container, Label, Painter};
use crate::{theme, BoxConstraints, CalendarData, Env, Event, EventCtx, LayoutCtx, LensExt, LifeCycle, LifeCycleCtx, PaintCtx, Rect, RenderContext, Size, UpdateCtx, Widget, WidgetExt, WidgetPod, MouseButton};
use chrono::{Datelike, NaiveDate};
use druid_shell::piet::Color;
use std::ops::Add;

/// Wraps a Label in a Container.
/// I chose Container because it takes a &mut that adds a border. Not sure if this is the right choice.
struct CustomLabelWrapper {
	label_in_container: Container<String>,
}

impl CustomLabelWrapper {
	pub fn new(text: String) -> Self {
		CustomLabelWrapper {
			label_in_container: Container::new(Label::new(text)),
		}
	}
}

impl Widget<(String, bool)> for CustomLabelWrapper {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut (String, bool), env: &Env) {
		match event {
			Event::MouseDown(mouse_event) => {
				if mouse_event.button == MouseButton::Left {
					ctx.set_active(true);
				}
			}
			Event::MouseUp(mouse_event) => {
				if ctx.is_active() && mouse_event.button == MouseButton::Left {
					ctx.set_active(false);
				}
			}
			_ => {}
		}
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &(String, bool), env: &Env) {
		match event {
			LifeCycle::WidgetAdded => {
				self.label_in_container.lifecycle(ctx, event, &(*data).0, env);
			}
			_ => {}
		}
	}

	fn update(&mut self, ctx: &mut UpdateCtx, old_data: &(String, bool), data: &(String, bool), env: &Env) {
		self.label_in_container.update(ctx, &(*old_data).0, &(*data).0, env);
	}

	fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &(String, bool), env: &Env) -> Size {
		self.label_in_container.layout(ctx, bc, &(*data).0, env)
	}

	fn paint(&mut self, ctx: &mut PaintCtx, data: &(String, bool), env: &Env) {
		let border = data.1;
		println!("inner paint ");
		self.label_in_container.paint(ctx, &(*data).0, env);
		if border {
			println!("i {:?}", *data);
			self.label_in_container.set_border(Color::WHITE, 1.);
		} else {
			println!("i {:?}", *data);
			self.label_in_container.set_border(theme::BACKGROUND_LIGHT, 1.);
		}
	}
}

pub struct CalendarDateWidget {
	days_widget: Vec<WidgetPod<String, Container<String>>>,
	//su, mo, tu, etc.
	// date of month cannot be a const. it changes per month
	dates_of_month_widget: Vec<WidgetPod<CalendarData, Container<CalendarData>>>, // this will be used to highlight.
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

	fn current_date_painter() -> Painter<u32> {
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

impl Widget<CalendarData> for CalendarDateWidget {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut CalendarData, env: &Env) {
		match event {
			Event::MouseDown(mouse_event) => {
				for (i, date_widget) in self.dates_of_month_widget.iter_mut().enumerate() {
					date_widget.event(ctx, event, data, env);
					if date_widget.has_active() { // we cant have date_widget.is_active()
						if data.active_index.is_some() {
							data.inactive_index = data.active_index;
						}
						data.active_index = Some(i);
						println!("active index {:?}", data.active_index);
						println!("inactive index {:?}", data.inactive_index);
					}
				}
			}
			Event::MouseUp(mouse_event) => {
				for date_widget in self.dates_of_month_widget.iter_mut() {
					date_widget.event(ctx, event, data, env);
				}
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
				for (i, date) in data.all_dates.iter().enumerate() {
					if *date == 1 {
						is_current_month = true;
					}

					if *date == data.current_day_of_month && is_current_month {
						let dynamic_date =
							Label::dynamic(|date_of_month: &u32, _| date_of_month.to_string())
								.center()
								.background(CalendarDateWidget::current_date_painter())
								.on_click(|ctx, data: &mut u32, _env| println!("clicked"))
								.lens(CalendarData::all_dates.index(i));
						let date_widget = Container::new(dynamic_date.padding(3.))
							.background(CalendarDateWidget::outer_date_painter());

						let mut date_widget = WidgetPod::new(date_widget);
						self.dates_of_month_widget.push(date_widget);
					} else {
						let dynamic_date =
							Label::dynamic(|date_of_month: &u32, _| date_of_month.to_string())
								.center()
								.on_click(|ctx, data: &mut u32, _env| println!("clicked"))
								.lens(CalendarData::all_dates.index(i));
						let date_widget = Container::new(dynamic_date.padding(3.))
							.background(CalendarDateWidget::outer_date_painter());

						let mut date_widget = WidgetPod::new(date_widget);
						self.dates_of_month_widget.push(date_widget);
					}
				}

				for dynamic_date in self.dates_of_month_widget.iter_mut() {
					dynamic_date.lifecycle(ctx, event, &data, env);
				}
			}
			_ => {
				for date_widget in self.dates_of_month_widget.iter_mut() {
					date_widget.lifecycle(ctx, event, &data, env);
				}
			}
		}
	}

	fn update(
		&mut self,
		ctx: &mut UpdateCtx,
		old_data: &CalendarData,
		data: &CalendarData,
		env: &Env,
	) {
		for date_widget in self.dates_of_month_widget.iter_mut() {
			date_widget.update(ctx, old_data, env);
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
				&data,
				env,
			);
			date_widget.set_layout_rect(ctx, &data, env, rect);
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

		for date_widget in self.dates_of_month_widget.iter_mut() {
			date_widget.paint(ctx, &data, env);
		}
	}
}
