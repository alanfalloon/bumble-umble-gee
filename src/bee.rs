//! The Bee!
//!
//! Follow the users touch! Mine the flowers! Do your best little bee! We are
//! rooting for you!

use crate::prelude::*;
use legion::system;
use macroquad::prelude::*;

/// The bees stats
#[derive(Clone, Copy, Debug, PartialEq)]
struct Bee {
    destination: Vec2D,
    thrust: Vec2D,
    mass: f32,
    max_thrust: f32,
}

pub fn roll_call(world: &mut legion::world::World, systems: &mut legion::systems::Builder) {
    let middle = Position::middle();
    world.push((
        Bee {
            destination: middle.0 / 2.,
            thrust: Vec2D::default(),
            mass: 1.,
            max_thrust: 100.,
        },
        middle,
        Velocity::default(),
    ));
    systems.add_system(update_destination_system());
    systems.add_system(head_for_destination_system());
    systems.add_system(fly_system());
    systems.add_system(draw_system());
}

#[system(for_each)]
fn update_destination(bee: &mut Bee, #[resource] inputs: &Inputs) {
    if let Some(mouse_pos) = inputs.mouse_click {
        bee.destination = mouse_pos;
    }
}

#[system(for_each)]
fn head_for_destination(bee: &mut Bee, pos: &Position) {
    let disp = bee.destination - pos.0;
    let dist = disp.length();
    bee.thrust = if dist > bee.max_thrust {
        (disp / dist) * bee.max_thrust
    } else {
        disp
    }
}

#[system(for_each)]
fn fly(bee: &Bee, vel: &mut Velocity, #[resource] tick: &Duration) {
    let Velocity(v) = *vel;
    // Add in a bit of drag
    let thrust = bee.thrust + (v * -0.5);
    *vel = Velocity::from(v + thrust * bee.mass * tick.as_secs_f32());
}

#[system(for_each)]
fn draw(bee: &Bee, pos: &Position) {
    let Position(pos) = *pos;
    let heading = bee.thrust.normalize();
    let starboard = Vec2D::new(-heading.y, heading.x);
    let head_pos = pos + heading * 6.;
    let sting_pos = pos - heading * 8.;
    let right_wing_pos = pos + starboard * 7.;
    let left_wing_pos = pos - starboard * 7.;
    let wing_color = Color { a: 0.6, ..SKYBLUE };
    draw_line(pos.x, pos.y, sting_pos.x, sting_pos.y, 2., BLACK);
    draw_circle(pos.x, pos.y, 6., YELLOW);
    draw_circle(head_pos.x, head_pos.y, 3., BLACK);
    draw_circle(right_wing_pos.x, right_wing_pos.y, 5., wing_color);
    draw_circle(left_wing_pos.x, left_wing_pos.y, 5., wing_color);
}
