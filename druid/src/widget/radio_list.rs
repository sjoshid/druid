use crate::widget::{List, ListIter, MyRadio};
use crate::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Widget,
};

pub struct RadioList<T> {
    radios: List<T>,
}

impl<T: Data + PartialEq> RadioList<T> {
    pub fn new(closure: impl Fn() -> MyRadio<T> + 'static) -> Self {
        RadioList {
            radios: List::new(closure),
        }
    }
}

impl<C: Data, T: ListIter<C>> Widget<T> for RadioList<C> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {}

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.radios.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {}
}
