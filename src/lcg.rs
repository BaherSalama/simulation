use crate::utils::*;
use eframe::egui::{self, Ui};

pub struct Lcg {
    start: f32,
    amount: usize,
    div: f32,
    add: f32,
    mul: f32,
    table: Vec<Vec<f32>>,
    headings: Vec<String>,
}

impl Default for Lcg {
    fn default() -> Self {
        Self {
            headings: vec!["i".into(), "Zi".into(), "Ui".into(), "Zn".into()],
            table: vec![],
            start: 5.,
            mul: 5.,
            amount: 32,
            div: 32.,
            add: 3.,
        }
    }
}

impl super::View for Lcg {
    fn ui(&mut self, ui: &mut Ui, id: u64) {
        egui::ScrollArea::vertical().id_salt(id).show(ui, |ui| {
            ui.heading("Lcg");
            ui.add(egui::Slider::new(&mut self.amount, 0..=100).text("count"));
            ui.add(egui::Slider::new(&mut self.start, 1.0..=100.0).text("X"));
            ui.add(egui::Slider::new(&mut self.div, 0.0..=100.).text("div"));
            ui.add(egui::Slider::new(&mut self.add, 0.0..=100.).text("add"));
            ui.add(egui::Slider::new(&mut self.mul, 0.0..=100.).text("mul"));
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

impl Lcg {
    fn solve(&mut self) {
        size_mat(&mut self.table, self.amount, 3);
        for row in 0..self.table.len() {
            if row == 0 {
                self.table[row][0] = self.start;
                self.table[row][1] = 0.;
                self.table[row][2] = (self.table[row][0] * self.mul + self.add) % self.div;
            } else {
                self.table[row][0] = self.table[row - 1][2];
                self.table[row][2] = (self.table[row][0] * self.mul + self.add) % self.div;
                self.table[row][1] = self.table[row][0] / self.div;
            }
        }
    }
}
