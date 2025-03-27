use crate::utils::*;
use eframe::egui::{self, Ui, Window};

pub struct Server {
    server_count: usize,
    customer_count: usize,
    arrival_rows: usize,
    service_rows: usize,
    arriavl_table: Vec<Vec<String>>,
    service_table: Vec<Vec<Vec<String>>>,
    random_table: Vec<Vec<String>>,
    interval_arrival: Vec<Vec<f32>>,
    interval_service: Vec<Vec<Vec<f32>>>,
    arrival_headings: Vec<String>,
    repeated_headings: Vec<String>,
    end_headings: Vec<String>,
    sol_headings: Vec<String>,
    arrival: Vec<std::ops::Range<u8>>,
    service: Vec<Vec<std::ops::Range<u8>>>,
    sol: Vec<Vec<f32>>,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            arrival: vec![1..5, 1..5],
            service: vec![
                vec![1..5, 1..5],
                vec![1..5, 1..5],
                vec![1..5, 1..5],
                vec![1..5, 1..5],
                vec![1..5, 1..5],
            ],
            sol: vec![],
            arrival_headings: vec![
                "arrival".into(),
                "prop".into(),
                "cumulative".into(),
                "interval prob".into(),
            ],
            repeated_headings: vec!["service".into(), "start".into(), "end".into()],
            sol_headings: vec![
                "Customer ".into(),
                "Interarrival time".into(),
                "Arrival clock".into(),
            ],
            end_headings: vec![
                "Waiting time in queue".into(),
                "Time of customer in system".into(),
            ],
            interval_arrival: vec![],
            interval_service: vec![vec![], vec![], vec![], vec![], vec![]],
            arrival_rows: 4,
            service_rows: 4,
            arriavl_table: vec![
                vec!["1".into(), "0.25".into()],
                vec!["2".into(), "0.40".into()],
                vec!["3".into(), "0.20".into()],
                vec!["4".into(), "0.15".into()],
            ],
            service_table: vec![
                vec![
                    vec!["2".into(), "0.3".into()],
                    vec!["3".into(), "0.28".into()],
                    vec!["4".into(), "0.25".into()],
                    vec!["5".into(), "0.17".into()],
                ],
                vec![
                    vec!["3".into(), "0.35".into()],
                    vec!["4".into(), "0.25".into()],
                    vec!["5".into(), "0.2".into()],
                    vec!["6".into(), "0.2".into()],
                ],
            ],
            random_table: vec![
                vec!["0".into(), "95".into()],
                vec!["26".into(), "21".into()],
                vec!["98".into(), "51".into()],
                vec!["90".into(), "92".into()],
                vec!["26".into(), "89".into()],
                vec!["42".into(), "38".into()],
                vec!["74".into(), "13".into()],
                vec!["80".into(), "61".into()],
                vec!["68".into(), "50".into()],
                vec!["22".into(), "49".into()],
            ],
            server_count: 2,
            customer_count: 10,
        }
    }
}

impl Server {
    pub fn in_arrival(&mut self, ui: &mut Ui) {
        let a = vec![(0, 0), (1, 1)];
        ui.add(egui::Slider::new(&mut self.arrival_rows, 0..=10).text("random"));
        if ui.button("arrival").clicked() {
            size_mat(&mut self.interval_arrival, self.arrival_rows, 4);
            parse_in(&self.arriavl_table, &mut self.interval_arrival, &a);
            make_intervals(&mut self.interval_arrival);
            make_range(&self.interval_arrival, &mut self.arrival);
        }
        input_table(
            ui,
            self.arrival_rows,
            &mut self.arriavl_table,
            &self.arrival_headings,
            &"arrival".into(),
        );
    }
    pub fn out_arrival(&mut self, ui: &mut Ui) {
        let a = vec![(0, 0), (1, 1)];
        display_table(
            ui,
            &self.interval_arrival,
            &"arrival interval".into(),
            false,
            &self.arrival_headings,
            Some(&self.arrival),
        )
    }

    pub fn in_service(&mut self, ui: &mut Ui, i: usize) {
        let a = vec![(0, 0), (1, 1)];
        ui.vertical(|ui| {
            ui.vertical(|ui| {
                let mut a = "service".to_string();
                a.push_str(&i.to_string());
                input_table(
                    ui,
                    self.service_rows,
                    &mut self.service_table[i],
                    &self.arrival_headings,
                    &a,
                );
            });
        });
    }

    pub fn out_service(&mut self, ui: &mut Ui, i: usize) {
        ui.vertical(|ui| {
            ui.vertical(|ui| {
                let mut a = "service".to_string();
                a.push_str(&i.to_string());
                display_table(
                    ui,
                    &self.interval_service[i],
                    &a,
                    false,
                    &self.arrival_headings,
                    Some(&self.service[i]),
                );
            });
        });
    }
    pub fn out_res(&mut self, ui: &mut Ui) {
        let a = vec![(0, 0), (1, 1)];
        ui.add(egui::Slider::new(&mut self.service_rows, 0..=10).text("random"));
        ui.add(egui::Slider::new(&mut self.server_count, 1..=10).text("number of servers"));
        if ui.button("run").clicked() {
            for i in 0..self.server_count {
                size_mat(&mut self.interval_service[i], self.service_rows, 4);
                parse_in(&self.service_table[i], &mut self.interval_service[i], &a);
                make_intervals(&mut self.interval_service[i]);
                make_range(&self.interval_service[i], &mut self.service[i]);
            }
        }
        self.service_table.resize(self.server_count, vec![]);
        for row in 0..self.server_count {
            self.service_table[row].resize(self.service_rows, vec![]);
        }
    }
    pub fn in_res(&mut self, ui: &mut Ui) {
        ui.add(egui::Slider::new(&mut self.customer_count, 0..=20).text("customer"));
        input_table(
            ui,
            self.customer_count,
            &mut self.random_table,
            &self.sol_headings[1..3],
            &"sol".into(),
        );
        let b = vec![(0, 0), (1, 2)];
        if ui.button("run").clicked() {
            make_headnig(
                &mut self.sol_headings,
                &mut self.repeated_headings,
                &self.end_headings,
                self.server_count,
            );
            size_mat(
                &mut self.sol,
                self.customer_count,
                4 + (self.server_count * 3),
            );
            parse_in(&self.random_table, &mut self.sol, &b);
            for j in 0..self.server_count {
                for i in 0..self.customer_count {
                    self.sol[i][(j * 3) + 2] = self.sol[i][2];
                }
            }
            range_edit(&mut self.sol, &self.arrival, &self.interval_arrival, 0);
            for i in 0..self.server_count {
                range_edit(
                    &mut self.sol,
                    &self.service[i],
                    &self.interval_service[i],
                    (i * 3) + 2,
                );
            }
            // self.solve();
            comulative(&mut self.sol, 0, 1);
            run(&mut self.sol, self.server_count);
        }
    }
    pub fn wd(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            display_table(
                ui,
                &self.sol,
                &"Answer".into(),
                true,
                &self.sol_headings,
                None,
            );
        });
    }
}

impl Server {
    fn solve(&mut self) {
        comulative(&mut self.sol, 0, 1);
        run(&mut self.sol, self.server_count);
    }
}
