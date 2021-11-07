//! The show must go on!
//!
//! Keeping track of everyone. Lets make sure everyone gets thier cues.

use crate::prelude::*;
use legion::{Resources, Schedule, World};

/// Gather everyone before the show.
fn roll_call() -> (World, Schedule) {
    let mut world = World::default();
    let mut builder = Schedule::builder();
    // In Z-order so drawing happens correctly
    crate::meadow::roll_call(&mut world, &mut builder);
    crate::bee::roll_call(&mut world, &mut builder);
    crate::camera::roll_call(&mut world, &mut builder);
    (world, builder.build())
}

pub struct StageManager {
    world: World,
    schedule: Schedule,
    resources: Resources,
}

impl StageManager {
    pub fn new() -> StageManager {
        let (world, schedule) = roll_call();
        StageManager {
            world,
            schedule,
            resources: Resources::default(),
        }
    }
    pub fn tick(&mut self) {
        self.schedule.execute(&mut self.world, &mut self.resources)
    }
}
