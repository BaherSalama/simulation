use crate::utils::*;
use eframe::egui::{self, Ui};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

pub struct Midsquare {
    start: String,
    amount: usize,
    table: Vec<Vec<f32>>,
    headings: Vec<String>,
}

impl Default for Midsquare {
    fn default() -> Self {
        Self {
            headings: vec!["i".into(), "zi".into(), "Ui".into(), "Zi2".into()],
            table: vec![],
            start: "8150".into(),
            amount: 10,
        }
    }
}

impl super::View for Midsquare {
    fn ui(&mut self, ui: &mut Ui, id: u64) {
        egui::ScrollArea::vertical().id_salt(id).show(ui, |ui| {
            ui.heading("Mid square");
            ui.add(egui::Slider::new(&mut self.amount, 0..=50).text("random"));
            ui.text_edit_singleline(&mut self.start);
            if ui.button("run").clicked() {
                self.solve();
            }
            display_table(
                ui,
                &self.table,
                &"arrival interval".into(),
                true,
                &self.headings,
                None,
            );
        });
    }
}

impl Midsquare {
    fn solve(&mut self) {
        size_mat(&mut self.table, self.amount, 3);
        for row in 0..self.table.len() {
            if row == 0 {
                self.table[row][0] = self.start.parse::<f32>().unwrap();
                self.table[row][1] = 0.;
                self.table[row][2] = self.table[row][0] * self.table[row][0];
            } else {
                self.table[row][0] = ((self.table[row - 1][2] / 100.) as u32 % 10000) as f32;
                self.table[row][2] = self.table[row][0] * self.table[row][0];
                self.table[row][1] = self.table[row][0] / 100.;
            }
        }
    }
}
