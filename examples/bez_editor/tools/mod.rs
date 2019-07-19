use std::any::Any;
use std::fmt::Debug;

use super::Contents;
use druid::kurbo::{Point, Rect};
use druid::{Data, Event};

mod pen;
mod select;

pub use pen::Pen;
pub use select::Select;

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
    /// whether or not the shift key is downpick 5ca86ff fixup corner point drawing.
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
