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

use std::any::Any;
use std::collections::BTreeSet;
use std::fmt::Debug;

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
mod pen;
mod select;
mod toolbar;

use draw::draw_paths;
use path::{Path, PathPoint, PointId};
use pen::Pen;
use select::Select;
use toolbar::{Toolbar, ToolbarState};

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
    toolbar: ToolbarState,
}

impl CanvasState {
    fn new() -> Self {
        CanvasState {
            tool: Box::new(Pen::new()),
            contents: Contents::default(),
            toolbar: ToolbarState::basic(),
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

        let Contents {
            paths, selection, ..
        } = self;
        for point in selection.iter().cloned() {
            if let Some(path) = Arc::make_mut(paths).iter_mut().find(|p| **p == point) {
                path.nudge_point(point, nudge);
            }
        }
    }

    pub(crate) fn delete_selection(&mut self) {
        let to_delete = std::mem::replace(self.selection_mut(), BTreeSet::new());
        for sel in to_delete.iter() {
            if let Some(path) = self.paths_mut().iter_mut().find(|p| *p == sel) {
                path.delete_point(*sel);
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
            .map(|path| path.next_point(id))
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
            .map(|path| path.prev_point(id))
            .unwrap_or(id);
        self.selection_mut().insert(id);
    }

    pub(crate) fn update_for_drag(&mut self, _start: Point, end: Point) {
        self.active_path_mut().unwrap().update_for_drag(end);
    }

    pub(crate) fn iter_points(&self) -> impl Iterator<Item = &PathPoint> {
        self.paths.iter().flat_map(|p| p.points().iter())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Mouse {
    Down(Point),
    Drag {
        start: Point,
        last: Point,
        current: Point,
    },
    Up(Point),
}

/// A trait for editor tools (selection, pen, etc). More concretely, this abstracts
/// away different sets of mouse and keyboard handling behaviour.
pub(crate) trait Tool: Debug + Any {
    /// Called when the tool should process some event. The tool should modify
    /// `data` as necessary, and return `true` if the event is handled.
    fn event(&mut self, data: &mut Contents, event: &Event) -> bool;

    /// The current rectangular selection, if this is the selection tool, and
    /// whether or not the shift key is down.
    fn selection_rect(&self) -> Option<Rect> {
        None
    }

    fn boxed_clone(&self) -> Box<dyn Tool>;
    //TODO: this doesn't work; remove me, probably make tool an `enum`.
    fn same_impl(&self, _other: &dyn Any) -> bool {
        false
    }
    fn name(&self) -> &str;
}

impl Clone for Box<dyn Tool> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}

impl Data for Box<dyn Tool> {
    fn same(&self, other: &Box<dyn Tool>) -> bool {
        self.same_impl(other)
    }
}

// It should be able to get this from a derive macro.
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
        let CanvasState { tool, contents, .. } = data;
        if ctx.is_handled() || tool.event(contents, event) {
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
