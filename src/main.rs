use macroquad::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod backstage;
mod bee;
mod camera;
mod meadow;
mod prelude;
mod spritesheet;

#[macroquad::main("BumbleUmbleGee")]
async fn main() {
    let mut stage_manager = backstage::StageManager::new();
    loop {
        stage_manager.execute();
        next_frame().await
    }
}
