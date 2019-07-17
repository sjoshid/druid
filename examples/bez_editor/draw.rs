//! Drawing algorithms and helpers

use std::collections::BTreeSet;

use super::path::{Path, PointId, PointType};
use super::Tool;
use druid::kurbo::{Affine, BezPath, Circle, CubicBez, Line, PathSeg, Point, QuadBez, Rect, Vec2};
use druid::piet::{Color, FillRule::NonZero, RenderContext};
use druid::PaintCtx;

const PATH_COLOR: Color = Color::rgb24(0x00_00_00);
const SELECTION_RECT_BG_COLOR: Color = Color::rgba32(0xDD_DD_DD_55);
const SELECTION_RECT_STROKE_COLOR: Color = Color::rgb24(0x53_8B_BB);
const ON_CURVE_POINT_COLOR: Color = Color::rgb24(0x0b_2b_db);
const OFF_CURVE_POINT_COLOR: Color = Color::rgb24(0xbb_bb_bb);
const OFF_CURVE_HANDLE_COLOR: Color = Color::rgb24(0xbb_bb_bb);
const DIRECTION_ARROW_COLOR: Color = Color::rgba32(0x00_00_00_44);

const ON_CURVE_RADIUS: f64 = 3.5;
const ON_CURVE_SELECTED_RADIUS: f64 = 4.;
const OFF_CURVE_RADIUS: f64 = 2.;
const OFF_CURVE_SELECTED_RADIUS: f64 = 2.5;

trait PaintHelpers: RenderContext {
    fn draw_path(&mut self, bez: &BezPath) {
        let path_brush = self.solid_brush(PATH_COLOR);
        self.stroke(bez, &path_brush, 1.0, None);
    }

    fn draw_control_handle(&mut self, p1: Point, p2: Point) {
        let brush = self.solid_brush(OFF_CURVE_HANDLE_COLOR);
        let l = Line::new(p1, p2);
        self.stroke(l, &brush, 1.0, None);
    }

    fn draw_point(&mut self, point: PointStyle) {
        let PointStyle {
            style,
            point,
            selected,
        } = point;
        match style {
            Style::Open(seg) => self.draw_open_path_terminal(&seg, selected),
            Style::Close(seg) => self.draw_open_path_terminal(&seg, selected),
            Style::OffCurve => self.draw_off_curve_point(point, selected),
            _ => self.draw_on_curve_point(point, selected),
        }
    }

    fn draw_open_path_terminal(&mut self, seg: &PathSeg, selected: bool) {
        let (p0, p1, p2, p3) = cubic_points_for_seg(seg);
        let cap = cap_line(p0, p1, p2, p3, 12.);
        let brush = self.solid_brush(OFF_CURVE_HANDLE_COLOR);
        if selected {
            self.stroke(cap, &brush, 3.0, None);
        } else {
            self.stroke(cap, &brush, 2.0, None);
        }
    }

    fn draw_on_curve_point(&mut self, p: Point, selected: bool) {
        let radius = if selected {
            ON_CURVE_SELECTED_RADIUS
        } else {
            ON_CURVE_RADIUS
        };
        let brush = self.solid_brush(ON_CURVE_POINT_COLOR);
        let circ = Circle::new(p, radius);
        if selected {
            self.fill(circ, &brush, NonZero);
        } else {
            self.stroke(circ, &brush, 1.0, None);
        }
    }

    fn draw_off_curve_point(&mut self, p: Point, selected: bool) {
        let radius = if selected {
            OFF_CURVE_SELECTED_RADIUS
        } else {
            OFF_CURVE_RADIUS
        };
        let brush = self.solid_brush(OFF_CURVE_POINT_COLOR);
        let circ = Circle::new(p, radius);
        if selected {
            self.fill(circ, &brush, NonZero);
        } else {
            self.stroke(circ, &brush, 1.0, None);
        }
    }

    fn draw_selection_rect(&mut self, rect: Rect) {
        let bg_brush = self.solid_brush(SELECTION_RECT_BG_COLOR);
        let stroke_brush = self.solid_brush(SELECTION_RECT_STROKE_COLOR);
        self.fill(rect, &bg_brush, NonZero);
        self.stroke(rect, &stroke_brush, 1.0, None);
    }

    fn draw_direction_indicator(&mut self, path: &BezPath) {
        let (p0, p1, p2, p3) = match path.segments().next() {
            None => return,
            Some(seg) => cubic_points_for_seg(&seg),
        };

        let mut arrow = make_arrow();
        let tangent = tangent_vector(0.05, p0, p1, p2, p3);
        let tangent = tangent / tangent.hypot(); // normalized
        let angle = Vec2::new(tangent.y, -tangent.x);
        let rotate = Affine::rotate(angle.atan2());
        let translate = Affine::translate(p0.to_vec2() + tangent * 4.0);
        arrow.apply_affine(rotate);
        arrow.apply_affine(translate);
        let brush = self.solid_brush(DIRECTION_ARROW_COLOR);
        self.fill(arrow, &brush, NonZero);
    }
}

impl<T: RenderContext> PaintHelpers for T {}

struct PointStyle {
    point: Point,
    style: Style,
    selected: bool,
}

enum Style {
    Open(PathSeg),
    Close(PathSeg),
    Corner,
    Smooth,
    Tangent,
    OffCurve,
}

struct PointIter<'a> {
    idx: usize,
    path: &'a Path,
    bez: &'a BezPath,
    sels: &'a BTreeSet<PointId>,
}

impl<'a> PointIter<'a> {
    fn new(path: &'a Path, bez: &'a BezPath, sels: &'a BTreeSet<PointId>) -> Self {
        PointIter {
            idx: 0,
            bez,
            path,
            sels,
        }
    }

    fn next_style(&self) -> Style {
        let len = self.path.points().len();
        if len == 1 {
            return Style::Corner;
        }

        let next = self.path.points()[self.idx];
        if next.is_on_curve() && !self.path.is_closed() {
            if self.idx == 0 {
                return Style::Open(self.bez.segments().next().unwrap());
            } else if self.idx == len - 1 {
                return Style::Close(reverse_seg(&self.bez.segments().last().unwrap()));
            }
        }

        match next.typ {
            PointType::OnCurve => Style::Corner,
            PointType::OffCurve => Style::OffCurve,
            PointType::OnCurveSmooth => {
                let prev = self.path.prev_point(next.id);
                let next = self.path.next_point(next.id);
                match (prev.is_on_curve(), next.is_on_curve()) {
                    (false, false) => Style::Smooth,
                    (true, false) => Style::Tangent,
                    (false, true) => Style::Tangent,
                    (true, true) => Style::Corner,
                }
            }
        }
    }
}

impl<'a> std::iter::Iterator for PointIter<'a> {
    type Item = PointStyle;
    fn next(&mut self) -> Option<PointStyle> {
        let point = self.path.points().get(self.idx)?;
        let style = self.next_style();
        let selected = self.sels.contains(&point.id);
        let point = point.point;
        self.idx += 1;
        Some(PointStyle {
            point,
            style,
            selected,
        })
    }
}

fn draw_control_point_lines(path: &Path, paint_ctx: &mut PaintCtx) {
    let mut prev_point = path.start_point().point;
    let mut idx = 0;
    while idx < path.points().len() {
        match path.points()[idx] {
            p if p.is_on_curve() => prev_point = p.point,
            p => {
                paint_ctx
                    .render_ctx
                    .draw_control_handle(prev_point, p.point);
                let p1 = path.points()[idx + 1].point;
                let p2 = path.points()[idx + 2].point;
                paint_ctx.render_ctx.draw_control_handle(p1, p2);
                idx += 2;
                prev_point = p2;
            }
        }
        idx += 1;
    }

    if let Some(trailing) = path.trailing() {
        if path.should_draw_trailing() {
            paint_ctx
                .render_ctx
                .draw_control_handle(prev_point, *trailing);
        }
    }
}

pub(crate) fn draw_paths(
    paths: &[Path],
    sels: &BTreeSet<PointId>,
    tool: &dyn Tool,
    ctx: &mut PaintCtx,
) {
    for path in paths {
        let bez = path.to_bezier();
        ctx.render_ctx.draw_path(&bez);
        draw_control_point_lines(path, ctx);
        ctx.render_ctx.draw_direction_indicator(&bez);

        let bez = path.to_bezier();
        for point in PointIter::new(path, &bez, sels) {
            ctx.render_ctx.draw_point(point)
        }

        if let Some(pt) = path.trailing() {
            if path.should_draw_trailing() {
                ctx.render_ctx.draw_off_curve_point(*pt, true);
            }
        }
    }

    if let Some(rect) = tool.selection_rect() {
        ctx.render_ctx.draw_selection_rect(rect);
    }
}

/// Return the tangent of the cubic bezier described by `(p0, p1, p2, p3)`
/// at time `t` as a vector relative to `p0`.
fn tangent_vector(t: f64, p0: Point, p1: Point, p2: Point, p3: Point) -> Vec2 {
    debug_assert!(t >= 0.0 && t <= 1.0);
    let one_minus_t = 1.0 - t;
    3.0 * one_minus_t.powi(2) * (p1 - p0)
        + 6.0 * t * one_minus_t * (p2 - p1)
        + 3.0 * t.powi(2) * (p3 - p2)
}

/// Create a line of length `len` perpendicular to the tangent of the cubic
/// bezier described by `(p0, p1, p2, p3)`, centered on `p0`.
fn cap_line(p0: Point, p1: Point, p2: Point, p3: Point, len: f64) -> Line {
    let tan_vec = tangent_vector(0.01, p0, p1, p2, p3);
    let end = p0 + tan_vec;
    perp(p0, end, len)
}

/// Create a line perpendicular to the line `(p1, p2)`, centered on `p1`.
fn perp(p0: Point, p1: Point, len: f64) -> Line {
    let perp_vec = Vec2::new(p0.y - p1.y, p1.x - p0.x);
    let norm_perp = perp_vec / perp_vec.hypot();
    let p2 = p0 + (len * -0.5) * norm_perp;
    let p3 = p0 + (len * 0.5) * norm_perp;
    Line::new(p2, p3)
}

//FIXME: remove when this gets added to kurbo
fn reverse_seg(seg: &PathSeg) -> PathSeg {
    match seg {
        PathSeg::Line(Line { p0, p1 }) => PathSeg::Line(Line::new(*p1, *p0)),
        PathSeg::Cubic(c) => PathSeg::Cubic(CubicBez::new(c.p3, c.p2, c.p1, c.p0)),
        PathSeg::Quad(q) => PathSeg::Quad(QuadBez::new(q.p2, q.p1, q.p0)),
    }
}

fn cubic_points_for_seg(seg: &PathSeg) -> (Point, Point, Point, Point) {
    match seg {
        PathSeg::Line(l) => (l.p0, l.p0, l.p1, l.p1),
        PathSeg::Cubic(c) => (c.p0, c.p1, c.p2, c.p3),
        PathSeg::Quad(q) => {
            let c = q.raise();
            (c.p0, c.p1, c.p2, c.p3)
        }
    }
}

fn make_arrow() -> BezPath {
    let mut bez = BezPath::new();
    //bez.move_to((-5., 0.));
    //bez.line_to((5., 0.));
    //bez.line_to((5., 11.));
    //bez.line_to((15., 11.));
    //bez.line_to((0., 32.));
    //bez.line_to((-15., 11.));
    //bez.line_to((-5., 11.));
    //bez.close_path();

    bez.move_to((0., 18.));
    bez.line_to((-12., 0.));
    bez.line_to((12., 0.));
    bez.close_path();
    bez
}
