use crate::kurbo::{Circle, Point, Rect, Size};
use crate::theme;
use crate::widget::{CrossAxisAlignment, Flex, Label, LabelText, Padding};
use crate::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, LinearGradient,
    PaintCtx, RenderContext, UnitPoint, UpdateCtx, Widget, WidgetExt, WidgetPod,
};

/// Radio without variant.
pub struct MyRadio<T> {
    child_label: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T: Data> MyRadio<T> {
    pub fn new(label: impl Into<LabelText<T>>) -> MyRadio<T> {
        MyRadio {
            child_label: WidgetPod::new(Label::new(label).boxed()),
        }
    }
}

impl<T: Data + PartialEq> Widget<T> for MyRadio<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    if ctx.is_hot() {
                        //*data = self.variant.clone();
                    }
                    ctx.request_paint();
                }
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &T, _env: &Env) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("RadioList");

        let label_size = self.child_label.layout(ctx, &bc, data, env);
        let padding = 5.0;
        let label_x_offset = env.get(theme::BASIC_WIDGET_HEIGHT) + padding;
        let origin = Point::new(label_x_offset, 0.0);

        self.child_label.set_layout_rect(
            ctx,
            data,
            env,
            Rect::from_origin_size(origin, label_size),
        );

        bc.constrain(Size::new(
            label_x_offset + label_size.width,
            env.get(theme::BASIC_WIDGET_HEIGHT).max(label_size.height),
        ))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = env.get(theme::BASIC_WIDGET_HEIGHT);

        let circle = Circle::new((size / 2., size / 2.), 7.);

        // Paint the background
        let background_gradient = LinearGradient::new(
            UnitPoint::TOP,
            UnitPoint::BOTTOM,
            (
                env.get(theme::BACKGROUND_LIGHT),
                env.get(theme::BACKGROUND_DARK),
            ),
        );

        ctx.fill(circle, &background_gradient);

        let border_color = if ctx.is_hot() {
            env.get(theme::BORDER_LIGHT)
        } else {
            env.get(theme::BORDER_DARK)
        };

        ctx.stroke(circle, &border_color, 1.);

        // Check if data enum matches our variant
        let inner_circle = Circle::new((size / 2., size / 2.), 2.);

        ctx.fill(inner_circle, &env.get(theme::LABEL_COLOR));

        // Paint the text label
        self.child_label.paint_with_offset(ctx, data, env);
    }
}
