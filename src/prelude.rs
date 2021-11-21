//! Where to stick the common vocabulary
pub use legion::{system, systems::Builder, Resources, Schedule, World};
pub use macroquad::prelude::*;
pub use std::time::Duration;

/// A location
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Position(pub Vec2);
impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position(Vec2::new(x, y))
    }
    pub fn middle() -> Position {
        Position::from(Self::far_corner().0 / 2.)
    }
    pub fn far_corner() -> Position {
        Self::new(screen_width(), screen_height())
    }
}
impl From<Vec2> for Position {
    fn from(position: Vec2) -> Self {
        Position(position)
    }
}

// A velocity
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Velocity(pub Vec2);
impl From<Vec2> for Velocity {
    fn from(velocity: Vec2) -> Self {
        Velocity(velocity)
    }
}

// Inputs. Loaded as a resource
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Inputs {
    pub mouse_click: Option<Vec2>,
}
/// Clock. Loaded as a resource
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct GameClock {
    pub time: f64,
    pub tick: Duration,
}
