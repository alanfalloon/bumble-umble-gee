use macroquad::prelude::*;
#[derive(Debug)]
pub struct Settings {
    pub quit: bool,
    pub restart: bool,
    pub mass: f32,
    pub max_thrust: f32,
    pub wind_resistance: f32,
    pub meadow_color: Color,
    pub animation_speed: f32,
    pub bee_size: f32,
    pub max_zoom: f32,
    pub velocity_zoom: f32,
    pub flower_core_size: f32,
    pub flower_uncollected_color: Color,
    pub flower_collected_color: Color,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            quit: false,
            restart: false,
            mass: 1.,
            max_thrust: 100.,
            animation_speed: 10.,
            wind_resistance: 70.,
            meadow_color: Color::new(0.58, 0.78, 0.58, 1.00),
            bee_size: 200.,
            flower_core_size: 250.,
            flower_uncollected_color: YELLOW,
            flower_collected_color: DARKBROWN,
            max_zoom: 40.,
            velocity_zoom: 50.,
        }
    }
}
impl Settings {
    pub fn want_quit(&mut self) -> bool {
        let res = self.quit;
        self.quit = false;
        res
    }
    pub fn want_restart(&mut self) -> bool {
        let res = self.restart || self.quit;
        self.restart = false;
        res
    }
}

#[cfg(feature = "console")]
mod ui {
    use super::Settings;
    use egui_macroquad::egui;
    use macroquad::prelude::*;

    impl Settings {
        pub fn egui(&mut self, ui: &mut egui::Ui) {
            self.quit.declare_ui("Quit", ui);
            self.restart.declare_ui("Restart", ui);
            self.mass.declare_ui("Mass", ui);
            self.max_thrust.declare_ui("Max thrust", ui);
            self.wind_resistance.declare_ui("Wind resistance", ui);
            self.animation_speed.declare_ui("Animation speed", ui);
            self.bee_size.declare_ui("Bee size", ui);
            self.flower_core_size.declare_ui("Flower centre", ui);
            self.max_zoom.declare_ui("Max zoom", ui);
            self.velocity_zoom.declare_ui("Velocity zoom", ui);
            self.meadow_color.declare_ui("Meadow color", ui);
            self.flower_uncollected_color
                .declare_ui("Uncollected color", ui);
            self.flower_collected_color
                .declare_ui("Collected color", ui);
        }
    }

    trait Config {
        fn declare_ui(&mut self, label: &str, ui: &mut egui::Ui);
    }

    impl Config for f32 {
        fn declare_ui(&mut self, label: &str, ui: &mut egui::Ui) {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add(egui::DragValue::new(self).clamp_range(1f32..=1000f32));
            });
        }
    }

    impl Config for bool {
        fn declare_ui(&mut self, label: &str, ui: &mut egui::Ui) {
            if ui.add(egui::Button::new(label)).clicked() {
                *self = true;
            }
        }
    }

    impl Config for Color {
        fn declare_ui(&mut self, label: &str, ui: &mut egui::Ui) {
            ui.horizontal(|ui| {
                ui.label(label);
                let Color { r, g, b, .. } = *self;
                let mut hsva = egui::color::Hsva::from_rgb([r, g, b]);
                egui::color_picker::color_edit_button_hsva(
                    ui,
                    &mut hsva,
                    egui::color_picker::Alpha::Opaque,
                );
                let [r, g, b] = hsva.to_rgb();
                *self = Color::new(r, g, b, 1.);
            });
        }
    }
}
