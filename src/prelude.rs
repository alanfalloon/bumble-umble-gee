//! Where to stick the common vocabulary
pub use crate::settings::Settings;
use legion::Entity;
pub use legion::{system, systems::Builder, Resources, Schedule, World};
pub use macroquad::prelude::*;
pub use std::time::Duration;

/// A location
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Position(pub Vec2);
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

/// The bees resource
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TheBee {
    pub entity: Entity,
}
