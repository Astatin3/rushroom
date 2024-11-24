use egui::{Ui, Color32, Stroke};
use eframe::egui_glow::glow;
use std::sync::Arc;
// use erased_serde::serialize_trait_object;

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub enum PaneMode {
    Hidden,
    Windowed,
    Right,
    Left,
    Bottom,
    Center,
}

#[typetag::serde(tag = "type")]
pub trait Pane {
    fn new() -> PaneState where Self: Sized;
    fn init(&mut self, pcc: &PsudoCreationContext);
    fn name(&mut self) -> &str;
    fn render(&mut self, ui: &mut Ui);
    fn context_menu(&mut self, ui: &mut Ui);
}

// impl Deserializer for Pane {

// }

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PaneState {
    // #[serde(skip)]
    pub pane: Box<dyn Pane>,
    pub id: String,
    pub mode: PaneMode,
    // pub window_location: Pos2,
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct NoPane {}
#[typetag::serde]
impl Pane for NoPane {
    fn new() -> PaneState where Self: Sized {
        let mut s = Self {};
        PaneState {
            id: s.name().to_string(),
            mode: PaneMode::Left,
            pane: Box::new(s),
        }
    }
    fn init(&mut self, _pcc: &PsudoCreationContext) {}
    fn name(&mut self) -> &str {"ERROR"}
    fn render(&mut self, _ui: &mut Ui){}
    fn context_menu(&mut self, _ui: &mut Ui) {}
}

impl PaneState {
    pub fn render(&mut self, ui: &mut Ui) {
        self.pane.render(ui);
    }
}


// impl Default for dyn Pane {
//     fn default() -> (impl Pane + 'static)  {
//         Box::new(NoPane {})
//     }
// }

// pub struct PaneGroup {
//     pub direction: TileDirection,
//     pub panes: Vec<PaneState>,
// }

// #[derive(serde::Deserialize, serde::Serialize)]
// #[derive(serde::Deserialize, serde::Serialize)]

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BluePane {}
#[typetag::serde]
impl Pane for BluePane {
    fn new() -> PaneState where Self: Sized {
        let mut s = Self {};
        PaneState {
            id: s.name().to_string(),
            mode: PaneMode::Left,
            pane: Box::new(s),
        }
    }
    fn init(&mut self, _pcc: &PsudoCreationContext){}
    fn name(&mut self) -> &str {"BLUE"}
    fn render(&mut self, ui: &mut Ui){
        ui.painter().rect(ui.max_rect(), 0., Color32::BLUE, Stroke::NONE);
    }
    fn context_menu(&mut self, _ui: &mut Ui) {}
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct GreenPane {}
#[typetag::serde]
impl Pane for GreenPane {
    fn new() -> PaneState where Self: Sized {
        let mut s = Self {};
        PaneState {
            id: s.name().to_string(),
            mode: PaneMode::Bottom,
            pane: Box::new(s),
        }
    }
    fn init(&mut self, _cc: &PsudoCreationContext){}
    fn name(&mut self) -> &str {"Green"}
    fn render(&mut self, ui: &mut Ui){
        ui.painter().rect(ui.max_rect(), 0., Color32::GREEN, Stroke::NONE);
    }
    fn context_menu(&mut self, _ui: &mut Ui) {}
}

pub struct PsudoCreationContext {
    pub gl: Option<Arc<glow::Context>>,
}

pub struct PaneManager {
    pcc: PsudoCreationContext,
    pub panes: Vec<PaneState>,
}


impl PaneManager {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // if let Some(cc) = cc {

        let mut panes = vec![
            BluePane::new(),
            GreenPane::new(),
            crate::point_cloud_renderer::PointRendererPane::new(),
        ];
 
        let pcc = PsudoCreationContext {
            gl: cc.gl.clone(),
        };

        for pane in &mut panes {
            pane.pane.init(&pcc);
        }

        Self {
            pcc,
            panes
        }
    }

    pub fn render(&mut self, ui: &mut Ui){
        let len = self.panes.len();

        egui::TopBottomPanel::top("top_panel").show(ui.ctx(), |ui| {

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!

                egui::widgets::global_theme_preference_switch(ui);
                
                ui.menu_button("File", |ui| {
                    if ui.button("Save Layout").clicked() {self.save_layout();}
                    if ui.button("Load Layout").clicked() {self.load_layout();}
                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("View", |ui| {

                    for i in 0..len {
                        ui.menu_button(self.panes[i].id.clone(), |ui| {
                            if ui.button("Hidden").clicked() {
                                self.panes[i].mode = PaneMode::Hidden;
                            }
                            if ui.button("Windowed").clicked() {
                                self.panes[i].mode = PaneMode::Windowed;
                            }
                            if ui.button("Left").clicked() {
                                self.panes[i].mode = PaneMode::Left;
                            }
                            if ui.button("Right").clicked() {
                                self.panes[i].mode = PaneMode::Right;
                            }
                            if ui.button("Bottom").clicked() {
                                self.panes[i].mode = PaneMode::Bottom;
                            }
                            if ui.button("Center").clicked() {
                                for a in 0..len {
                                    let pane2: &mut PaneState = &mut self.panes[a];
                                    if pane2.mode == PaneMode::Center {
                                        pane2.mode = PaneMode::Windowed;
                                    }
                                }
                                self.panes[i].mode = PaneMode::Center;
                            }
                        });
                    }
                });

                ui.separator();

                for i in 0..len {
                    ui.menu_button(self.panes[i].id.clone(), |ui| {
                        let pane: &mut PaneState = &mut self.panes[i];
                        if pane.mode != PaneMode::Hidden {
                            pane.pane.context_menu(ui);
                        }
                    });
                }
            });
        });

        for i in 0..len {
            let pane: &mut PaneState = &mut self.panes[i];

            match pane.mode {
                PaneMode::Hidden => {},
                PaneMode::Left => {
                    egui::panel::SidePanel::left(pane.id.clone())
                    .resizable(true)
                    .show(ui.ctx(), |ui| {
                        pane.render(ui);
                    });
                },
                PaneMode::Right => {
                    egui::panel::SidePanel::right(pane.id.clone())
                    .resizable(true)
                    .show(ui.ctx(), |ui| {
                        pane.render(ui);
                    });
                },
                PaneMode::Bottom => {
                    egui::panel::TopBottomPanel::bottom(pane.id.clone())
                    .resizable(true)
                    .show(ui.ctx(), |ui| {
                        pane.render(ui);
                    });
                },
                PaneMode::Windowed => {
                    egui::Window::new(pane.id.clone())
                    .resizable(true)
                    .max_width(ui.clip_rect().width()).max_height(ui.clip_rect().height())
                    .show(ui.ctx(), |ui| {
                        pane.render(ui);
                    });
                },
                PaneMode::Center => {
                    egui::CentralPanel::default()
                    .show(ui.ctx(), |ui| {
                        pane.render(ui);
                    });
                }
            }
        }
    }

    fn save_layout(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.panes) {
            let _ = std::fs::write("pane_layout.json", json);
        }
    }

    fn load_layout(&mut self) {
        if let Ok(panes) = std::fs::read_to_string("pane_layout.json") {
            if let Ok(panes) = serde_json::from_str(&panes) {

                self.panes = panes;

                for (mut pane) in &mut self.panes {
                    pane.pane.init(&self.pcc);
                }
            }
        }
    }
}
