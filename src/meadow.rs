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
    pub fn rand_pos(&self) -> Vec2 {
        vec2(gen_range(0., self.size.x), gen_range(0., self.size.y))
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
    let meadow = {
        let settings = resources.get::<Settings>().expect("Missing settings");
        let meadow = Meadow::new(settings.meadow_height * 100., settings.meadow_width * 100.);
        for _ in 0..settings.num_flowers * 10 {
            let pos = meadow.rand_pos();
            let color = rand_flower_color();
            let radius = gen_range(settings.flower_size.start, settings.flower_size.end);
            world.push((
                Flower {
                    color,
                    radius,
                    collected: false,
                },
                Position::from(pos),
            ));
        }
        meadow
    };
    resources.insert(meadow);
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
fn draw_ground(#[resource] _: &Meadow, #[resource] settings: &Settings) {
    clear_background(settings.meadow_color);
}

#[system(for_each)]
fn draw_flower(flower: &Flower, pos: &Position, #[resource] settings: &Settings) {
    let Position(pos) = pos;
    draw_circle(pos.x, pos.y, flower.radius, flower.color);
    draw_circle(
        pos.x,
        pos.y,
        flower.radius * settings.flower_core_size / 1000.,
        if flower.collected {
            settings.flower_collected_color
        } else {
            settings.flower_uncollected_color
        },
    );
}

/// Make reasonable flower colors, basically green<=blue&red, and at least one maxed channel.
fn rand_flower_color() -> Color {
    let r = gen_range(0., 1.);
    let b = gen_range(0., 1.);
    let g = gen_range(0f32, 1.).min(r).min(b);
    // scale them all so that the max channel is 1.
    let [r, g, b]: [f32; 3] = (vec3(r, g, b) / r.max(b)).into();
    Color::new(r, g, b, 1.)
}
