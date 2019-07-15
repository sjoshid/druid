//! Drawing algorithms and helpers

use std::collections::BTreeSet;

use super::path::{Path, PointId, PointType};
use super::Tool;
use druid::kurbo::{Circle, Line, Point, Rect};
use druid::piet::{Color, FillRule::NonZero, RenderContext};
use druid::PaintCtx;

const PATH_COLOR: Color = Color::rgb24(0x00_00_00);
const SELECTION_RECT_BG_COLOR: Color = Color::rgba32(0xDD_DD_DD_55);
const SELECTION_RECT_STROKE_COLOR: Color = Color::rgb24(0x53_8B_BB);
const ON_CURVE_POINT_COLOR: Color = Color::rgb24(0x0b_2b_db);
const OFF_CURVE_POINT_COLOR: Color = Color::rgb24(0xbb_bb_bb);
const OFF_CURVE_HANDLE_COLOR: Color = Color::rgb24(0xbb_bb_bb);

const ON_CURVE_RADIUS: f64 = 3.5;
const ON_CURVE_SELECTED_RADIUS: f64 = 4.;
const OFF_CURVE_RADIUS: f64 = 2.;
const OFF_CURVE_SELECTED_RADIUS: f64 = 2.5;

trait PaintHelpers: RenderContext {
    fn draw_control_handle(&mut self, p1: Point, p2: Point) {
        let brush = self.solid_brush(OFF_CURVE_HANDLE_COLOR);
        let l = Line::new(p1, p2);
        self.stroke(l, &brush, 1.0, None);
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
}

impl<T: RenderContext> PaintHelpers for T {}

struct PointStyle {
    point: Point,
    typ: PointType,
    selected: bool,
}

struct PointIter<'a> {
    idx: usize,
    path: &'a Path,
    sels: &'a BTreeSet<PointId>,
}

impl<'a> PointIter<'a> {
    fn new(path: &'a Path, sels: &'a BTreeSet<PointId>) -> Self {
        PointIter { idx: 0, path, sels }
    }
}

impl<'a> std::iter::Iterator for PointIter<'a> {
    type Item = PointStyle;
    fn next(&mut self) -> Option<PointStyle> {
        match self.path.points().get(self.idx) {
            None => None,
            Some(path_point) => {
                let typ = path_point.typ;
                let selected = self.sels.contains(&path_point.id);
                self.idx += 1;
                Some(PointStyle {
                    point: path_point.point,
                    typ,
                    selected,
                })
            }
        }
    }
}

pub(crate) fn draw_inactive_path(path: &Path, paint_ctx: &mut PaintCtx) {
    let path_brush = paint_ctx.render_ctx.solid_brush(PATH_COLOR);
    let bez = path.to_bezier();
    paint_ctx.render_ctx.stroke(bez, &path_brush, 1.0, None);
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
        draw_inactive_path(path, ctx);
        draw_control_point_lines(path, ctx);

        for PointStyle {
            point,
            typ,
            selected,
        } in PointIter::new(path, sels)
        {
            if typ.is_on_curve() {
                ctx.render_ctx.draw_on_curve_point(point, selected);
            } else {
                ctx.render_ctx.draw_off_curve_point(point, selected);
            }
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
