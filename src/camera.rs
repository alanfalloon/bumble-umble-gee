//! The camera.
//!
//! Follow the bee, show us what is coming. Help us notice stuff. But be gentle,
//! remember we are moving the bee with our touch.

use legion::{component, world::SubWorld, Entity, EntityStore, IntoQuery};

use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Camera {
    bee: Entity,
    camera2d: Camera2D,
}

pub fn roll_call(
    world: &mut legion::world::World,
    systems: &mut legion::systems::Builder,
    resources: &mut legion::systems::Resources,
) {
    let all_bees: Vec<Entity> = Entity::query()
        .filter(component::<crate::bee::Bee>())
        .iter_mut(world)
        .copied()
        .collect();
    assert_eq!(all_bees.len(), 1, "Expected exactly one bee entity");
    let camera = Camera {
        bee: all_bees[0],
        camera2d: Camera2D::default(),
    };
    resources.insert(camera);
    systems.add_system(follow_bee_system());
}

#[system]
#[read_component(Position)]
#[read_component(Velocity)]
fn follow_bee(world: &mut SubWorld, #[resource] camera: &mut Camera) {
    let bee = world.entry_ref(camera.bee).expect("Bee missing");
    let Position(pos) = *bee.get_component::<Position>().expect("Bee missing pos");
    let Velocity(vel) = *bee.get_component::<Velocity>().expect("Bee missing vel");
    let screen = vec2(screen_width(), screen_height());
    let aspect = screen.normalize() * vel.length().max(40.) * 5.;
    let target = pos + vel - aspect / 2.;
    camera.camera2d = Camera2D::from_display_rect(Rect {
        x: target.x,
        y: target.y,
        w: aspect.x,
        h: aspect.y,
    });
    set_camera(&camera.camera2d);
}

impl Camera {
    pub fn screen_to_world(&self, point: Vec2) -> Vec2 {
        self.camera2d.screen_to_world(point)
    }
}
