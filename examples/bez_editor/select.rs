//! The bezier pen tool.

use std::collections::BTreeSet;
use std::sync::Arc;

use druid::kurbo::{Point, Rect, Vec2};
use druid::{Event, KeyCode, KeyEvent, MouseEvent};

use super::path::PointId;
use super::{Contents, Mouse, Tool, MIN_POINT_DISTANCE};

/// The state of the selection tool.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Select {
    mouse: Mouse,
    /// when a drag is in progress, this is the state of the selection at the start of the drag.
    prev_selection: Option<Arc<BTreeSet<PointId>>>,
}

impl Select {
    pub fn new() -> Self {
        Select {
            mouse: Mouse::Up(Point::ZERO),
            prev_selection: None,
        }
    }

    fn mouse_down(&mut self, canvas: &mut Contents, event: &MouseEvent) -> bool {
        let sel = canvas
            .iter_points()
            .find(|p| p.point.distance(event.pos) <= MIN_POINT_DISTANCE)
            .map(|p| p.id);
        if let Some(point_id) = sel {
            if !event.mods.shift {
                // when clicking a point, if it is not selected we set it as the selection,
                // otherwise we keep the selection intact for a drag.
                if !canvas.selection.contains(&point_id) {
                    canvas.selection_mut().clear();
                    canvas.selection_mut().insert(point_id);
                }
            } else if !canvas.selection_mut().remove(&point_id) {
                canvas.selection_mut().insert(point_id);
            }
        } else if !event.mods.shift {
            canvas.selection_mut().clear();
        }
        self.mouse = Mouse::Down(event.pos);
        true
    }

    fn mouse_up(&mut self, _canvas: &mut Contents, event: &MouseEvent) -> bool {
        self.prev_selection = None;
        self.mouse = Mouse::Up(event.pos);
        true
    }

    //TODO: this is identical to the code for the Pen. maybe we want something
    //more like a shared gesture tracking state machine thing?
    fn mouse_moved(&mut self, canvas: &mut Contents, event: &MouseEvent) -> bool {
        self.mouse = match self.mouse {
            Mouse::Up(_) => Mouse::Up(event.pos),
            Mouse::Drag { start, current, .. } => Mouse::Drag {
                start,
                last: current,
                current: event.pos,
            },
            Mouse::Down(point) => {
                // have we moved far enough to start a drag gesture?
                if point.distance(event.pos) > MIN_POINT_DISTANCE {
                    // was the original click on a point?
                    let on_point = canvas
                        .iter_points()
                        .any(|p| p.point.distance(point) <= MIN_POINT_DISTANCE);
                    self.prev_selection = if on_point {
                        None
                    } else {
                        Some(canvas.selection.clone())
                    };

                    Mouse::Drag {
                        start: point,
                        last: point,
                        current: event.pos,
                    }
                } else {
                    Mouse::Down(point)
                }
            }
        };

        if let Mouse::Drag {
            start,
            current,
            last,
        } = self.mouse
        {
            if let Some(prev_selection) = self.prev_selection.as_ref() {
                let rect = Rect::from_points(current, start);
                self.update_selection_for_drag(canvas, prev_selection, rect, event.mods.shift);
            } else {
                canvas.nudge_selection(current - last);
            }
            true
        } else {
            false
        }
    }

    fn key_down(&mut self, canvas: &mut Contents, event: &KeyEvent) -> bool {
        use KeyCode::*;
        match event {
            e if e.key_code == ArrowLeft
                || e.key_code == ArrowDown
                || e.key_code == ArrowUp
                || e.key_code == ArrowRight =>
            {
                self.nudge(canvas, event);
                true
            }
            e if e.key_code == Backspace => {
                self.delete_selection(canvas);
                true
            }
            _ => false,
        }
    }

    fn update_selection_for_drag(
        &self,
        canvas: &mut Contents,
        prev_sel: &BTreeSet<PointId>,
        rect: Rect,
        shift: bool,
    ) {
        let in_select_rect = canvas
            .iter_points()
            .filter(|p| rect_contains(rect, p.point))
            .map(|p| p.id)
            .collect();
        let new_sel = if shift {
            prev_sel
                .symmetric_difference(&in_select_rect)
                .copied()
                .collect()
        } else {
            prev_sel.union(&in_select_rect).copied().collect()
        };
        *canvas.selection_mut() = new_sel;
    }

    fn nudge(&mut self, canvas: &mut Contents, event: &KeyEvent) {
        use KeyCode::*;
        let mut nudge = match event.key_code {
            ArrowLeft => Vec2::new(-1.0, 0.),
            ArrowRight => Vec2::new(1.0, 0.),
            ArrowUp => Vec2::new(0.0, -1.0),
            ArrowDown => Vec2::new(0.0, 1.0),
            _ => unreachable!(),
        };

        if event.mods.meta {
            nudge *= 100.;
        } else if event.mods.shift {
            nudge *= 10.;
        }
        canvas.nudge_selection(nudge);
    }

    fn delete_selection(&mut self, _canvas: &mut Contents) {}
}

impl Tool for Select {
    fn event(&mut self, data: &mut Contents, event: &Event) -> bool {
        match event {
            Event::MouseDown(mouse) if mouse.button.is_right() => false,
            Event::MouseUp(mouse) if mouse.button.is_right() => false,
            Event::MouseMoved(mouse) if mouse.button.is_right() => false,
            Event::MouseDown(mouse) if mouse.count == 1 => self.mouse_down(data, mouse),
            Event::MouseMoved(mouse) => self.mouse_moved(data, mouse),
            Event::MouseUp(mouse) => self.mouse_up(data, mouse),
            Event::KeyDown(key) => self.key_down(data, key),
            //Event::MouseDown(mouse) if mouse.count == 2 => self.double_click(data, mouse),
            _ => false,
        }
    }

    fn selection_rect(&self) -> Option<Rect> {
        match self.mouse {
            Mouse::Drag { start, current, .. } if self.prev_selection.is_some() => {
                Some(Rect::from_points(start, current))
            }
            _ => None,
        }
    }

    fn boxed_clone(&self) -> Box<dyn Tool> {
        Box::new(self.clone())
    }

    fn name(&self) -> &str {
        "select"
    }
}

fn rect_contains(rect: Rect, point: Point) -> bool {
    point.x >= rect.x0 && point.x <= rect.x1 && point.y >= rect.y0 && point.y <= rect.y1
}
