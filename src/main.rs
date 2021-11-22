#[cfg(feature = "console")]
use crate::settings::Settings;
#[cfg(feature = "console")]
use egui_macroquad::egui;
use macroquad::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod backstage;
mod bee;
mod camera;
mod meadow;
mod prelude;
mod settings;
mod spritesheet;

#[macroquad::main("BumbleUmbleGee")]
async fn main() {
    let mut stage_manager = backstage::StageManager::new();
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        // Process keys, mouse etc.
        #[cfg(feature = "console")]
        egui_macroquad::ui(|egui_ctx| {
            let mut settings = stage_manager
                .resources
                .get_mut::<Settings>()
                .expect("missing settings");
            egui::Window::new("Settings").show(egui_ctx, |ui| {
                settings.egui(ui);
            });
        });
        stage_manager.execute();
        #[cfg(feature = "console")]
        egui_macroquad::draw();
        next_frame().await
    }
}
