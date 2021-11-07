//! The Bee!
//!
//! Follow the users touch! Mine the flowers! Do your best little bee! We are
//! rooting for you!

use crate::prelude::*;
use legion::system;
use macroquad::prelude::*;

/// The bees stats
#[derive(Clone, Copy, Debug, PartialEq)]
struct Bee;

pub fn roll_call(world: &mut legion::world::World, systems: &mut legion::systems::Builder) {
    world.push((Bee {}, Position::middle()));
    systems.add_system(draw_system());
}

#[system(for_each)]
fn draw(_: &mut Bee, pos: &Position) {
    let middle = (screen_width() / 2., screen_height() / 2.);
    draw_circle(middle.0, middle.1, 60., BLACK);
}
