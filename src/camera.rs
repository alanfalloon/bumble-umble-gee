//! The camera.
//!
//! Follow the bee, show us what is coming. Help us notice stuff. But be gentle,
//! remember we are moving the bee with our touch.

use legion::{world::SubWorld, EntityStore};

use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Camera {
    pub rect: Rect,
    camera2d: Camera2D,
}

pub fn roll_call(
    _world: &mut legion::world::World,
    systems: &mut legion::systems::Builder,
    resources: &mut legion::systems::Resources,
) {
    let camera = Camera {
        rect: Rect::default(),
        camera2d: Camera2D::default(),
    };
    resources.insert(camera);
    systems.add_system(follow_bee_system());
}

#[system]
#[read_component(Position)]
#[read_component(Velocity)]
fn follow_bee(
    world: &mut SubWorld,
    #[resource] camera: &mut Camera,
    #[resource] the_bee: &TheBee,
    #[resource] settings: &Settings,
) {
    let bee = world.entry_ref(the_bee.entity).expect("Bee missing");
    let Position(pos) = *bee.get_component::<Position>().expect("Bee missing pos");
    let Velocity(vel) = *bee.get_component::<Velocity>().expect("Bee missing vel");
    let screen = vec2(screen_width(), screen_height());
    let aspect =
        screen.normalize() * (vel.length() * settings.velocity_zoom / 10.).max(settings.max_zoom);
    let target = pos + vel - aspect / 2.;
    camera.rect = Rect {
        x: target.x,
        y: target.y,
        w: aspect.x,
        h: aspect.y,
    };
    camera.camera2d = Camera2D::from_display_rect(camera.rect);
    set_camera(&camera.camera2d);
}

impl Camera {
    pub fn screen_to_world(&self, point: Vec2) -> Vec2 {
        self.camera2d.screen_to_world(point)
    }
}
