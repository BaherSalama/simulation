mod lcg;
mod midsquare;
pub mod server;
mod utils;

use eframe::egui::{self};
use lcg::Lcg;
use midsquare::Midsquare;
use server::Server;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<Mainapp>::default()
        }),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::<Mainapp>::default()),
            )
            .await
            .expect("failed to start eframe");
    });
}

#[derive(PartialEq, Eq)]
enum Pages {
    Server,
    Midsquare,
    Lcg,
}

pub struct Mainapp {
    page: Pages,
    server: Server,
    mid: Midsquare,
    lcg: Lcg,
}

impl Default for Mainapp {
    fn default() -> Self {
        Self {
            page: Pages::Server,
            server: Server::default(),
            mid: Midsquare::default(),
            lcg: Lcg::default(),
        }
    }
}

impl eframe::App for Mainapp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.page, Pages::Server, "Server");
                ui.selectable_value(&mut self.page, Pages::Midsquare, "MidSquare");
                ui.selectable_value(&mut self.page, Pages::Lcg, "Lcg");
            });
            match self.page {
                Pages::Server => self.server.ui(ui),
                Pages::Midsquare => self.mid.ui(ui),
                Pages::Lcg => self.lcg.ui(ui),
            }
        });
    }
}
