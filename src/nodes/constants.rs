use eframe::epaint::Color32;
use egui::Ui;
use egui_snarl::{InPin, OutPin};
use egui_snarl::ui::{PinInfo, WireStyle};
use crate::pane_manager::{Pane, PaneMode, PaneState, PsudoCreationContext};
use crate::panes::pipeline_editor::Node;


#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Constants {
    vars: Vec<String>,
    popup_open: bool,
}
#[typetag::serde]
impl Node for Constants {
    fn new() -> Self {
        Self {
            vars: Vec::new(),
            popup_open: false,
        }
    }

    fn get_name(&self) -> &str {
        "Constants"
    }
    fn get_description(&self) -> &str {"Test Node"}

    fn duplicate(&self) -> Box<dyn Node> {
        Box::new(Self::new())
    }

    fn inputs(&self) -> usize {
        0
    }
    fn outputs(&self) -> usize {
        1
    }
    fn show_input(&self, _pin: &InPin, _ui: &mut Ui, _scale: f32) -> PinInfo {
        PinInfo::square()
    }
    fn show_output(&self, _pin: &OutPin, _ui: &mut Ui, _scale: f32) -> PinInfo {
        PinInfo::square().with_fill(Color32::RED).with_wire_style(WireStyle::Bezier3)
    }
    fn can_rx(&self, _other: &Box<dyn Node>) -> bool {
        true
    }
    fn can_tx(&self, _other: &Box<dyn Node>) -> bool {
        true
    }
    fn context_menu(&self, ui: &mut Ui) {
        egui::Window::new("TEST").show(ui.ctx(), |ui| {
            ui.heading("EEEEE");
        });
        // if ui.button("Edit").clicked() {
        //     self.popup_open = !self.popup_open;
        // }
    }
    fn update(&self, ui: &mut Ui) {

    }
}

// #[derive(serde::Serialize, serde::Deserialize, Clone)]
// pub struct Constant_Edit_Popup {
//     pub data: Vec<String>,
//     pub has_changed: bool,
// }
// #[typetag::serde]
// impl Pane for Constant_Edit_Popup {
//     fn new() -> PaneState where Self: Sized {
//         let mut s = Self {
//             node: None,
//             has_changed: false,
//         };
//         PaneState {
//             id: s.name().to_string(),
//             mode: PaneMode::Popup,
//             pane: Box::new(s),
//         }
//     }
//     fn init(&mut self, _pcc: &PsudoCreationContext) {}
//     fn name(&mut self) -> &str {"ERROR"}
//     fn render(&mut self, _ui: &mut Ui){}
//     fn context_menu(&mut self, _ui: &mut Ui) {}
// }
