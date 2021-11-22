//! The Bee!
//!
//! Follow the users touch! Mine the flowers! Do your best little bee! We are
//! rooting for you!

use std::ops::Rem;

use crate::{
    meadow::{Flower, Meadow},
    prelude::*,
    spritesheet,
};
use legion::{system, world::SubWorld, Entity, EntityStore, IntoQuery};
use macroquad::prelude::*;

/// The bees stats
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bee {
    destination: Vec2,
    thrust: Vec2,
    mass: f32,
    max_thrust: f32,
    texture: Texture2D,
}
/// The bees resource
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TheBee {
    pub entity: Entity,
}

pub fn roll_call(
    world: &mut legion::world::World,
    systems: &mut legion::systems::Builder,
    resources: &mut legion::systems::Resources,
) {
    let middle = Position::middle();
    let entity = world.push((
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
    resources.insert(TheBee { entity });
    systems.add_system(update_destination_system());
    systems.add_system(head_for_destination_system());
    systems.add_system(fly_system());
    systems.add_system(found_flower_system());
    systems.add_system(draw_system());
}

#[system(for_each)]
fn update_destination(bee: &mut Bee, #[resource] inputs: &Inputs, #[resource] meadow: &Meadow) {
    if let Some(mouse_pos) = inputs.mouse_click {
        bee.destination = meadow.clamp(mouse_pos);
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
fn fly(bee: &Bee, vel: &mut Velocity, #[resource] clock: &GameClock) {
    let Velocity(v) = *vel;
    // Add in a bit of drag, with a bit of random walk thrown in
    let wind = -v;
    let thrust = bee.thrust + wind * 0.7;
    *vel = Velocity::from(v + thrust * bee.mass * clock.tick.as_secs_f32());
}

#[system]
#[read_component(Bee)]
#[write_component(Flower)]
#[read_component(Position)]
fn found_flower(world: &mut SubWorld, #[resource] the_bee: &TheBee) {
    let bee = world.entry_ref(the_bee.entity).expect("Bee missing");
    let Position(bee_pos) = *bee.get_component::<Position>().expect("Bee missing pos");
    for (flower, Position(flower_pos)) in <(&mut Flower, &Position)>::query().iter_mut(world) {
        if bee_pos.distance_squared(*flower_pos) < flower.radius * flower.radius {
            flower.collected = true;
            return;
        }
    }
}

#[system(for_each)]
fn draw(bee: &Bee, pos: &Position, #[resource] clock: &GameClock) {
    let frame_num = ((10. * clock.time) as usize).rem(3);
    // Texture coordinates s*
    let Rect {
        x: sx,
        y: sy,
        w: sw,
        h: sh,
    } = spritesheet::BEE_FLYING_FRAMES[frame_num].xy;
    // Scale the texture
    let (w, h) = (vec2(sw, sh) * 0.2).into();
    // Find the midpoint for the rotation
    let Position(pos) = *pos;
    let (x, y) = pos.into();
    let points = {
        // half-width and half-height
        let hw = w / 2.;
        let hh = h / 2.;
        [
            vec2(x - hw, y - hh) - pos,
            vec2(x + hw, y - hh) - pos,
            vec2(x + hw, y + hh) - pos,
            vec2(x - hw, y + hh) - pos,
        ]
    };
    let texture_uv: [Vec2; 4] = {
        let tx = bee.texture.width();
        let ty = bee.texture.height();
        [
            vec2(sx / tx, sy / ty),
            vec2((sx + sw) / tx, sy / ty),
            vec2((sx + sw) / tx, (sy + sh) / ty),
            vec2(sx / tx, (sy + sh) / ty),
        ]
    };
    // Rotation matrix is:
    // | cos t; -sin t |
    // | sin t;  cos t |

    // Since `heading` is a unit rotation, then normally `cos t == heading.x`
    // and `sin t == heading.y`, *but* since these sprites treat "up" as
    // "forward" we need to adjust by rotating 90deg. I drew a picture in
    // Notes.key.
    let heading = bee.thrust.normalize().perp();
    // | heading.x; -heading.y |
    // | heading.y;  heading.x |
    let rot = Mat2::from_cols_array_2d(&[[heading.x, heading.y], [-heading.y, heading.x]]);
    let points: [_; 4] = array_init::array_init(|n| rot * points[n] + pos);
    // Vertices and triangles:
    //  0 - 1
    //  | \ |
    //  3 - 2
    let indices = vec![0, 1, 2, 0, 2, 3];
    let vertices: Vec<_> = {
        use macroquad::models::Vertex;
        (0..4)
            .into_iter()
            .map(|n| Vertex {
                position: points[n].extend(0.),
                uv: texture_uv[n],
                color: WHITE,
            })
            .collect()
    };
    draw_mesh(&Mesh {
        vertices,
        indices,
        texture: Some(bee.texture),
    });
    for (fr, to) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
        draw_line(
            points[fr].x,
            points[fr].y,
            points[to].x,
            points[to].y,
            1.,
            RED,
        );
    }
    // draw_circle_lines(x, y, 3., 1., BLUE);
    // let heading = heading * 10.;
    // draw_line(x, y, x + heading.x, y + heading.y, 1., BLUE);
    // let heading = bee.thrust.normalize() * 10.;
    // draw_line(x, y, x + heading.x, y + heading.y, 1., GREEN);
    draw_circle_lines(bee.destination.x, bee.destination.y, 2., 1., MAGENTA);
}
