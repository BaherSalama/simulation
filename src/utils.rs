use core::f32;

use crate::egui;
use eframe::glow::VENDOR;
use egui_plot::Bar;
use egui_plot::BarChart;
use egui_plot::Legend;
use egui_plot::Plot;

pub fn display_table(
    ui: &mut egui::Ui,
    list: &Vec<Vec<f32>>,
    name: &String,
    order: bool,
    headings: &Vec<String>,
    end: Option<&Vec<std::ops::Range<u8>>>,
) {
    ui.label(name);
    egui::Grid::new(name).striped(true).show(ui, |ui| {
        for row in headings {
            ui.label(format!("{row}"));
        }
        ui.end_row();
        for row in 0..list.len() {
            for col in 0..list[0].len() {
                if col == 0 {
                    if order {
                        ui.label(format!("{row}"));
                    }
                    ui.label(format!("{:.3}", list[row][col]));
                } else {
                    if col == list.len() - 1 {
                        if row == list.len() - 1 {
                            match end {
                                Some(a) => ui.label(format!("{:?}", a[row].start..0)),
                                None => ui.label(format!("{:.3}", list[row][col])),
                            };
                        } else {
                            match end {
                                Some(a) => ui.label(format!("{:?}", a[row])),
                                None => ui.label(format!("{:.3}", list[row][col])),
                            };
                        }
                    } else {
                        ui.label(format!("{:.3}", list[row][col]));
                    }
                }
            }
            ui.end_row();
        }
    });
    ui.separator();
}
pub fn run(mat: &mut Vec<Vec<f32>>, size: usize) {
    let clock: usize = 1;
    let service: usize = 2;
    let begin: usize = 3;
    let end: usize = 4;
    let wq: usize = mat[0].len() - 2;
    let ws: usize = mat[0].len() - 1;
    let mut end_prev: Vec<f32> = vec![0.; size];
    for row in 0..mat.len() {
        if row == 0 {
            mat[row][begin] = 0.;
            mat[row][end] = mat[row][service];
            mat[row][wq] = 0.;
            mat[row][ws] = mat[row][service];
        } else {
            if row == 1 {
                end_prev[0] = mat[row - 1][end];
            }
            let b = find_min_order(&end_prev, mat[row][clock]);
            let a = b * 3 + end;
            let c = b * 3 + begin;
            let d = b * 3 + service;
            if end_prev[b] < mat[row][clock] {
                mat[row][c] = mat[row][clock];
                mat[row][a] = mat[row][clock] + mat[row][d];
                mat[row][wq] = 0.;
                mat[row][ws] = mat[row][d];
                end_prev[b] = mat[row][a];
            } else {
                let b = 0;
                let a = end;
                let c = begin;
                let d = service;
                mat[row][c] = end_prev[b];
                mat[row][a] = end_prev[b] + mat[row][d];
                mat[row][wq] = end_prev[b] - mat[row][clock];
                mat[row][ws] = mat[row][d] + mat[row][wq];
                // mat[row][a] = end_prev[b] + mat[row][d] + (end_prev[b] - mat[row][clock]);
                end_prev[b] = mat[row][a];
            }
        }
    }
}
pub fn find_min(sad: &Vec<f32>) -> usize {
    let mut min = f32::INFINITY;
    let mut index: usize = 0;
    for i in 0..sad.len() {
        if sad[i] <= min {
            min = sad[i];
            index = i;
            break;
        }
    }
    index
}
pub fn find_min_order(sad: &Vec<f32>, less: f32) -> usize {
    let mut index: usize = 0;
    for i in 0..sad.len() {
        if sad[i] <= less {
            index = i;
            break;
        }
    }
    index
}
pub fn comulative(mat: &mut Vec<Vec<f32>>, what: usize, pos: usize) {
    for row in 0..mat.len() {
        mat[row][pos] = 0.;
    }
    for row in 0..mat.len() {
        for col in 0..=row {
            mat[row][pos] += mat[col][what];
        }
    }
}
pub fn size_mat(input: &mut Vec<Vec<f32>>, x: usize, y: usize) {
    input.resize(x, vec![]);
    for row in 0..x {
        input[row].resize(y, 0.);
    }
}
pub fn make_range(a: &Vec<Vec<f32>>, b: &mut Vec<std::ops::Range<u8>>) {
    let mut begin: u8 = 1;
    let mut e: u8 = 0;
    b.resize(a.len(), 1..2);
    for row in 0..a.len() {
        e = (a[row][2] * 100.).round() as u8;
        if row != 0 {
            {
                begin = (a[row - 1][2] * 100. + 1.).round() as u8;
            }
        }
        b[row] = begin..e;
    }
}
pub fn range_edit(
    a: &mut Vec<Vec<f32>>,
    b: &Vec<std::ops::Range<u8>>,
    c: &Vec<Vec<f32>>,
    inside: usize,
) {
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    for row in 0..a.len() {
        one_num_check(a, b, c, inside, row);
    }
}
pub fn one_num_check(
    a: &mut Vec<Vec<f32>>,
    b: &Vec<std::ops::Range<u8>>,
    c: &Vec<Vec<f32>>,
    inside: usize,
    row: usize,
) {
    for col in 0..b.len() {
        if b[col].contains(&(a[row][inside] as u8)) {
            a[row][inside] = c[col][0];
            break;
        }
    }
}

pub fn parse_in(
    inputs: &Vec<Vec<String>>,
    outpot: &mut Vec<Vec<f32>>,
    this_in: &Vec<(usize, usize)>,
) {
    for row in 0..inputs.len() {
        for (a, b) in this_in {
            outpot[row][*b] = inputs[row][*a].parse::<f32>().unwrap();
        }
    }
}

pub fn make_intervals(mat: &mut Vec<Vec<f32>>) {
    comulative(mat, 1, 2);
}
pub fn input_table(
    ui: &mut egui::Ui,
    count: usize,
    list: &mut Vec<Vec<String>>,
    headings: &[String],
    id: &String,
) {
    egui::Grid::new(id).striped(true).show(ui, |ui| {
        for row in headings {
            ui.label(format!("{row}"));
        }
        ui.end_row();
        list.resize(count, vec![]);
        for row in 0..count {
            list[row].resize(2, "0".to_string());
        }
        for row in 0..list.len() {
            for col in 0..list[0].len() {
                ui.text_edit_singleline(&mut list[row][col]);
            }
            ui.end_row();
        }
    });
    ui.separator();
}
pub fn make_headnig(
    headings: &mut Vec<String>,
    repeated: &mut Vec<String>,
    end: &Vec<String>,
    count: usize,
) {
    headings.resize(4 + (count * 3) + 1, "".to_string());
    let a = headings.len();
    for i in 3..a - 2 {
        headings[i] = repeated[i % 3].to_string();
    }
    headings[a - 2] = end[0].to_string();
    headings[a - 1] = end[1].to_string();
}

pub fn draw_char(ui: &mut egui::Ui) {
    let mut chart1 = BarChart::new(vec![
        Bar::new(0.5, 1.0).name("Day 1"),
        Bar::new(1.5, 3.0).name("Day 2"),
        Bar::new(2.5, 1.0).name("Day 3"),
        Bar::new(3.5, 2.0).name("Day 4"),
        Bar::new(4.5, 4.0).name("Day 5"),
    ])
    .width(0.7)
    .name("Set 1");

    let mut chart2 = BarChart::new(vec![
        Bar::new(0.5, 1.0),
        Bar::new(1.5, 1.5),
        Bar::new(2.5, 0.1),
        Bar::new(3.5, 0.7),
        Bar::new(4.5, 0.8),
    ])
    .width(0.7)
    .name("Set 2")
    .stack_on(&[&chart1]);

    let mut chart3 = BarChart::new(vec![
        Bar::new(0.5, -0.5),
        Bar::new(1.5, 1.0),
        Bar::new(2.5, 0.5),
        Bar::new(3.5, -1.0),
        Bar::new(4.5, 0.3),
    ])
    .width(0.7)
    .name("Set 3")
    .stack_on(&[&chart1, &chart2]);

    let mut chart4 = BarChart::new(vec![
        Bar::new(0.5, 0.5),
        Bar::new(1.5, 1.0),
        Bar::new(2.5, 0.5),
        Bar::new(3.5, -0.5),
        Bar::new(4.5, -0.5),
    ])
    .width(0.7)
    .name("Set 4")
    .stack_on(&[&chart1, &chart2, &chart3]);

    chart1 = chart1.horizontal();
    chart2 = chart2.horizontal();
    chart3 = chart3.horizontal();
    chart4 = chart4.horizontal();

    Plot::new("Stacked Bar Chart Demo")
        .legend(Legend::default())
        .data_aspect(1.0)
        // .allow_drag(self.allow_drag)
        .show(ui, |plot_ui| {
            plot_ui.bar_chart(chart1);
            plot_ui.bar_chart(chart2);
            plot_ui.bar_chart(chart3);
            plot_ui.bar_chart(chart4);
        });
}
