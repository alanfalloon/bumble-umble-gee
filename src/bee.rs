//! The Bee!
//!
//! Follow the users touch! Mine the flowers! Do your best little bee! We are
//! rooting for you!

use crate::{prelude::*, spritesheet};
use legion::system;
use macroquad::prelude::*;

/// The bees stats
#[derive(Clone, Copy, Debug, PartialEq)]
struct Bee {
    destination: Vec2,
    thrust: Vec2,
    mass: f32,
    max_thrust: f32,
    texture: Texture2D,
}

pub fn roll_call(world: &mut legion::world::World, systems: &mut legion::systems::Builder) {
    let middle = Position::middle();
    world.push((
        Bee {
            destination: middle.0 / 2.,
            thrust: Vec2::default(),
            mass: 1.,
            max_thrust: 100.,
            texture: Texture2D::from_file_with_format(
                crate::spritesheet::SPRITESHEET_PNG_BYTES,
                Some(ImageFormat::Png),
            ),
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
    let frame_rect = &spritesheet::BEE_FLYING_FRAMES[0];
    let frame_corner: Vec2 = UVec2::from(spritesheet::BEE_FLYING_FRAME_SIZE).as_f32() * 0.2;
    // Vertices and triangles:
    //  0 - 1
    //  | / |
    //  2 - 3
    let v1 = frame_corner;
    let v0 = Vec2::new(-v1.x, v1.y);
    let v2 = -v1;
    let v3 = -v0;
    let indices = vec![0, 1, 2, 1, 2, 3];
    let Position(pos) = *pos;
    let heading = bee.thrust.normalize();
    // Rotation matrix is:
    // | cos t; -sin t |
    // | sin t;  cos t |
    // Since `heading` is a unit rotation, then `cos t == heading.x`
    // and `sin t == heading.y`. I drew a picture in Notes.key.
    let rot = Mat2::from_cols_array_2d(&[[heading.x, heading.y], [-heading.y, heading.x]]);
    // UV space.
    let uv = Vec2::from(frame_rect.uv);
    let delta_uv = Vec2::from(spritesheet::BEE_FLYING_FRAME_UV);
    draw_mesh(&Mesh {
        vertices: vec![
            macroquad::models::Vertex {
                position: (rot * v0 + pos).extend(0.),
                uv: uv,
                color: WHITE,
            },
            macroquad::models::Vertex {
                position: (rot * v1 + pos).extend(0.),
                uv: uv + (delta_uv.x, 0.).into(),
                color: WHITE,
            },
            macroquad::models::Vertex {
                position: (rot * v2 + pos).extend(0.),
                uv: uv + (0., delta_uv.y).into(),
                color: WHITE,
            },
            macroquad::models::Vertex {
                position: (rot * v3 + pos).extend(0.),
                uv: uv + delta_uv,
                color: WHITE,
            },
        ],
        indices,
        texture: Some(bee.texture),
    });
}
