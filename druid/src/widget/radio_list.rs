use crate::widget::{Label, LabelText, List, ListIter, MyRadio};
use crate::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Rect,
    Size, UpdateCtx, Widget,
};

pub struct RadioList<T> {
    add_closure: Box<dyn Fn(&T, &Env) -> Label<T>>,
    radios: Vec<MyRadio<T>>,
}

impl<T: Data + PartialEq> RadioList<T> {
    pub fn new(closure: impl Fn(&T, &Env) -> Label<T> + 'static) -> Self {
        RadioList {
            add_closure: Box::new(closure),
            radios: Vec::new(),
        }
    }
}

impl<C: Data + PartialEq, T: ListIter<C>> Widget<T> for RadioList<C> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {}

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        data.for_each(|child_data, _| {
            let my_label = (self.add_closure)(child_data, env);
            let mut my_radio = MyRadio::new(my_label);
            my_radio.layout(ctx, bc, child_data, env);
            self.radios.push(my_radio);
        });
        let my_size = bc.constrain(Size::new(0.0, 0.0));
        let insets = Rect::ZERO - Rect::ZERO.with_size(my_size);
        ctx.set_paint_insets(insets);
        my_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {}
}
