use crate::Point;

use super::super::state::Pen;

/// A drawing primitive
#[derive(Debug, Clone)]
pub enum DrawPrim {
}

/// A handle to one of the items in a display list
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrimHandle(usize);

/// A list of drawing primitives in the order that they are rendered
#[derive(Default, Debug)]
pub struct DisplayList {
    items: Vec<DrawPrim>,
}

impl DisplayList {
    pub fn push_line(&mut self, start: Point, end: Point, pen: &Pen) -> PrimHandle {
        //TODO: A `Point` is in logical coordiantes, whereas Vector2F
        todo!()
    }

    pub fn replace_line(&mut self, handle: PrimHandle, start: Point, end: Point, pen: &Pen) {
        todo!()
    }
}
