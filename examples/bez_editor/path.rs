use std::sync::Arc;

use druid::kurbo::{BezPath, Point, Vec2};
use druid::Data;

/// We give paths & points unique integer identifiers.
fn next_id() -> usize {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct PointId {
    path: usize,
    point: usize,
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
    Corner,
    Curve,
    OffCurve,
    Smooth,
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
    pub fn is_corner(self) -> bool {
        match self {
            PointType::Corner => true,
            _ => false,
        }
    }

    pub fn is_on_curve(self) -> bool {
        match self {
            PointType::Curve | PointType::Smooth | PointType::Corner => true,
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
            typ: PointType::Corner,
        }
    }

    pub fn is_on_curve(&self) -> bool {
        self.typ.is_on_curve()
    }

    pub fn is_corner(&self) -> bool {
        self.typ.is_corner()
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
        self.points().len() == 1 || !self.points.last().unwrap().is_corner()
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

        while i < self.points.len() {
            if self.points[i].is_on_curve() {
                bez.line_to(self.points[i].point);
                i += 1;
            } else {
                bez.curve_to(
                    self.points[i].point,
                    self.points[i + 1].point,
                    self.points[i + 2].point,
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

    pub fn nudge_point(&mut self, point_id: PointId, v: Vec2) {
        if let Some(p) = Arc::make_mut(&mut self.points)
            .iter_mut()
            .find(|p| p.id == point_id)
        {
            p.point += v;
        }
    }

    /// Called when the user drags (modifying the bezier control points) after clicking.
    pub fn update_for_drag(&mut self, handle: Point) {
        assert!(!self.points.is_empty());
        if self
            .points
            .last()
            .map(|p| p.typ == PointType::Corner)
            .unwrap()
        {
            self.convert_last_to_curve(handle);
        } else {
            self.update_trailing(handle);
        }
    }

    /// If the user drags after mousedown, we convert the last point to a curve.
    fn convert_last_to_curve(&mut self, handle: Point) {
        assert!(!self.points.is_empty());
        if self.points.len() > 1 {
            let mut prev = Arc::make_mut(&mut self.points).pop().unwrap();
            prev.typ = PointType::Curve;
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
}

impl Data for Path {
    fn same(&self, other: &Self) -> bool {
        self.points.same(&other.points)
            && self.closed.same(&other.closed)
            && self.trailing == other.trailing
    }
}
