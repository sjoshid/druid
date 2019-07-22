//! The bezier pen tool.

use std::collections::BTreeSet;
use std::sync::Arc;

use druid::kurbo::{Point, Rect, Vec2};
use druid::{KeyCode, KeyEvent, MouseEvent};

use super::{Drag, MouseDelegate, Tool};
use crate::path::{DVec2, PointId};
use crate::{Contents, MIN_POINT_DISTANCE};

/// The state of the selection tool.
#[derive(Debug, Clone)]
pub struct Select {
    /// when a drag is in progress, this is the state of the selection at the start
    /// of the drag.
    prev_selection: Option<Arc<BTreeSet<PointId>>>,
    drag_rect: Option<Rect>,
    last_drag_pos: Option<Point>,
}

impl Select {
    pub fn new() -> Self {
        Select {
            prev_selection: None,
            drag_rect: None,
            last_drag_pos: None,
        }
    }

    fn update_selection_for_drag(
        &self,
        canvas: &mut Contents,
        prev_sel: &BTreeSet<PointId>,
        rect: Rect,
        shift: bool,
    ) {
        let vport = canvas.vport;
        let in_select_rect = canvas
            .iter_points()
            .filter(|p| rect_contains(rect, p.to_screen(vport)))
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
        canvas.nudge_selection(DVec2::from_raw(nudge));
    }
}

impl MouseDelegate<Contents> for Select {
    fn left_down(&mut self, canvas: &mut Contents, event: &MouseEvent) -> bool {
        if event.count == 1 {
            let sel = canvas
                .iter_points()
                .find(|p| p.screen_dist(canvas.vport, event.pos) <= MIN_POINT_DISTANCE)
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
        } else if event.count == 2 {
            if canvas
                .iter_points()
                .find(|p| p.screen_dist(canvas.vport, event.pos) <= MIN_POINT_DISTANCE)
                .map(|p| p.is_on_curve())
                .unwrap_or(false)
            {
                canvas.toggle_selected_on_curve_type();
            } else {
                canvas.select_path(event.pos, event.mods.shift);
            }
        }
        true
    }

    fn left_up(&mut self, _canvas: &mut Contents, _event: &MouseEvent) -> bool {
        self.prev_selection = None;
        self.drag_rect = None;
        true
    }

    fn left_drag_began(&mut self, canvas: &mut Contents, drag: Drag) -> bool {
        self.prev_selection = if canvas
            .iter_points()
            .any(|p| p.screen_dist(canvas.vport, drag.start.pos) <= MIN_POINT_DISTANCE)
        {
            None
        } else {
            Some(canvas.selection.clone())
        };
        true
    }

    fn left_drag_changed(&mut self, canvas: &mut Contents, drag: Drag) -> bool {
        if let Some(prev_selection) = self.prev_selection.as_ref() {
            let rect = Rect::from_points(drag.current.pos, drag.start.pos);
            self.drag_rect = Some(rect);
            self.update_selection_for_drag(canvas, prev_selection, rect, drag.current.mods.shift);
        } else {
            let last_drag_pos = self.last_drag_pos.unwrap_or(drag.start.pos);
            let dvec = drag.current.pos - last_drag_pos;
            let drag_vec = dvec * (1.0 / canvas.vport.zoom);
            let drag_vec = DVec2::from_raw((drag_vec.x.floor(), drag_vec.y.floor()));
            if drag_vec.hypot() > 0. {
                // multiple small drag updates that don't make up a single point in design
                // space should be aggregated
                let aligned_drag_delta = drag_vec.to_screen(canvas.vport);
                let aligned_last_drag = last_drag_pos + aligned_drag_delta;
                self.last_drag_pos = Some(aligned_last_drag);
                canvas.nudge_selection(drag_vec);
            }
        }
        true
    }

    fn left_drag_ended(&mut self, _canvas: &mut Contents, _drag: Drag) -> bool {
        self.last_drag_pos = None;
        true
    }

    fn cancel(&mut self, canvas: &mut Contents) {
        if let Some(prev) = self.prev_selection.take() {
            canvas.selection = prev;
        }
        self.drag_rect = None;
    }
}

impl Tool for Select {
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
                canvas.delete_selection();
                true
            }
            e if e.text() == Some("a") && e.mods.meta && !e.mods.shift => {
                canvas.select_all();
                true
            }
            e if e.key_code == Tab => {
                if e.mods.shift {
                    canvas.select_prev();
                } else {
                    canvas.select_next();
                }
                true
            }
            _ => false,
        }
    }

    fn selection_rect(&self) -> Option<Rect> {
        self.drag_rect
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
