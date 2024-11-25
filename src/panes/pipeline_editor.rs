use crate::pane_manager::{Pane, PaneMode, PaneState, PsudoCreationContext};


use egui::{Color32, Id, Pos2, Ui};
use egui_snarl::{
    ui::{PinInfo, SnarlStyle, SnarlViewer},
    InPin, NodeId, OutPin, Snarl,
};
use egui_snarl::ui::{PinShape, WireStyle};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PipelinePane {
    snarl: Option<Snarl<Box<dyn Node>>>,
    style: Option<SnarlStyle>,
    snarl_ui_id: Option<Id>,
}
#[typetag::serde]
impl Pane for PipelinePane {
    fn new() -> PaneState
    where
        Self: Sized,
    {
        let mut s = Self {
            snarl: Some(Snarl::new()),
            style: Some(SnarlStyle::new()),
            snarl_ui_id: None,
        };
        PaneState {
            id: s.name().to_string(),
            mode: PaneMode::Center,
            pane: Box::new(s),
        }
    }
    fn init(&mut self, _cc: &PsudoCreationContext) {}
    fn name(&mut self) -> &str {
        "Pipeline Pane"
    }
    fn render(&mut self, ui: &mut Ui) {
        self.snarl_ui_id = Some(ui.id());

        if let Some(snarl) = &mut self.snarl {
            if let Some(style) = &self.style {
                snarl.show(&mut NodeViewer, style, "snarl", ui);
            }
        }


    }
    fn context_menu(&mut self, ui: &mut Ui) {

        ui.menu_button("Add Node", |ui| {
            if let Some(snarl) = &mut self.snarl {
                NodeViewer::add_node_menu(Pos2 { x: 0., y: 0. }, ui, snarl);
            }
        });
        if ui.button("Run").clicked() {
            ui.close_menu();
            self.run();
        }
        // if !self.snarl.is_none() {
        //     self.snarl.unwrap().add_node_menu(ui, ui.clip_rect().min.clone(), )
        // }
    }
}

impl PipelinePane {
    pub fn run(&mut self) {
        // Todo:
    }
}

fn format_float(v: f64) -> String {
    let v = (v * 1000.0).round() / 1000.0;
    format!("{}", v)
}

#[typetag::serde(tag = "type")]
trait Node {
    fn new() -> Self
    where
        Self: Sized;
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn duplicate(&self) -> Box<dyn Node>;
    fn inputs(&self) -> usize;
    fn outputs(&self) -> usize;
    fn show_input(&self, pin: &InPin, ui: &mut Ui, scale: f32) -> PinInfo;
    fn show_output(&self, pin: &OutPin, ui: &mut Ui, scale: f32) -> PinInfo;
    fn can_rx(&self, other: &Box<dyn Node>) -> bool;
    fn can_tx(&self, other: &Box<dyn Node>) -> bool;
    fn context_menu(&self, ui: &mut Ui);
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Node1;
#[typetag::serde]
impl Node for Node1 {
    fn new() -> Self {
        Self
    }

    fn get_name(&self) -> &str {
        "Test"
    }
    fn get_description(&self) -> &str {"Test Node"}

    fn duplicate(&self) -> Box<dyn Node> {
        Box::new(Self::new())
    }

    fn inputs(&self) -> usize {
        1
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
        ui.label("Test!");
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct NodeViewer;

impl SnarlViewer<Box<dyn Node>> for NodeViewer {
    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Box<dyn Node>>) {
        // Validate connection

        let rx = snarl.get_node(to.id.node).unwrap();
        let tx = snarl.get_node(from.id.node).unwrap();

        if rx.can_rx(tx) && tx.can_tx(rx) {
            for &remote in &to.remotes {
                snarl.disconnect(remote, to.id);
            }

            snarl.connect(from.id, to.id);
        }
    }

    fn title(&mut self, node: &Box<dyn Node>) -> String {
        node.get_name().to_string()
    }

    fn outputs(&mut self, node: &Box<dyn Node>) -> usize {
        node.outputs()
    }

    fn inputs(&mut self, node: &Box<dyn Node>) -> usize {
        node.inputs()
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<Box<dyn Node>>,
    ) -> PinInfo {
        snarl
            .get_node(pin.id.node)
            .unwrap()
            .show_input(pin, ui, scale)
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<Box<dyn Node>>,
    ) -> PinInfo {
        snarl
            .get_node(pin.id.node)
            .unwrap()
            .show_output(pin, ui, scale)
    }


    fn has_graph_menu(&mut self, _pos: Pos2, _snarl: &mut Snarl<Box<dyn Node>>) -> bool {
        true
    }

    fn show_graph_menu(
        &mut self,
        pos: egui::Pos2,
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<Box<dyn Node>>,
    ) {
        NodeViewer::add_node_menu(pos, ui, snarl);
    }

    fn has_on_hover_popup(&mut self, _: &Box<dyn Node>) -> bool {
        true
    }

    fn show_on_hover_popup(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<Box<dyn Node>>,
    ) {
        ui.label(snarl.get_node(node).unwrap().get_description());
    }

    fn has_node_menu(&mut self, _node: &Box<dyn Node>) -> bool {
        true
    }

    fn show_node_menu(
        &mut self,
        nodeid: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<Box<dyn Node + 'static>>,
    ) {
        ui.label("Node menu");
        if ui.button("Remove").clicked() {
            snarl.remove_node(nodeid);
            ui.close_menu();
        } else if ui.button("Duplicate").clicked() {
            snarl.insert_node(Pos2 {x:0.,y:0.}, snarl.get_node(nodeid).unwrap().duplicate());
            ui.close_menu();
        } else {
            snarl.get_node(nodeid).unwrap().context_menu(ui);
        }
    }
}

impl NodeViewer {
    pub fn add_node_menu(pos: Pos2, ui: &mut Ui, snarl: &mut Snarl<Box<dyn Node>>) {
        ui.label("Add node");
        let button = ui.button("Test1");
        if button.clicked() {
            snarl.insert_node(pos, Box::new(Node1::new()));
            ui.close_menu();
        }
    }
}