//! Where to stick the common vocabulary

pub use legion::{system, systems::Builder, Resources, Schedule, World};
pub use macroquad::prelude::*;

pub struct WorldSpace;
pub type Vec = euclid::Vector2D<f32, WorldSpace>;

/// A location
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub position: Vec,
}
impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position {
            position: Vec::new(x, y),
        }
    }
    pub fn middle() -> Position {
        Position::from(Self::far_corner().position / 2.)
    }
    pub fn far_corner() -> Position {
        Self::new(screen_width(), screen_height())
    }
}
impl From<Vec> for Position {
    fn from(position: Vec) -> Self {
        Position { position }
    }
}
