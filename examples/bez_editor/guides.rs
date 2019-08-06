use crate::path::{DPoint, DVec2, PointId};
use crate::ViewPort;
use druid::kurbo::ParamCurveNearest;
use druid::kurbo::{Line, Point, Vec2};

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

    pub fn toggle_vertical_horiz(&mut self, new_point: DPoint) {
        let new = match self.guide {
            GuideLine::Horiz(_) => GuideLine::Vertical(new_point),
            GuideLine::Vertical(_) => GuideLine::Horiz(new_point),
            GuideLine::Angle { p1, p2 } => GuideLine::Angle { p1, p2 },
        };
        self.guide = new;
    }

    pub fn screen_dist(&self, vport: ViewPort, point: Point) -> f64 {
        match self.guide {
            GuideLine::Horiz(p) => {
                let Point { y, .. } = p.to_screen(vport);
                (point.y - y).abs()
            }
            GuideLine::Vertical(p) => {
                let Point { x, .. } = p.to_screen(vport);
                (point.x - x).abs()
            }
            GuideLine::Angle { p1, p2 } => {
                //FIXME: this line is not not infinite, which it should be.
                let line = vport.transform() * Line::new(p1.to_raw(), p2.to_raw());
                let (x, y) = line.nearest(point, 0.1);
                Vec2::new(x, y).hypot()
            }
        }
    }

    pub fn nudge(&mut self, nudge: DVec2) {
        match self.guide {
            GuideLine::Horiz(ref mut p) => p.y += nudge.y,
            GuideLine::Vertical(ref mut p) => p.x += nudge.x,
            GuideLine::Angle {
                ref mut p1,
                ref mut p2,
            } => {
                p1.x += nudge.x;
                p2.x += nudge.x;
                p1.y += nudge.y;
                p2.y += nudge.y;
            }
        }
    }
}
