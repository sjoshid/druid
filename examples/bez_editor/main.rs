// Copyright 2018 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A simple bezier path editor.

use std::collections::BTreeSet;

use druid::kurbo::{Point, Rect, Size, Vec2};
use druid::piet::{Color, RenderContext};
use druid::shell::window::Cursor;
use druid::shell::{runloop, WindowBuilder};
use std::sync::Arc;

use druid::{
    Action, BaseState, BoxConstraints, Data, Env, Event, EventCtx, KeyCode, LayoutCtx, PaintCtx,
    UiMain, UiState, UpdateCtx, Widget, WidgetPod,
};

mod draw;
mod path;
mod toolbar;
mod tools;

use draw::draw_paths;
use path::{Path, PathPoint, PointId};
use toolbar::{Toolbar, ToolbarState};
use tools::{Mouse, Pen, Select, Tool};

const BG_COLOR: Color = Color::rgb24(0xfb_fb_fb);
const TOOLBAR_POSITION: Point = Point::new(8., 8.);

pub(crate) const MIN_POINT_DISTANCE: f64 = 10.0;

struct Canvas {
    toolbar: WidgetPod<ToolbarState, Toolbar>,
}

impl Canvas {
    fn new() -> Self {
        Canvas {
            toolbar: WidgetPod::new(Toolbar::default()),
        }
    }
}

#[derive(Debug, Clone)]
struct CanvasState {
    tool: Box<dyn Tool>,
    /// The paths in the canvas
    contents: Contents,
    mouse: Mouse,
    toolbar: ToolbarState,
}

impl CanvasState {
    fn new() -> Self {
        CanvasState {
            tool: Box::new(Pen::new()),
            contents: Contents::default(),
            toolbar: ToolbarState::basic(),
            mouse: Mouse::new(),
        }
    }

    fn update_tool_if_necessary(&mut self) {
        if self.toolbar.selected_item().name == self.tool.name() {
            return;
        }

        let new_tool: Box<dyn Tool> = match self.toolbar.selected_item().name.as_str() {
            "pen" => Box::new(Pen::new()),
            _ => Box::new(Select::new()),
        };
        self.tool = new_tool;
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Contents {
    next_path_id: usize,
    paths: Arc<Vec<Path>>,
    /// Selected points, including the path index and the point id.
    selection: Arc<BTreeSet<PointId>>,
}

/// A helper for iterating through a selection in per-path chunks.
struct PathSelection {
    inner: Vec<PointId>,
}

impl PathSelection {
    fn new(src: &BTreeSet<PointId>) -> PathSelection {
        let mut inner: Vec<_> = src.iter().copied().collect();
        inner.sort();
        PathSelection { inner }
    }

    fn iter(&self) -> PathSelectionIter {
        PathSelectionIter {
            inner: &self.inner,
            idx: 0,
        }
    }
}

struct PathSelectionIter<'a> {
    inner: &'a [PointId],
    idx: usize,
}

impl<'a> Iterator for PathSelectionIter<'a> {
    type Item = &'a [PointId];
    fn next(&mut self) -> Option<&'a [PointId]> {
        if self.idx >= self.inner.len() {
            return None;
        }
        let path_id = self.inner[self.idx].path;
        let end_idx = self.inner[self.idx..]
            .iter()
            .position(|p| p.path != path_id)
            .map(|idx| idx + self.idx)
            .unwrap_or(self.inner.len());
        let range = self.idx..end_idx;
        self.idx = end_idx;
        // probably unnecessary, but we don't expect empty slices
        if range.start == range.end {
            None
        } else {
            Some(&self.inner[range])
        }
    }
}

impl Contents {
    pub(crate) fn paths_mut(&mut self) -> &mut Vec<Path> {
        Arc::make_mut(&mut self.paths)
    }

    pub(crate) fn selection_mut(&mut self) -> &mut BTreeSet<PointId> {
        Arc::make_mut(&mut self.selection)
    }

    /// Return the index of the path that is currently drawing. To be currently
    /// drawing, there must be a single currently selected point.
    fn active_path_idx(&self) -> Option<usize> {
        if self.selection.len() == 1 {
            let active = self.selection.iter().next().unwrap();
            self.paths.iter().position(|p| *p == *active)
        } else {
            None
        }
    }

    fn path_for_point_mut(&mut self, point: PointId) -> Option<&mut Path> {
        self.paths_mut().iter_mut().find(|p| **p == point)
    }

    pub(crate) fn active_path_mut(&mut self) -> Option<&mut Path> {
        match self.active_path_idx() {
            Some(idx) => self.paths_mut().get_mut(idx),
            None => None,
        }
    }

    pub(crate) fn active_path(&self) -> Option<&Path> {
        match self.active_path_idx() {
            Some(idx) => self.paths.get(idx),
            None => None,
        }
    }

    pub(crate) fn new_path(&mut self, start: Point) {
        let path = Path::new(start);
        let point = path.points()[0].id;

        self.paths_mut().push(path);
        self.selection_mut().clear();
        self.selection_mut().insert(point);
    }

    pub(crate) fn add_point(&mut self, point: Point) {
        if self.active_path_idx().is_none() {
            self.new_path(point);
        } else {
            let new_point = self.active_path_mut().unwrap().append_point(point);
            self.selection_mut().clear();
            self.selection_mut().insert(new_point);
        }
    }

    pub(crate) fn nudge_selection(&mut self, nudge: Vec2) {
        if self.selection.is_empty() {
            return;
        }

        let to_nudge = PathSelection::new(&self.selection);
        for path_points in to_nudge.iter() {
            if let Some(path) = self.path_for_point_mut(path_points[0]) {
                path.nudge_points(path_points, nudge);
            }
        }
    }

    pub(crate) fn delete_selection(&mut self) {
        let to_delete = PathSelection::new(&self.selection);
        self.selection_mut().clear();
        for path_points in to_delete.iter() {
            if let Some(path) = self.path_for_point_mut(path_points[0]) {
                path.delete_points(path_points);
            }
        }
        self.paths_mut().retain(|p| !p.points().is_empty());
    }

    pub(crate) fn select_all(&mut self) {
        *self.selection_mut() = self.iter_points().map(|p| p.id).collect();
    }

    pub(crate) fn select_next(&mut self) {
        if self.selection.len() != 1 {
            return;
        }
        let id = self.selection.iter().next().copied().unwrap();
        self.selection_mut().clear();
        let id = self
            .paths
            .iter()
            .find(|p| **p == id)
            .map(|path| path.next_point(id).id)
            .unwrap_or(id);
        self.selection_mut().insert(id);
    }

    pub(crate) fn select_prev(&mut self) {
        if self.selection.len() != 1 {
            return;
        }
        let id = self.selection.iter().next().copied().unwrap();
        self.selection_mut().clear();
        let id = self
            .paths
            .iter()
            .find(|p| **p == id)
            .map(|path| path.prev_point(id).id)
            .unwrap_or(id);
        self.selection_mut().insert(id);
    }

    pub(crate) fn select_path(&mut self, point: Point, toggle: bool) -> bool {
        let path_idx = match self.paths.iter().position(|p| {
            let (_, x, y) = p.bezier().nearest(point, 0.1);
            Point::new(x, y).to_vec2().hypot() < MIN_POINT_DISTANCE
        }) {
            Some(idx) => idx,
            None => return false,
        };

        let points: Vec<_> = self.paths[path_idx].points().to_owned();
        for point in points {
            if !self.selection_mut().insert(point.id) && toggle {
                self.selection_mut().remove(&point.id);
            }
        }
        true
    }

    pub(crate) fn update_for_drag(&mut self, drag_point: Point) {
        self.active_path_mut().unwrap().update_for_drag(drag_point);
    }

    pub(crate) fn iter_points(&self) -> impl Iterator<Item = &PathPoint> {
        self.paths.iter().flat_map(|p| p.points().iter())
    }

    /// If there is a single on curve point selected, toggle it between corner and smooth
    pub(crate) fn toggle_selected_on_curve_type(&mut self) {
        if self.selection.len() == 1 {
            let point = self.selection.iter().copied().next().unwrap();
            let path = self.active_path_mut().unwrap();
            path.toggle_on_curve_point_type(point);
        }
    }
}

impl Data for CanvasState {
    fn same(&self, other: &Self) -> bool {
        self.contents.same(&other.contents) && self.toolbar.same(&other.toolbar)
        //&& self.tool == other.tool
    }
}

impl Data for Contents {
    fn same(&self, other: &Self) -> bool {
        self.paths.same(&other.paths) && self.selection.same(&other.selection)
    }
}

impl Widget<CanvasState> for Canvas {
    fn paint(
        &mut self,
        paint_ctx: &mut PaintCtx,
        _base: &BaseState,
        data: &CanvasState,
        _env: &Env,
    ) {
        paint_ctx.render_ctx.clear(BG_COLOR);
        draw_paths(
            &data.contents.paths,
            &data.contents.selection,
            &*data.tool,
            paint_ctx,
        );
        self.toolbar
            .paint_with_offset(paint_ctx, &data.toolbar, _env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &CanvasState,
        env: &Env,
    ) -> Size {
        let toolbar_size = self.toolbar.layout(ctx, bc, &data.toolbar, env);
        self.toolbar
            .set_layout_rect(Rect::from_origin_size(TOOLBAR_POSITION, toolbar_size));
        bc.max()
    }

    fn event(
        &mut self,
        event: &Event,
        ctx: &mut EventCtx,
        data: &mut CanvasState,
        _env: &Env,
    ) -> Option<Action> {
        // first check for top-level commands
        match event {
            Event::KeyUp(key) if key.key_code == KeyCode::Escape => {
                data.contents.selection_mut().clear();
                ctx.set_handled();
            }
            Event::KeyUp(key) if data.toolbar.idx_for_key(key).is_some() => {
                let idx = data.toolbar.idx_for_key(key).unwrap();
                data.toolbar.set_selected(idx);
                ctx.set_handled();
            }
            other => {
                self.toolbar.event(other, ctx, &mut data.toolbar, _env);
            }
        }

        // then pass the event to the active tool
        let CanvasState {
            tool,
            contents,
            mouse,
            ..
        } = data;
        if ctx.is_handled()
            || match event {
                Event::KeyDown(k) => tool.key_down(contents, k),
                Event::MouseUp(m) => mouse.mouse_up(contents, m.clone(), tool),
                Event::MouseMoved(m) => mouse.mouse_moved(contents, m.clone(), tool),
                Event::MouseDown(m) => mouse.mouse_down(contents, m.clone(), tool),
                _ => false,
            }
        {
            ctx.invalidate();
        }

        data.update_tool_if_necessary();
        None
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old: Option<&CanvasState>,
        new: &CanvasState,
        _env: &Env,
    ) {
        // update the mouse icon if the active tool has changed
        let old = match old {
            Some(old) => old,
            None => return,
        };

        if old.toolbar.selected_idx() != new.toolbar.selected_idx() {
            match new.toolbar.selected_item().name.as_str() {
                "select" => ctx.window().set_cursor(&Cursor::Arrow),
                "pen" => ctx.window().set_cursor(&Cursor::Crosshair),
                other => eprintln!("unknown tool '{}'", other),
            }
            ctx.invalidate();
        }
        self.toolbar.update(ctx, &new.toolbar, _env);
    }
}

fn main() {
    druid_shell::init();

    let mut run_loop = runloop::RunLoop::new();
    let mut builder = WindowBuilder::new();
    let state = CanvasState::new();
    let mut state = UiState::new(Canvas::new(), state);
    state.set_active(true);
    builder.set_title("Paths");
    builder.set_handler(Box::new(UiMain::new(state)));
    let window = builder.build().unwrap();
    window.show();
    run_loop.run();
}
