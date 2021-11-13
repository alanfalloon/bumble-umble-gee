//! The meadow. The stage where it all takes place.
//!
//! Give us somewhere to frolic!

use macroquad::rand::gen_range;

use crate::prelude::*;

/// The meadow
#[derive(Clone, Copy, Debug, PartialEq)]

struct Meadow {}
/// A flower
#[derive(Clone, Copy, Debug, PartialEq)]
struct Flower {
    color: Color,
    radius: f32,
}

pub fn roll_call(world: &mut legion::world::World, systems: &mut legion::systems::Builder) {
    world.push((Meadow {},));
    let max = Position::far_corner().0;
    for _ in 0..100 {
        let pos = Vec2::new(gen_range(0., max.x), gen_range(0., max.y));
        let color = Color::new(
            gen_range(0.2, 1.),
            gen_range(0., 0.1),
            gen_range(0.2, 1.),
            1.,
        );
        let radius = gen_range(15., 25.);
        world.push((Flower { color, radius }, Position::from(pos)));
    }
    systems
        .add_system(update_position_system())
        .add_system(draw_ground_system())
        .flush()
        .add_system(draw_flower_system())
        .flush();
}

#[system(for_each)]
fn update_position(pos: &mut Position, vel: &Velocity, #[resource] tick: &Duration) {
    let Position(p) = *pos;
    let Velocity(v) = *vel;
    *pos = Position::from(p + v * tick.as_secs_f32())
}

#[system(for_each)]
fn draw_ground(_: &Meadow) {
    clear_background(Color::new(0.58, 0.78, 0.58, 1.00));
}

#[system(for_each)]
fn draw_flower(flower: &Flower, pos: &Position) {
    let Position(pos) = pos;
    draw_circle(pos.x, pos.y, flower.radius, flower.color);
}
