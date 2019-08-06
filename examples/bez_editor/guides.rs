use crate::path::DPoint;
use druid::kurbo::{Line, Rect};

/// A guideline, represented as two points on an infinite line.
///
/// It is an invariant that the points are non-equal.
#[derive(Debug, Clone)]
pub enum Guide {
    Horiz(DPoint),
    Vertical(DPoint),
    Angle { p1: DPoint, p2: DPoint },
}

impl Guide {
    pub fn horiz(p1: DPoint) -> Self {
        Guide::Horiz(p1)
    }

    pub fn angle(p1: DPoint, p2: DPoint) -> Self {
        Guide::Angle { p1, p2 }
    }

    pub fn toggle_vertical_horiz(&mut self) {
        let new = match &self {
            Guide::Horiz(point) => Guide::Vertical(*point),
            Guide::Vertical(point) => Guide::Horiz(*point),
            Guide::Angle { p1, p2 } => Guide::angle(*p1, *p2),
        };
        *self = new;
    }
}
