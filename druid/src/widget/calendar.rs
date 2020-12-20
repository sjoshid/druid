use crate::calendar_data::{DAYS_OF_WEEK, DEFAULT_DAY_WIDGET_SIZE, DEFAULT_GRID_SPACING};
use crate::widget::{BackgroundBrush, Container, Label, Painter};
use crate::{
    theme, BoxConstraints, CalendarData, Env, Event, EventCtx, LayoutCtx, LensExt, LifeCycle,
    LifeCycleCtx, PaintCtx, Rect, RenderContext, Size, UpdateCtx, Widget, WidgetExt, WidgetPod,
};
use chrono::{Datelike, NaiveDate};
use druid_shell::piet::Color;
use std::ops::Add;

pub struct Calendar {
    days_widget: Vec<WidgetPod<String, Container<String>>>,
    //su, mo, tu, etc.
    // date of month cannot be a const. it changes per month
    dates_of_month_widget: Vec<WidgetPod<CalendarData, Container<CalendarData>>>, // this will be used to highlight.
}

impl Calendar {
    pub fn new() -> Calendar {
        Calendar {
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

            if ctx.is_hot() {
                ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
                println!("outer date hot");
            }

            if ctx.is_active() {
                println!("outer date active");
                ctx.fill(bounds, &env.get(theme::PRIMARY_LIGHT));
            }
        });
        painter
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
}

impl Widget<CalendarData> for Calendar {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut CalendarData, env: &Env) {}

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &CalendarData,
        env: &Env,
    ) {
        match event {
            LifeCycle::WidgetAdded => {
                self.days_widget = Calendar::get_days_of_week();
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
                                .background(Calendar::current_date_painter())
                                .lens(CalendarData::all_dates.index(i));
                        let date_widget = Container::new(dynamic_date.padding(3.))
                            .background(Calendar::outer_date_painter());

                        let mut date_widget = WidgetPod::new(date_widget);
                        self.dates_of_month_widget.push(date_widget);
                    } else {
                        let dynamic_date =
                            Label::dynamic(|date_of_month: &u32, _| date_of_month.to_string())
                                .center()
                                .lens(CalendarData::all_dates.index(i));
                        let date_widget = Container::new(dynamic_date.padding(3.))
                            .background(Calendar::outer_date_painter());

                        let mut date_widget = WidgetPod::new(date_widget);
                        self.dates_of_month_widget.push(date_widget);
                    }
                }

                for dynamic_date in self.dates_of_month_widget.iter_mut() {
                    dynamic_date.lifecycle(ctx, event, &data, env);
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
