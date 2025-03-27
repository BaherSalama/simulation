mod lcg;
mod midsquare;
mod server;
mod utils;

use std::{
    collections::hash_map::RandomState,
    fmt,
    hash::{BuildHasher, Hasher},
    mem::ManuallyDrop,
    sync::{Arc, LazyLock, Mutex},
};

use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use lcg::Lcg;
use midsquare::Midsquare;
use server::Server;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, id: u64);
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
            Ok(Box::<App>::default())
        }),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::<App>::default())),
            )
            .await;
    });
}

// Shared server state
static SERVER: LazyLock<Mutex<Server>> = LazyLock::new(|| Mutex::new(Server::default()));

// Tab types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ServerTab {
    InService,
    OutService,
    InArrival,
    OutArrival,
    InRes,
    OutRes,
    Wd,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TabType {
    Midsquare,
    Lcg,
    Server(ServerTab),
}

impl fmt::Display for TabType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TabType::Midsquare => write!(f, "MidSquare"),
            TabType::Lcg => write!(f, "LCG"),
            TabType::Server(ServerTab::InService) => write!(f, "Input service"),
            TabType::Server(ServerTab::OutService) => write!(f, "Output service"),
            TabType::Server(ServerTab::InArrival) => write!(f, "Input arrival"),
            TabType::Server(ServerTab::OutArrival) => write!(f, "Output arrival"),
            TabType::Server(ServerTab::InRes) => write!(f, "input res"),
            TabType::Server(ServerTab::OutRes) => write!(f, "output res"),
            TabType::Server(ServerTab::Wd) => write!(f, "wd"),
        }
    }
}

// Tab content with proper memory management
enum TabContent {
    Midsquare(ManuallyDrop<Midsquare>),
    Lcg(ManuallyDrop<Lcg>),
    Server(ServerTab),
}

impl TabContent {
    fn new(tab_type: TabType) -> Self {
        match tab_type {
            TabType::Midsquare => Self::Midsquare(ManuallyDrop::new(Midsquare::default())),
            TabType::Lcg => Self::Lcg(ManuallyDrop::new(Lcg::default())),
            TabType::Server(tab) => Self::Server(tab),
        }
    }
}

impl Drop for TabContent {
    fn drop(&mut self) {
        match self {
            Self::Midsquare(mid) => unsafe { ManuallyDrop::drop(mid) },
            Self::Lcg(lcg) => unsafe { ManuallyDrop::drop(lcg) },
            Self::Server(_) => {}
        }
    }
}

// Main tab structure
struct Tab {
    id: u64,
    tab_type: TabType,
    content: TabContent,
}

impl Tab {
    fn new(tab_type: TabType) -> Self {
        Self {
            id: fast_random_hash(),
            tab_type: tab_type.clone(),
            content: TabContent::new(tab_type),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        match &mut self.content {
            TabContent::Midsquare(mid) => unsafe { &mut **mid }.ui(ui, self.id),
            TabContent::Lcg(lcg) => unsafe { &mut **lcg }.ui(ui, self.id),
            TabContent::Server(tab) => {
                let mut server = SERVER.lock().unwrap();
                match tab {
                    ServerTab::InService => server.in_service(ui, 0),
                    ServerTab::OutService => server.out_service(ui, 0),
                    ServerTab::InArrival => server.in_arrival(ui),
                    ServerTab::OutArrival => server.out_arrival(ui),
                    ServerTab::InRes => server.in_res(ui),
                    ServerTab::OutRes => server.out_res(ui),
                    ServerTab::Wd => server.wd(ui),
                }
            }
        }
    }
}

// Main application
pub struct App {
    tree: DockState<Tab>,
}

impl Default for App {
    fn default() -> Self {
        let mut tree = DockState::new(vec![
            Tab::new(TabType::Lcg),
            Tab::new(TabType::Midsquare),
            Tab::new(TabType::Server(ServerTab::Wd)),
        ]);
        Self { tree }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_menu_bar(ctx);

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut TabViewer);
    }
}

impl App {
    fn show_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);

                ui.menu_button("Tools", |ui| {
                    if ui.button("Add LCG").clicked() {
                        self.add_tab(TabType::Lcg);
                    }
                    if ui.button("Add MidSquare").clicked() {
                        self.add_tab(TabType::Midsquare);
                    }

                    ui.menu_button("Server", |ui| {
                        if ui.button("Input service").clicked() {
                            self.add_tab(TabType::Server(ServerTab::InService));
                        }
                        if ui.button("Output Service").clicked() {
                            self.add_tab(TabType::Server(ServerTab::OutService));
                        }
                        if ui.button("Input arrival").clicked() {
                            self.add_tab(TabType::Server(ServerTab::InArrival));
                        }
                        if ui.button("Output arrival").clicked() {
                            self.add_tab(TabType::Server(ServerTab::OutArrival));
                        }
                        if ui.button("Input res").clicked() {
                            self.add_tab(TabType::Server(ServerTab::InRes));
                        }
                        if ui.button("Output res").clicked() {
                            self.add_tab(TabType::Server(ServerTab::OutRes));
                        }
                        if ui.button("wd").clicked() {
                            self.add_tab(TabType::Server(ServerTab::Wd));
                        }
                    });
                });
            });
        });
    }

    fn add_tab(&mut self, tab_type: TabType) {
        self.tree
            .main_surface_mut()
            .push_to_focused_leaf(Tab::new(tab_type));
    }
}

struct TabViewer;

impl egui_dock::TabViewer for TabViewer {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.tab_type.to_string().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.push_id(tab.id, |ui| {
            tab.ui(ui);
        });
    }
    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(tab.id)
    }
}

// Helper function
fn fast_random_hash() -> u64 {
    let hasher = RandomState::new().build_hasher();
    hasher.finish()
}
