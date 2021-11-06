// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use macroquad::prelude::*;
#[macroquad::main("BumbleUmbleGee")]
async fn main() {
    // #[cfg(target_arch = "wasm32")]
    // std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    loop {
        clear_background(LIGHTGRAY);
        let middle = (screen_width() / 2., screen_height() / 2.);
        draw_circle(middle.0, middle.1, 60., BLACK);
        next_frame().await
    }
}
