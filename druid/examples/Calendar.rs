use std::collections::HashMap;
use std::sync::Arc;

use druid::{AppLauncher, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle, LifeCycleCtx, PaintCtx, Size, theme, UpdateCtx, Widget, WidgetExt, WindowDesc};
use druid::widget::{BackgroundBrush, Container, Label};
use druid_shell::kurbo::{Circle, Point};
use druid_shell::piet::{Color, LinearGradient, RenderContext, UnitPoint};
use druid_shell::piet::kurbo::Rect;

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
    current_month_of_year: u32, // this will be used to show all days in current month.
}

struct Calendar {
    days_widget: Vec<Container<String>>, // this will be used to highlight.
}

impl Calendar {
    fn new() -> Calendar {
        Calendar {
            days_widget: Calendar::get_days_of_week(),
        }
    }

    fn get_days_of_week() -> Vec<Container<String>> {
        let mut days_widgets = Vec::with_capacity(7);

        for (i, day) in DAYS_OF_WEEK.iter().enumerate() {
            let day = Container::new(Label::new(String::from(*day)));
            let day = day.background(BackgroundBrush::Color(Color::rgb8(i as u8, i as u8, 55)));
            let day = day.border(Color::WHITE, 1.0);
            days_widgets.push(day);
        }

        days_widgets
    }
}

impl Widget<CalendarData> for Calendar {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut CalendarData, env: &Env) {
        for widget in self.days_widget.iter_mut() {
            widget.event(ctx, event, &mut String::from("Su"), env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &CalendarData, env: &Env) {
        for widget in self.days_widget.iter_mut() {
            widget.lifecycle(ctx, event, &String::from("Su"), env);
        }
        ctx.request_paint();
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &CalendarData, data: &CalendarData, env: &Env) {
        for widget in self.days_widget.iter_mut() {
            widget.update(ctx, &String::from("Su"), &String::from("Su"), env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &CalendarData, env: &Env) -> Size {
        for day in DAYS_OF_WEEK.iter() {
            let x_position = DEFAULT_GRID_SPACING;
            let y_position = DEFAULT_GRID_SPACING;
            for day_widget in self.days_widget.iter_mut() {
                let rect = Rect::new(
                    x_position,
                    y_position,
                    x_position + 25.0,
                    y_position + 25.0,
                );
                day_widget.layout(ctx, &BoxConstraints::new(DEFAULT_DAY_WIDGET_SIZE, DEFAULT_DAY_WIDGET_SIZE), &String::from(*day), env);
            }
        }
        // cell size is 25x25. spacing is 5.
        // so width = (25 + 5) * 7 = 210
        // height = (25 + 5) * 5 = 150
        // sj_todo
        Size {
            width: 210.0,
            height: 150.0,
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &CalendarData, env: &Env) {
        for day in DAYS_OF_WEEK.iter() {
            for day_widget in self.days_widget.iter_mut() {
                day_widget.paint(ctx, &String::from(*day), env);
            }
        }
        /*let circle = Circle::new((20.0 / 2., 20.0 / 2.), DEFAULT_RADIO_RADIUS);

        // Paint the background
        let background_gradient = LinearGradient::new(
            UnitPoint::TOP,
            UnitPoint::BOTTOM,
            (
                env.get(theme::BACKGROUND_LIGHT),
                env.get(theme::BACKGROUND_DARK),
            ),
        );

        ctx.fill(circle, &background_gradient);*/
    }
}

fn ui_builder() -> impl Widget<DateWidgetData> {
    Calendar::new().lens((DateWidgetData::day_and_month))
}

fn main() {
    let main_window = WindowDesc::new(ui_builder).title("Calendar");
    let day_and_month = CalendarData {
        current_day_of_month: 0,
        current_month_of_year: 0,
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