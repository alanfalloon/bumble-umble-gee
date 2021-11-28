//! Where to stick the common vocabulary
pub use crate::settings::Settings;
use legion::Entity;
pub use legion::{system, systems::Builder, Resources, Schedule, World};
pub use macroquad::prelude::*;
use std::ops::Index;
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

/// A quadrilateral of points. Points appear clockwise.
pub struct Quad(pub [Vec2; 4]);

impl Quad {
    /// Vertices and triangles for tessellating a sprite:
    ///  0 - 1
    ///  | \ |
    ///  3 - 2
    const TESS_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

    /// Side indices
    const SIDE_INDICES: [(usize, usize); 4] = [(0, 1), (1, 2), (2, 3), (3, 0)];

    pub fn from_rect(rect: &Rect) -> Self {
        Quad([
            vec2(rect.left(), rect.top()),
            vec2(rect.right(), rect.top()),
            vec2(rect.right(), rect.bottom()),
            vec2(rect.left(), rect.bottom()),
        ])
    }

    pub fn scale_to_origin(&self, scale: f32) -> Self {
        Quad(array_init::array_init(|n| self.0[n] * scale))
    }

    pub fn translate(&self, offset: Vec2) -> Self {
        Quad(array_init::array_init(|n| self.0[n] + offset))
    }

    pub fn rotate_to(&self, unit: Vec2) -> Self {
        // Rotation matrix is:
        // | cos t; -sin t |
        // | sin t;  cos t |

        // Since `unit` is a unit vector, then  `cos t == unit.x`
        // and `sin t == unit.y`. So rotation matrix is:

        // | unit.x; -unit.y |
        // | unit.y;  unit.x |
        let rot = Mat2::from_cols_array_2d(&[[unit.x, unit.y], [-unit.y, unit.x]]);
        Quad(array_init::array_init(|n| rot * self.0[n]))
    }

    pub fn sides(&self) -> [(Vec2, Vec2); 4] {
        array_init::from_iter(
            Self::SIDE_INDICES
                .iter()
                .copied()
                .map(|(from, to)| (self.0[from], self.0[to])),
        )
        .unwrap()
    }

    pub fn draw_sprite(&self, texture: Texture2D, uv: Rect, color: Color) {
        let uv = Self::from_rect(&uv);
        let indices = Vec::from(Self::TESS_INDICES);
        let vertices: Vec<_> = {
            use macroquad::models::Vertex;
            (0..4)
                .into_iter()
                .map(|n| Vertex {
                    position: self[n].extend(0.),
                    uv: uv[n],
                    color,
                })
                .collect()
        };
        draw_mesh(&Mesh {
            vertices,
            indices,
            texture: Some(texture),
        });
    }

    pub fn draw_sides(&self, thickness: f32, color: Color) {
        for (from, to) in self.sides() {
            draw_line(from.x, from.y, to.x, to.y, thickness, color);
        }
    }
}
impl Index<usize> for Quad {
    type Output = Vec2;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
