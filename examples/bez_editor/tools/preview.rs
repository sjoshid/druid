use super::{Drag, MouseDelegate, Tool};
use crate::{Contents, MIN_POINT_DISTANCE};

/// The state of the pen.
#[derive(Debug, Clone, PartialEq)]
pub struct Preview;

impl MouseDelegate<Contents> for Preview {
    fn cancel(&mut self, canvas: &mut Contents) {
        canvas.selection_mut().clear();
    }

    fn left_drag_changed(&mut self, canvas: &mut Contents, drag: Drag) -> bool {
        let delta = drag.current.pos - drag.prev.pos;
        canvas.vport.pan(delta);
        true
    }
}

impl Tool for Preview {
    fn boxed_clone(&self) -> Box<dyn Tool> {
        Box::new(self.clone())
    }

    fn name(&self) -> &str {
        "preview"
    }
}

impl Preview {
    pub(crate) fn new() -> Self {
        Preview
    }
}
