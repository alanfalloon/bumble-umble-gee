//! The meadow. The stage where it all takes place.
//!
//! Give us somewhere to frolic!

use legion::{world::SubWorld, Entity, EntityStore as _};
use macroquad::rand::gen_range;
use static_aabb2d_index::{StaticAABB2DIndex, StaticAABB2DIndexBuilder};

use crate::prelude::*;

/// The meadow
#[derive(Debug)]

pub struct Meadow {
    pub size: Vec2,
    pub flower_index: StaticAABB2DIndex<f32>,
    pub flower_entities: Vec<Entity>,
}
impl Meadow {
    pub fn new(world: &mut legion::world::World, settings: &Settings) -> Self {
        let meadow_size = vec2(settings.meadow_height * 100., settings.meadow_width * 100.);
        let num_flowers = settings.num_flowers * 10;
        let mut flower_index_builder = StaticAABB2DIndexBuilder::new(num_flowers);
        let mut flower_entities = Vec::with_capacity(num_flowers);
        for _ in 0..num_flowers {
            let pos = rand_pos(&meadow_size);
            let color = rand_flower_color();
            let radius = gen_range(settings.flower_size.start, settings.flower_size.end);
            flower_entities.push(world.push((
                Flower {
                    color,
                    radius,
                    collected: false,
                },
                Position::from(pos),
            )));
            flower_index_builder.add(
                pos.x - radius,
                pos.y - radius,
                pos.x + radius,
                pos.y + radius,
            );
        }
        Meadow {
            size: meadow_size,
            flower_entities,
            flower_index: flower_index_builder.build().unwrap(),
        }
    }

    pub fn clamp(&self, point: Vec2) -> Vec2 {
        point.clamp(Vec2::ZERO, self.size)
    }

    pub fn rand_pos(&self) -> Vec2 {
        rand_pos(&self.size)
    }

    pub fn flower_index_within(&self, rect: Rect) -> impl Iterator<Item = usize> + '_ {
        self.flower_index
            .query_iter(rect.left(), rect.top(), rect.right(), rect.bottom())
    }
}
fn rand_pos(size: &Vec2) -> Vec2 {
    vec2(gen_range(0., size.x), gen_range(0., size.y))
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
        Meadow::new(world, &settings)
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

#[system]
#[read_component(Position)]
#[read_component(Flower)]
fn draw_flower(
    world: &mut SubWorld,
    #[resource] camera: &mut crate::camera::Camera,
    #[resource] meadow: &Meadow,
    #[resource] settings: &Settings,
) {
    for flower_entry in meadow.flower_index_within(camera.rect).map(|index| {
        world
            .entry_ref(meadow.flower_entities[index])
            .expect("flower dissapeared")
    }) {
        let Position(pos) = *flower_entry
            .get_component::<Position>()
            .expect("Flower missing pos");
        let flower = flower_entry
            .get_component::<Flower>()
            .expect("Flower missing flower data");
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
}

/// Make reasonable flower colors, basically green<=blue&red, and at least one maxed channel.
fn rand_flower_color() -> Color {
    let r = gen_range(0f32, 1.);
    let b = gen_range(0., 1.);
    let g = gen_range(0., r.min(b));
    // scale them all so that the max channel is 1.
    let [r, g, b]: [f32; 3] = (vec3(r, g, b) / r.max(b)).into();
    Color::new(r, g, b, 1.)
}
