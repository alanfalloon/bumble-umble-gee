//! The meadow. The stage where it all takes place.
//!
//! Give us somewhere to frolic!

use macroquad::rand::gen_range;

use crate::prelude::*;

/// The meadow
#[derive(Clone, Copy, Debug, PartialEq)]

pub struct Meadow {
    pub size: Vec2,
}
impl Meadow {
    pub fn new(h: f32, w: f32) -> Meadow {
        Meadow { size: vec2(w, h) }
    }
    pub fn clamp(&self, point: Vec2) -> Vec2 {
        point.clamp(Vec2::ZERO, self.size)
    }
}
/// A flower
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Flower {
    color: Color,
    pub radius: f32,
    pub collected: bool,
}

pub fn roll_call(
    world: &mut legion::world::World,
    systems: &mut legion::systems::Builder,
    resources: &mut legion::systems::Resources,
) {
    resources.insert(Meadow::new(1_000., 1_000.));
    for _ in 0..100 {
        let pos = Vec2::new(gen_range(0., 1_000.), gen_range(0., 1_000.));
        let color = Color::new(
            gen_range(0.2, 1.),
            gen_range(0., 0.1),
            gen_range(0.2, 1.),
            1.,
        );
        let radius = gen_range(10., 20.);
        world.push((
            Flower {
                color,
                radius,
                collected: false,
            },
            Position::from(pos),
        ));
    }
    systems
        .add_system(update_position_system())
        .add_system(draw_ground_system())
        .flush()
        .add_system(draw_flower_system())
        .flush();
}

#[system(for_each)]
fn update_position(pos: &mut Position, vel: &Velocity, #[resource] clock: &GameClock) {
    let Position(p) = *pos;
    let Velocity(v) = *vel;
    *pos = Position::from(p + v * clock.tick.as_secs_f32())
}

#[system]
fn draw_ground(#[resource] _: &Meadow) {
    clear_background(Color::new(0.58, 0.78, 0.58, 1.00));
}

#[system(for_each)]
fn draw_flower(flower: &Flower, pos: &Position) {
    let Position(pos) = pos;
    draw_circle(pos.x, pos.y, flower.radius, flower.color);
    draw_circle(
        pos.x,
        pos.y,
        flower.radius / 4.,
        if flower.collected { BLACK } else { YELLOW },
    );
}
