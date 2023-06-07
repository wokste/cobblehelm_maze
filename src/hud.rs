use bevy::prelude::{Res, Resource};
use bevy_egui::{egui,EguiContexts};


// This resource tracks the game's score
#[derive(Resource)]
pub struct HUDInfo {
//    hp: i32,
//    hp_max: i32,
    score: i32,
    score_str: String,
}

impl HUDInfo {
    pub fn score_points(&mut self, score: i32) {
        self.score += score;
        self.score_str = format!("Score: {}", self.score);
    }
}

impl Default for HUDInfo {
    fn default() -> Self {
        let mut s = Self {
//            hp: 10,
//            hp_max: 10,
            score: 0,
            score_str: String::new(),
        };
        s.score_points(0);
        s
    }
}

pub fn render_hud(mut contexts: EguiContexts, hud: Res<HUDInfo>) {
    egui::TopBottomPanel::bottom("hud").show(contexts.ctx_mut(), |ui| {
        ui.label("HP: ??/??");
        ui.label(&hud.score_str);
    });
}