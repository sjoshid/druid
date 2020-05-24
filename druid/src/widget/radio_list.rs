use crate::widget::{Label, LabelText, List, ListIter, MyRadio};
use crate::{BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Rect, Size, UpdateCtx, Widget, Point, WidgetPod};
use std::cmp::Ordering;

pub struct RadioList<T> {
    add_closure: Box<dyn Fn(&T, &Env) -> Label<T>>,
    children: Vec<WidgetPod<T, MyRadio<T>>>,
    selected_radio_index: u64,
}

impl<T: Data + PartialEq> RadioList<T> {
    pub fn new(closure: impl Fn(&T, &Env) -> Label<T> + 'static) -> Self {
        RadioList {
            add_closure: Box::new(closure),
            children: Vec::new(),
            selected_radio_index: 0,
        }
    }

    fn update_child_count(&mut self, data: &impl ListIter<T>, env: &Env) -> bool {
        let len = self.children.len();
        println!("update_child_count children {}", self.children.len());
        match len.cmp(&data.data_len()) {
            Ordering::Greater => self.children.truncate(data.data_len()),
            Ordering::Less => data.for_each(|child_data, i| {
                if i >= len {
                    let my_label = (self.add_closure)(child_data, env);
                    let mut my_radio = MyRadio::new(my_label);
                    let child = WidgetPod::new(my_radio);
                    self.children.push(child);
                }
            }),
            Ordering::Equal => (),
        }
        len != data.data_len()
    }
}

impl<C: Data + PartialEq, T: ListIter<C>> Widget<T> for RadioList<C> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        println!("Event {:?}", event);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        println!("lifecycle event {:?}", event);
        if let LifeCycle::WidgetAdded = event {
            if self.update_child_count(data, env) {
                ctx.children_changed();
            }
            // When this widget is added, by default we select first radio.
            if self.children.len() > 0 {
                let selected_radio = self.selected_radio_index;
                let first_radio = self.children.get_mut(selected_radio as usize).unwrap();
                let inner_first_radio = first_radio.widget_mut();
                inner_first_radio.selected = true;
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        println!("update children {}", self.children.len());
        if self.update_child_count(data, env) {
            ctx.children_changed();
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let mut width = bc.min().width;
        let mut y = 0.0;

        let mut paint_rect = Rect::ZERO;
        println!("layout children {}", self.children.len());
        let mut children = self.children.iter_mut();
        data.for_each(|child_data, _| {
            let child = match children.next() {
                Some(child) => child,
                None => {
                    return;
                }
            };
            let child_bc = BoxConstraints::new(
                Size::new(bc.min().width, 0.0),
                Size::new(bc.max().width, std::f64::INFINITY),
            );
            let child_size = child.layout(ctx, &child_bc, child_data, env);
            let rect = Rect::from_origin_size(Point::new(0.0, y), child_size);
            child.set_layout_rect(ctx, child_data, env, rect);
            paint_rect = paint_rect.union(child.paint_rect());
            width = width.max(child_size.width);
            y += child_size.height;
        });

        let my_size = bc.constrain(Size::new(width, y));
        let insets = paint_rect - Rect::ZERO.with_size(my_size);
        ctx.set_paint_insets(insets);
        my_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let mut children = self.children.iter_mut();
        data.for_each(|child_data, _| {
            if let Some(child) = children.next() {
                child.paint_with_offset(ctx, child_data, env);
            }
        });
    }
}
