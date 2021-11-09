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
    clock: GameTime,
}

impl StageManager {
    pub fn new() -> StageManager {
        let (world, schedule) = roll_call();
        StageManager {
            world,
            schedule,
            resources: Resources::default(),
            clock: GameTime::new(),
        }
    }
    pub fn tick(&mut self) {
        self.resources.insert(self.clock.tick());
        self.resources.insert(Inputs::grab());
        self.schedule.execute(&mut self.world, &mut self.resources)
    }
}
struct GameTime {
    time: f64,
    tick: Duration,
}
impl GameTime {
    fn new() -> Self {
        GameTime {
            time: get_time(),
            tick: Duration::default(),
        }
    }
    fn tick(&mut self) -> Duration {
        let time = get_time();
        let tick = Duration::from_secs_f64(time - self.time);
        self.time = time;
        self.tick = tick;
        tick
    }
}
