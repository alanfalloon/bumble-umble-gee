use macroquad::prelude::*;
pub struct Settings {
    pub mass: f32,
    pub max_thrust: f32,
    pub meadow_color: Color,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            mass: 1.,
            max_thrust: 100.,
            meadow_color: Color::new(0.58, 0.78, 0.58, 1.00),
        }
    }
}

#[cfg(feature = "console")]
mod ui {
    use super::Settings;
    use egui_macroquad::egui;
    use macroquad::prelude::*;

    impl Settings {
        pub fn egui(&mut self, ui: &mut egui::Ui) {
            ui.horizontal(|ui| {
                ui.label("Mass");
                ui.add(egui::DragValue::new(&mut self.mass).clamp_range(1f32..=1000f32));
            });
            ui.horizontal(|ui| {
                ui.label("Max thrust");
                ui.add(egui::DragValue::new(&mut self.max_thrust).clamp_range(1f32..=1000f32));
            });
            ui.horizontal(|ui| {
                ui.label("Meadow color");
                let Color { r, g, b, .. } = self.meadow_color;
                let mut hsva = egui::color::Hsva::from_rgb([r, g, b]);
                egui::color_picker::color_edit_button_hsva(
                    ui,
                    &mut hsva,
                    egui::color_picker::Alpha::Opaque,
                );
                let [r, g, b] = hsva.to_rgb();
                self.meadow_color = Color::new(r, g, b, 1.);
            });
        }
    }
}
