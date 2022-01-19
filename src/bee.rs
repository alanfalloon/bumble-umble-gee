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
use legion::{system, world::SubWorld, EntityStore};
use macroquad::prelude::*;
use parry2d::{math::Isometry, query::intersection_test, shape::Ball};

/// This is the bees sprite rect translated so the bee position is at the
/// origin.
const BEE_SPRITE: Rect = Rect {
    x: -(spritesheet::BEE_FLYING_FRAME_SIZE.x as f32 * 0.85),
    y: -(spritesheet::BEE_FLYING_FRAME_SIZE.y as f32 * 0.5),
    w: spritesheet::BEE_FLYING_FRAME_SIZE.x as f32,
    h: spritesheet::BEE_FLYING_FRAME_SIZE.y as f32,
};
/// This is the Bees hitbox relative to the bee position. To calculate the
/// final transformed hitbox, it must go through the same transformations as the
/// sprite itself.
const BEE_HITBOX: Rect = Rect {
    x: BEE_SPRITE.x,
    y: -BEE_SPRITE.h / 6.0,
    w: -BEE_SPRITE.x,
    h: BEE_SPRITE.h / 3.0,
};

/// The bees stats
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bee {
    destination: Vec2,
    thrust: Vec2,
    score: u32,
}
impl Bee {
    pub fn transform_rect(&self, pos: Vec2, settings: &Settings, rect: &Rect) -> Quad {
        Quad::from_rect(rect)
            .scale_to_origin(settings.bee_size / 1000.)
            .rotate_to(self.thrust.normalize())
            .translate(pos)
    }
}

pub fn roll_call(
    world: &mut legion::world::World,
    systems: &mut legion::systems::Builder,
    resources: &mut legion::systems::Resources,
) {
    let entity = {
        let meadow = resources.get::<Meadow>().expect("No meadow");
        world.push((
            Bee {
                destination: meadow.rand_pos(),
                thrust: Vec2::default(),
                score: 0,
            },
            Position::from(meadow.rand_pos()),
            Velocity::default(),
        ))
    };
    resources.insert(TheBee { entity });
    systems.add_system(update_destination_system());
    systems.add_system(head_for_destination_system());
    systems.add_system(fly_system());
    systems.add_system(found_flower_system());
    systems.add_system(draw_system());
    systems.add_system(draw_score_system());
}

#[system(for_each)]
fn update_destination(bee: &mut Bee, #[resource] inputs: &Inputs, #[resource] meadow: &Meadow) {
    if let Some(mouse_pos) = inputs.mouse_click {
        bee.destination = meadow.clamp(mouse_pos);
    }
}

#[system(for_each)]
fn head_for_destination(bee: &mut Bee, pos: &Position, #[resource] settings: &Settings) {
    let disp = bee.destination - pos.0;
    let dist = disp.length();
    bee.thrust = if dist > settings.max_thrust {
        (disp / dist) * settings.max_thrust
    } else {
        disp
    }
}

#[system(for_each)]
fn fly(
    bee: &Bee,
    vel: &mut Velocity,
    #[resource] clock: &GameClock,
    #[resource] settings: &Settings,
) {
    let Velocity(v) = *vel;
    // Add in a bit of drag, with a bit of random walk thrown in
    let wind = -v;
    let thrust = bee.thrust + wind * settings.wind_resistance / 100.;
    *vel = Velocity::from(v + clock.tick.as_secs_f32() * thrust / settings.mass);
}

#[system]
#[write_component(Bee)]
#[write_component(Flower)]
#[read_component(Position)]
fn found_flower(
    world: &mut SubWorld,
    #[resource] the_bee: &TheBee,
    #[resource] meadow: &Meadow,
    #[resource] settings: &Settings,
) {
    let bee = world.entry_ref(the_bee.entity).expect("Bee missing");
    let Position(bee_pos) = *bee.get_component::<Position>().expect("Bee missing pos");
    let bee = *bee.get_component::<Bee>().expect("Bee missing bee data");
    let hitbox = bee.transform_rect(bee_pos, settings, &BEE_HITBOX);
    let bee_shape = hitbox.polyline();
    let identity = Isometry::identity();
    let mut score_delta = 0u32;
    for index in meadow.flower_index_within(hitbox.bb()) {
        let mut flower_entry = world
            .entry_mut(meadow.flower_entities[index])
            .expect("flower disappeared");
        let Position(flower_pos) = *flower_entry
            .get_component::<Position>()
            .expect("Flower missing pos");
        let flower = flower_entry
            .get_component_mut::<Flower>()
            .expect("Flower missing flower data");
        if flower.collected {
            continue;
        }
        let flower_shape = Ball::new(flower.radius);
        let flower_isometry = Isometry::translation(flower_pos.x, flower_pos.y);
        if intersection_test(&identity, &bee_shape, &flower_isometry, &flower_shape).unwrap() {
            flower.collected = true;
            score_delta += 1;
        }
    }
    if score_delta > 0 {
        let mut bee = world.entry_mut(the_bee.entity).expect("Bee missing");
        let bee = bee
            .get_component_mut::<Bee>()
            .expect("Bee missing bee data");
        bee.score += score_delta;
    }
}

#[system(for_each)]
fn draw(
    bee: &Bee,
    pos: &Position,
    #[resource] clock: &GameClock,
    #[resource] settings: &Settings,
    #[resource] texture: &Texture2D,
) {
    let frame_num = ((settings.animation_speed as f64 * clock.time) as usize)
        .rem(spritesheet::BEE_FLYING_FRAMES.len());
    let animation_frame = &spritesheet::BEE_FLYING_FRAMES[frame_num];
    let Position(pos) = *pos;
    let points = bee.transform_rect(pos, settings, &BEE_SPRITE);
    points.draw_sprite(*texture, animation_frame.uv, WHITE);
    #[cfg(feature = "wireframes")]
    {
        points.draw_sides(0.5, YELLOW);
        let hitbox = bee.transform_rect(pos, settings, &BEE_HITBOX);
        hitbox.draw_sides(0.5, RED);
        draw_circle_lines(pos.x, pos.y, 1., 0.5, YELLOW);
        draw_circle_lines(bee.destination.x, bee.destination.y, 2., 0.5, MAGENTA);
    }
}

#[system(for_each)]
fn draw_score(
    bee: &Bee,
    #[resource] settings: &Settings,
    #[resource] camera: &crate::camera::Camera,
) {
    let Rect { x, y, w, h } = camera.rect;
    let font = Font::default();
    let font_size = (settings.font_size / 10.) as u16;
    let font_scale = vec2(w, h).length() / settings.max_zoom;
    let text = &format!("{}", bee.score);
    let TextDimensions {
        width, offset_y, ..
    } = measure_text(text, Some(font), font_size, font_scale);
    let pad = 2.0;
    let settings_x = font_scale * (settings.score_x_offset - 500.);
    let settings_y = font_scale * (settings.score_y_offset - 500.);
    let x = x + w - width - pad + settings_x;
    let y = y + offset_y + pad + settings_y;
    let params = TextParams {
        font,
        font_size,
        font_scale,
        font_scale_aspect: 1.0,
        color: WHITE,
    };
    draw_text_ex(text, x, y, params);
}
