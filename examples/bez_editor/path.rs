use std::collections::HashSet;
use std::sync::Arc;

use druid::kurbo::{BezPath, Point, Vec2};
use druid::Data;

/// We give paths & points unique integer identifiers.
fn next_id() -> usize {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct PointId {
    pub(crate) path: usize,
    pub(crate) point: usize,
}

impl std::cmp::PartialEq<Path> for PointId {
    fn eq(&self, other: &Path) -> bool {
        self.path == other.id
    }
}

impl std::cmp::PartialEq<PointId> for Path {
    fn eq(&self, other: &PointId) -> bool {
        self.id == other.path
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointType {
    OnCurve,
    OnCurveSmooth,
    OffCurve,
}

#[derive(Debug, Clone, Copy)]
pub struct PathPoint {
    pub id: PointId,
    pub point: Point,
    pub typ: PointType,
}

#[derive(Debug, Clone)]
pub struct Path {
    id: usize,
    points: Arc<Vec<PathPoint>>,
    trailing: Option<Point>,
    closed: bool,
}

impl PointType {
    pub fn is_on_curve(self) -> bool {
        match self {
            PointType::OnCurve | PointType::OnCurveSmooth => true,
            PointType::OffCurve => false,
        }
    }
}

impl PathPoint {
    fn off_curve(path: usize, point: Point) -> PathPoint {
        let id = PointId {
            path,
            point: next_id(),
        };
        PathPoint {
            id,
            point,
            typ: PointType::OffCurve,
        }
    }

    fn on_curve(path: usize, point: Point) -> PathPoint {
        let id = PointId {
            path,
            point: next_id(),
        };
        PathPoint {
            id,
            point,
            typ: PointType::OnCurve,
        }
    }

    pub fn is_on_curve(&self) -> bool {
        self.typ.is_on_curve()
    }
}

impl Path {
    pub fn new(point: Point) -> Path {
        let id = next_id();
        let start = PathPoint::on_curve(id, point);

        Path {
            id,
            points: Arc::new(vec![start]),
            closed: false,
            trailing: None,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub fn points(&self) -> &[PathPoint] {
        self.points.as_slice()
    }

    fn points_mut(&mut self) -> &mut Vec<PathPoint> {
        Arc::make_mut(&mut self.points)
    }

    pub fn trailing(&self) -> Option<&Point> {
        self.trailing.as_ref()
    }

    pub fn clear_trailing(&mut self) {
        self.trailing = None;
    }

    /// Whether we should draw the 'trailing' control point & handle.
    /// We always do this for the first point, if it exists; otherwise
    /// we do it for curve points only.
    pub fn should_draw_trailing(&self) -> bool {
        self.points.len() == 1 || self.last_segment_is_curve()
    }

    /// Returns the start point of the path.
    pub fn start_point(&self) -> &PathPoint {
        if self.closed {
            self.points.last().unwrap()
        } else {
            self.points.first().unwrap()
        }
    }

    pub fn to_bezier(&self) -> BezPath {
        let mut bez = BezPath::new();
        bez.move_to(self.start_point().point);
        let mut i = if self.closed { 0 } else { 1 };
        self.debug_print_points();

        while i < self.points.len() {
            if self.points[i].is_on_curve() {
                bez.line_to(self.points[i].point);
                i += 1;
            } else {
                bez.curve_to(
                    self.points[i].point,
                    self.points[i + 1].point,
                    self.points[self.next_idx(i + 1)].point,
                );
                i += 3;
            }
        }
        if self.closed {
            bez.close_path();
        }
        bez
    }

    /// Appends a point. Called when the user clicks. This point is always a corner;
    /// if the user drags it will be converted to a curve then.
    ///
    /// Returns the id of the newly added point, or the start/end point if this
    /// closes the path.
    pub fn append_point(&mut self, point: Point) -> PointId {
        if !self.closed && point == self.points[0].point {
            return self.close();
        }
        let new = PathPoint::on_curve(self.id, point);
        Arc::make_mut(&mut self.points).push(new);
        new.id
    }

    pub fn nudge_points(&mut self, points: &[PointId], v: Vec2) {
        let mut to_nudge = HashSet::new();
        for point in points {
            to_nudge.insert(*point);
            let idx = match self.points.iter().position(|p| p.id == *point) {
                Some(idx) => idx,
                None => continue,
            };
            if self.points[idx].is_on_curve() {
                let prev = self.prev_idx(idx);
                let next = self.next_idx(idx);
                if !self.points[prev].is_on_curve() {
                    to_nudge.insert(self.points[prev].id);
                }
                if !self.points[next].is_on_curve() {
                    to_nudge.insert(self.points[next].id);
                }
            }
        }

        for point in to_nudge {
            self.nudge_point(point, v);
        }
    }

    fn nudge_point(&mut self, point_id: PointId, v: Vec2) {
        if let Some(p) = Arc::make_mut(&mut self.points)
            .iter_mut()
            .find(|p| p.id == point_id)
        {
            p.point += v;
        }
    }

    fn debug_print_points(&self) {
        eprintln!(
            "path {}, len {} closed {}",
            self.id,
            self.points.len(),
            self.closed
        );
        for point in self.points.iter() {
            eprintln!(
                "[{}, {}]: {:?} {:?}",
                point.id.path, point.id.point, point.point, point.typ
            );
        }
    }

    pub fn delete_points(&mut self, points: &[PointId]) {
        eprintln!("deleting {:?}", points);
        for point in points {
            self.delete_point(*point)
        }
    }

    //FIXME: this is currently buggy :(
    fn delete_point(&mut self, point_id: PointId) {
        let idx = match self.points.iter().position(|p| p.id == point_id) {
            Some(idx) => idx,
            None => return,
        };

        let prev_idx = self.prev_idx(idx);
        let next_idx = self.next_idx(idx);

        eprintln!("deleting {:?}", point_id);
        self.debug_print_points();

        match self.points[idx].typ {
            PointType::OffCurve => {
                // delete both of the off curve points for this segment
                let other_id = if self.points[prev_idx].typ == PointType::OffCurve {
                    self.points[prev_idx].id
                } else {
                    assert!(self.points[next_idx].typ == PointType::OffCurve);
                    self.points[next_idx].id
                };
                self.points_mut()
                    .retain(|p| p.id != point_id && p.id != other_id);
            }
            _on_curve if self.points.len() == 1 => {
                self.points_mut().clear();
            }
            // with less than 4 points they must all be on curve
            _on_curve if self.points.len() == 4 => {
                self.points_mut()
                    .retain(|p| p.is_on_curve() && p.id != point_id);
            }

            _on_curve if self.points[prev_idx].is_on_curve() => {
                // this is a line segment
                self.points_mut().remove(idx);
            }
            _on_curve if self.points[next_idx].is_on_curve() => {
                // if we neighbour a corner point, leave handles (neighbour becomes curve)
                self.points_mut().remove(idx);
            }
            _ => {
                assert!(self.points.len() > 4);
                let prev = self.points[prev_idx];
                let next = self.points[next_idx];
                assert!(!prev.is_on_curve() && !next.is_on_curve());
                let to_del = [prev.id, next.id, point_id];
                self.points_mut().retain(|p| !to_del.contains(&p.id));
                if self.points.len() == 3 {
                    self.points_mut().retain(|p| p.is_on_curve());
                }
            }
        }

        // normalize our representation
        let len = self.points.len();
        if len > 2 && !self.points[0].is_on_curve() && !self.points[len - 1].is_on_curve() {
            self.points_mut().rotate_left(1);
        }

        // if we have fewer than three on_curve points we are open.
        if self.points.len() < 3 {
            self.closed = false;
        }
    }

    /// Called when the user drags (modifying the bezier control points) after clicking.
    pub fn update_for_drag(&mut self, handle: Point) {
        assert!(!self.points.is_empty());
        if !self.last_segment_is_curve() {
            self.convert_last_to_curve(handle);
        } else {
            self.update_trailing(handle);
        }
    }

    pub fn last_segment_is_curve(&self) -> bool {
        let len = self.points.len();
        len > 2 && !self.points[len - 2].is_on_curve()
    }

    /// If the user drags after mousedown, we convert the last point to a curve.
    fn convert_last_to_curve(&mut self, handle: Point) {
        assert!(!self.points.is_empty());
        if self.points.len() > 1 {
            let mut prev = self.points_mut().pop().unwrap();
            prev.typ = PointType::OnCurveSmooth;
            let p1 = self
                .trailing
                .take()
                .unwrap_or(self.points.last().unwrap().point);
            let p2 = prev.point - (handle - prev.point);
            let pts = &[
                PathPoint::off_curve(self.id, p1),
                PathPoint::off_curve(self.id, p2),
                prev,
            ];
            Arc::make_mut(&mut self.points).extend(pts);
        }
        self.trailing = Some(handle);
    }

    /// Update the curve while the user drags a new control point.
    fn update_trailing(&mut self, handle: Point) {
        if self.points.len() > 1 {
            let len = self.points.len();
            assert!(self.points[len - 1].typ != PointType::OffCurve);
            assert!(self.points[len - 2].typ == PointType::OffCurve);
            let on_curve_pt = self.points[len - 1].point;
            Arc::make_mut(&mut self.points)[len - 2].point = on_curve_pt - (handle - on_curve_pt);
        }
        self.trailing = Some(handle);
    }

    // in an open path, the first point is essentially a `move_to` command.
    // 'closing' the path means moving this path to the end of the list.
    fn close(&mut self) -> PointId {
        assert!(!self.closed);
        Arc::make_mut(&mut self.points).rotate_left(1);
        self.closed = true;
        self.points.last().unwrap().id
    }

    #[inline]
    fn prev_idx(&self, idx: usize) -> usize {
        if idx == 0 {
            self.points.len() - 1
        } else {
            idx - 1
        }
    }

    #[inline]
    fn next_idx(&self, idx: usize) -> usize {
        (idx + 1) % self.points.len()
    }

    fn idx_for_point(&self, point: PointId) -> Option<usize> {
        self.points.iter().position(|p| p.id == point)
    }

    pub(crate) fn prev_point(&self, point: PointId) -> PathPoint {
        assert!(point.path == self.id);
        let idx = self.idx_for_point(point).expect("bad input to prev_point");
        let idx = self.prev_idx(idx);
        self.points[idx]
    }

    pub(crate) fn next_point(&self, point: PointId) -> PathPoint {
        assert!(point.path == self.id);
        let idx = self.idx_for_point(point).expect("bad input to next_point");
        let idx = self.next_idx(idx);
        self.points[idx]
    }
}

impl Data for Path {
    fn same(&self, other: &Self) -> bool {
        self.points.same(&other.points)
            && self.closed.same(&other.closed)
            && self.trailing == other.trailing
    }
}
