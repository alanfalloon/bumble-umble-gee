//! Where to stick the common vocabulary
pub use legion::{system, systems::Builder, Resources, Schedule, World};
pub use macroquad::prelude::*;
pub use std::time::Duration;

pub struct WorldSpace;
pub type Vec2D = euclid::Vector2D<f32, WorldSpace>;

/// A location
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Position(pub Vec2D);
impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position(Vec2D::new(x, y))
    }
    pub fn middle() -> Position {
        Position::from(Self::far_corner().0 / 2.)
    }
    pub fn far_corner() -> Position {
        Self::new(screen_width(), screen_height())
    }
}
impl From<Vec2D> for Position {
    fn from(position: Vec2D) -> Self {
        Position(position)
    }
}

// A velocity
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Velocity(pub Vec2D);
impl From<Vec2D> for Velocity {
    fn from(velocity: Vec2D) -> Self {
        Velocity(velocity)
    }
}

// Inputs. Loaded as a resource
pub struct Inputs {
    pub mouse_click: Option<Vec2D>,
}
impl Inputs {
    pub fn grab() -> Self {
        let mouse_click = if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            Some(Vec2D::new(x, y))
        } else {
            None
        };
        Self { mouse_click }
    }
}
