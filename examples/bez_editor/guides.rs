use crate::path::{DPoint, PointId};

#[derive(Debug, Clone)]
pub struct Guide {
    pub id: PointId,
    pub guide: GuideLine,
}

/// A guideline.
#[derive(Debug, Clone)]
pub enum GuideLine {
    Horiz(DPoint),
    Vertical(DPoint),
    Angle { p1: DPoint, p2: DPoint },
}

impl Guide {
    fn new(guide: GuideLine) -> Self {
        let id = PointId::for_guide();
        Guide { id, guide }
    }

    pub fn horiz(p1: DPoint) -> Self {
        Guide::new(GuideLine::Horiz(p1))
    }

    pub fn vertical(p1: DPoint) -> Self {
        Guide::new(GuideLine::Vertical(p1))
    }

    pub fn angle(p1: DPoint, p2: DPoint) -> Self {
        Guide::new(GuideLine::Angle { p1, p2 })
    }

    pub fn toggle_vertical_horiz(&mut self) {
        let new = match self.guide {
            GuideLine::Horiz(point) => GuideLine::Vertical(point),
            GuideLine::Vertical(point) => GuideLine::Horiz(point),
            GuideLine::Angle { p1, p2 } => GuideLine::Angle { p1, p2 },
        };
        self.guide = new;
    }
}
