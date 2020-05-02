use std::sync::Arc;

use serde::{Serialize, Deserialize};
use tokio::sync::{RwLock, Mutex, MutexGuard};

use super::state::{TurtleState, DrawingState};
use super::renderer::display_list::PrimHandle;

/// The unique ID of a particular turtle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TurtleId(usize);

#[derive(Default, Debug)]
pub struct TurtleDrawings {
    pub state: TurtleState,
    pub drawings: Vec<PrimHandle>,
}

/// The entire state of the application, shared between threads in the server
#[derive(Default, Debug)]
pub struct App {
    /// The current state of the drawing
    drawing: Mutex<DrawingState>,
    /// Each `TurtleId` indexes into this field
    ///
    /// Note that we have to be very careful deleting from this field since we don't want to
    /// invalidate any `TurtleId` or cloned `Arc<Mutex<TurtleDrawings>>`.
    ///
    /// The outer `RwLock` makes it possible to push into the `Vec` using `write` and also `clone`
    /// multiple items in the `Vec` at the same time using `read`.
    turtles: RwLock<Vec<Arc<Mutex<TurtleDrawings>>>>,
}

impl App {
    /// Adds a new turtle to the application state, returning its `TurtleId`
    pub async fn add_turtle(&self) -> TurtleId {
        let mut turtles = self.turtles.write().await;
        let id = TurtleId(turtles.len());
        turtles.push(Default::default());
        id
    }

    /// Returns a mutable handle to the drawing state
    pub async fn drawing_mut(&self) -> MutexGuard<'_, DrawingState> {
        self.drawing.lock().await
    }

    /// Returns a handle to a the state and drawings of the given turtle
    pub async fn turtle(&self, id: TurtleId) -> Arc<Mutex<TurtleDrawings>> {
        let TurtleId(index) = id;
        let turtles = self.turtles.read().await;
        turtles[index].clone()
    }
}
