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
    let max = Position::far_corner().position;
    for _ in 0..100 {
        let pos = Vec::new(gen_range(0., max.x), gen_range(0., max.y));
        let color = Color::new(gen_range(0., 1.), gen_range(0., 1.), gen_range(0., 1.), 1.);
        let radius = gen_range(15., 35.);
        world.push((Flower { color, radius }, Position::from(pos)));
    }
    systems
        .add_system(ground_system())
        .flush()
        .add_system(flower_system())
        .flush();
}

#[system(for_each)]
fn ground(_: &Meadow) {
    clear_background(Color::new(0.58, 0.78, 0.58, 1.00));
}

#[system(for_each)]
fn flower(flower: &Flower, pos: &Position) {
    draw_circle(pos.position.x, pos.position.y, flower.radius, flower.color);
}
