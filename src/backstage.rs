//! The show must go on!
//!
//! Keeping track of everyone. Lets make sure everyone gets thier cues.

use crate::{camera::Camera, prelude::*};
use legion::{system, Resources, Schedule, World};

pub struct StageManager {
    world: World,
    schedule: Schedule,
    pub resources: Resources,
}

impl StageManager {
    pub fn new(settings: Settings) -> StageManager {
        let mut world = World::default();
        let mut builder = Schedule::builder();
        let mut resources = Resources::default();
        // First, settings
        resources.insert(settings);
        // Next timekeeping. Add the clock and the system to keep it current.
        resources.insert(GameClock {
            time: get_time(),
            tick: Duration::default(),
        });
        builder.add_system(tick_system());
        // Next inputs.
        resources.insert(Inputs { mouse_click: None });
        builder.add_system(inputs_system());
        // In Z-order so drawing happens correctly
        crate::meadow::roll_call(&mut world, &mut builder, &mut resources);
        crate::bee::roll_call(&mut world, &mut builder, &mut resources);
        crate::camera::roll_call(&mut world, &mut builder, &mut resources);
        let schedule = builder.build();
        StageManager {
            world,
            schedule,
            resources,
        }
    }

    pub fn execute(&mut self) {
        self.schedule.execute(&mut self.world, &mut self.resources)
    }

    pub fn settings(&self) -> atomic_refcell::AtomicRefMut<Settings> {
        self.resources
            .get_mut::<Settings>()
            .expect("missing settings")
    }
}

#[system]
fn tick(#[resource] clock: &mut GameClock) {
    let time = get_time();
    let tick = Duration::from_secs_f64(time - clock.time);
    clock.time = time;
    clock.tick = tick;
}

#[system]
fn inputs(#[resource] inputs: &mut Inputs, #[resource] camera: &Camera) {
    inputs.mouse_click = if is_mouse_button_pressed(MouseButton::Left) {
        Some(camera.screen_to_world(mouse_position().into()))
    } else {
        None
    };
}
