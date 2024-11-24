use egui::{Ui, Color32, Stroke};
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

pub trait Pane {
    fn new(cc: &eframe::CreationContext<'_>) -> PaneState where Self: Sized;
    fn name(&mut self) -> &str;
    fn render(&mut self, ui: &mut Ui);
    fn context_menu(&mut self, ui: &mut Ui);
}

// #[derive(serde::Deserialize, serde::Serialize)]
pub struct PaneState {
    // #[serde(skip)]
    pub pane: Box<dyn Pane>,
    pub id: String,
    pub mode: PaneMode,
    // pub window_location: Pos2,
}

// pub struct NoPane {}
// impl Pane for NoPane {
//     fn name(&mut self) -> &str {"ERROR"}
//     fn render(&mut self, _ui: &mut Ui){}
//     fn context_menu(&mut self, _ui: &mut Ui) {}
// }


// impl Default for dyn Pane {
//     fn default() -> (impl Pane + 'static)  {
//         Box::new(NoPane {})
//     }
// }

impl PaneState {
    pub fn render(&mut self, ui: &mut Ui) {
        self.pane.render(ui);
    }
}

// pub struct PaneGroup {
//     pub direction: TileDirection,
//     pub panes: Vec<PaneState>,
// }

// #[derive(serde::Deserialize, serde::Serialize)]
pub struct PaneManager {
    pub panes: Vec<PaneState>,
}



pub struct BluePane {}
impl Pane for BluePane {
    fn new(cc: &eframe::CreationContext<'_>) -> PaneState where Self: Sized {

        let mut s = Self {};
        PaneState {
            id: s.name().to_string(),
            mode: PaneMode::Left,
            pane: Box::new(s),
        }
    }
    fn name(&mut self) -> &str {"BLUE"}
    fn render(&mut self, ui: &mut Ui){
        ui.painter().rect(ui.max_rect(), 0., Color32::BLUE, Stroke::NONE);
    }
    fn context_menu(&mut self, _ui: &mut Ui) {}
}

pub struct GreenPane {}
impl Pane for GreenPane {
    fn new(cc: &eframe::CreationContext<'_>) -> PaneState where Self: Sized {
        let mut s = Self {};
        PaneState {
            id: s.name().to_string(),
            mode: PaneMode::Bottom,
            pane: Box::new(s),
        }
    }
    fn name(&mut self) -> &str {"Green"}
    fn render(&mut self, ui: &mut Ui){
        ui.painter().rect(ui.max_rect(), 0., Color32::GREEN, Stroke::NONE);
    }
    fn context_menu(&mut self, _ui: &mut Ui) {}
}

impl PaneManager {
    pub fn new(cc: Option<&eframe::CreationContext<'_>>) -> Self {
        if let Some(cc) = cc {
            Self {
                panes: vec![
                    BluePane::new(cc),
                    GreenPane::new(cc),
                    // PaneState {pane: Box::new(BluePane{}), id: "iqnhjqnbekjq".to_string(), mode: PaneMode::Windowed},
                    // PaneState {pane: Box::new(GreenPane{}), id: "kjwkjwqfd".to_string(), mode: PaneMode::Right}, 
                    crate::point_cloud_renderer::PointRendererPane::new(cc),
                ],
            }
        } else {
            Self {
                panes: vec![]
            }
        }
    }


    pub fn render(&mut self, ui: &mut Ui){
        let len = self.panes.len();

        egui::TopBottomPanel::top("top_panel").show(ui.ctx(), |ui| {

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!

                egui::widgets::global_theme_preference_switch(ui);
                
                ui.menu_button("File", |ui| {
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
                // self.
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
}

// impl 








// use eframe::egui;
// use egui::Stroke;
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use std::fs;
// use egui::Color32;

// #[derive(Serialize, Deserialize, Clone, PartialEq)]
// enum PaneMode {
//     Hidden,
//     Tiled,
//     Windowed,
// }

// #[derive(Serialize, Deserialize, Clone, PartialEq)]
// enum SplitDirection {
//     Horizontal,
//     Vertical,
// }

// #[derive(Serialize, Deserialize, Clone)]
// struct GroupNode {
//     direction: SplitDirection,
//     children: Vec<LayoutNode>,
//     sizes: Vec<f32>, // Stores the relative sizes of children
// }

// #[derive(Serialize, Deserialize, Clone)]
// enum LayoutNode {
//     Pane(String), // Stores pane identifier
//     Group(GroupNode),
// }

// #[derive(Serialize, Deserialize, Clone)]
// struct LayoutConfig {
//     root: Option<LayoutNode>,
//     windowed_panes: HashMap<String, WindowedPaneConfig>,
// }

// #[derive(Serialize, Deserialize, Clone)]
// struct WindowedPaneConfig {
//     position: [f32; 2],
//     size: [f32; 2],
// }

// pub trait Pane {
//     fn name(&self) -> &str;
//     fn show(&mut self, ui: &mut egui::Ui);
//     fn default_size(&self) -> egui::Vec2;
// }

// // Example panes implementation
// struct ConsolePane {
//     content: String,
// }

// impl Pane for ConsolePane {
//     fn name(&self) -> &str { "Console" }
//     fn show(&mut self, ui: &mut egui::Ui) {
//         ui.painter().rect(ui.max_rect(), 0., Color32::RED, Stroke::NONE);
//         ui.text_edit_multiline(&mut self.content);
//     }
//     fn default_size(&self) -> egui::Vec2 { egui::vec2(300.0, 200.0) }
// }

// struct PropertiesPane {
//     value: f32,
// }

// impl Pane for PropertiesPane {
//     fn name(&self) -> &str { "Properties" }
//     fn show(&mut self, ui: &mut egui::Ui) {
//         ui.painter().rect(ui.max_rect(), 0., Color32::BLUE, Stroke::NONE);
//         ui.add(egui::Slider::new(&mut self.value, 0.0..=100.0));
//     }
//     fn default_size(&self) -> egui::Vec2 { egui::vec2(200.0, 300.0) }
// }

// struct DragState {
//     dragged_pane: String,
//     original_group_path: Vec<usize>,
//     drag_started: bool,
//     drop_target: Option<DropTarget>,
// }

// #[derive(Clone)]
// struct DropTarget {
//     target_pane: String,
//     group_path: Vec<usize>,
//     position: DropPosition,
// }

// #[derive(Clone, PartialEq)]
// enum DropPosition {
//     Above,
//     Below,
//     Left,
//     Right,
//     Center,
// }

// pub struct PaneManager {
//     panes: HashMap<String, Box<dyn Pane>>,
//     layout: LayoutConfig,
//     drag_state: Option<DragState>,
// }

// impl PaneManager {
//     pub fn new() -> Self {
//         let mut panes: HashMap<String, Box<dyn Pane>> = HashMap::new();
//         panes.insert("properties".to_string(), Box::new(PropertiesPane { value: 50.0 }));
//         panes.insert("console".to_string(), Box::new(ConsolePane { content: String::new() }));

//         // Create initial layout
//         let initial_group = GroupNode {
//             direction: SplitDirection::Vertical,
//             children: vec![
//                 LayoutNode::Pane("console".to_string()),
//                 LayoutNode::Pane("properties".to_string()),
//             ],
//             sizes: vec![0.5, 0.5],
//         };

//         let layout = LayoutConfig {
//             root: Some(LayoutNode::Group(initial_group)),
//             windowed_panes: HashMap::new(),
//         };

//         Self {
//             panes,
//             layout,
//             drag_state: None,
//         }
//     }

//     fn save_layout(&self) {
//         if let Ok(json) = serde_json::to_string_pretty(&self.layout) {
//             let _ = fs::write("layout.json", json);
//         }
//     }

//     fn load_layout(&mut self) {
//         if let Ok(contents) = fs::read_to_string("layout.json") {
//             if let Ok(layout) = serde_json::from_str(&contents) {
//                 self.layout = layout;
//             }
//         }
//     }

//     fn handle_drop(&mut self) {
//         if let Some(drag_state) = &self.drag_state {
//             if let Some(drop_target) = &drag_state.drop_target {
//                 let mut new_layout = self.layout.clone();
                
//                 // Remove dragged pane from original location
//                 self.remove_pane_from_path(&mut new_layout.root, &drag_state.original_group_path, &drag_state.dragged_pane);
                
//                 // Insert at new location based on drop position
//                 self.insert_pane_at_target(
//                     &mut new_layout.root,
//                     &drop_target.group_path,
//                     &drop_target.target_pane,
//                     &drag_state.dragged_pane,
//                     &drop_target.position,
//                 );
                
//                 self.layout = new_layout;
//             }
//         }
//         self.drag_state = None;
//     }

//     fn remove_pane_from_path(
//         &self,
//         node: &mut Option<LayoutNode>,
//         path: &[usize],
//         pane_id: &str,
//     ) -> bool {
//         if path.is_empty() {
//             return false;
//         }

//         if let Some(LayoutNode::Group(group)) = node {
//             if path.len() == 1 {
//                 if let Some(idx) = group.children.iter().position(|child| {
//                     matches!(child, LayoutNode::Pane(id) if id == pane_id)
//                 }) {
//                     group.children.remove(idx);
//                     group.sizes.remove(idx);
//                     return true;
//                 }
//             } else if path[0] < group.children.len() {
//                 return self.remove_pane_from_path(
//                     &mut Some(group.children[path[0]].clone()),
//                     &path[1..],
//                     pane_id,
//                 );
//             }
//         }
//         false
//     }

//     fn insert_pane_at_target(
//         &self,
//         node: &mut Option<LayoutNode>,
//         path: &[usize],
//         target_pane: &str,
//         dragged_pane: &str,
//         position: &DropPosition,
//     ) {
//         if let Some(LayoutNode::Group(group)) = node {
//             if path.len() == 1 {
//                 let target_idx = group.children.iter().position(|child| {
//                     matches!(child, LayoutNode::Pane(id) if id == target_pane)
//                 }).unwrap();
//                 // if target_idx.is_none(){ return; }
//                 // target_idx = target_idx.unwrap();
//                 // target_idx = target_idx.unwrap();

//                 match (position, &group.direction) {
//                     (DropPosition::Above | DropPosition::Below, SplitDirection::Vertical)
//                     | (DropPosition::Left | DropPosition::Right, SplitDirection::Horizontal) => {
//                         // Insert in same group
//                         let insert_idx = if matches!(position, DropPosition::Below | DropPosition::Right) {
//                             target_idx + 1
//                         } else {
//                             target_idx
//                         };
//                         group.children.insert(insert_idx, LayoutNode::Pane(dragged_pane.to_string()));
//                         group.sizes.insert(insert_idx, 1.0 / (group.sizes.len() + 1) as f32);
//                         // Normalize sizes
//                         let total: f32 = group.sizes.iter().sum();
//                         for size in &mut group.sizes {
//                             *size /= total;
//                         }
//                     }
//                     _ => {
//                         // Create new group
//                         let new_direction = if matches!(position, DropPosition::Above | DropPosition::Below) {
//                             SplitDirection::Vertical
//                         } else {
//                             SplitDirection::Horizontal
//                         };

//                         let mut new_group = GroupNode {
//                             direction: new_direction,
//                             children: vec![],
//                             sizes: vec![],
//                         };

//                         if matches!(position, DropPosition::Above | DropPosition::Left) {
//                             new_group.children.push(LayoutNode::Pane(dragged_pane.to_string()));
//                             new_group.children.push(LayoutNode::Pane(target_pane.to_string()));
//                         } else {
//                             new_group.children.push(LayoutNode::Pane(target_pane.to_string()));
//                             new_group.children.push(LayoutNode::Pane(dragged_pane.to_string()));
//                         }
//                         new_group.sizes = vec![0.5, 0.5];

//                         group.children[target_idx] = LayoutNode::Group(new_group);
//                     }
//                 }
//             } else if path[0] < group.children.len() {
//                 self.insert_pane_at_target(
//                     &mut Some(group.children[path[0]].clone()),
//                     &path[1..],
//                     target_pane,
//                     dragged_pane,
//                     position,
//                 );
//             }
//         }
//     }

//     fn show_group(
//         &mut self,
//         ui: &mut egui::Ui,
//         group: &GroupNode,
//         path: &mut Vec<usize>,
//         rect: egui::Rect,
//     ) {
//         let mut current_offset = if group.direction == SplitDirection::Horizontal {
//             rect.left()
//         } else {
//             rect.top()
//         };

//         for (idx, (child, &size)) in group.children.iter().zip(group.sizes.iter()).enumerate() {
//             path.push(idx);

//             let child_size = if group.direction == SplitDirection::Horizontal {
//                 size * rect.width()
//             } else {
//                 size * rect.height()
//             };

//             let child_rect = if group.direction == SplitDirection::Horizontal {
//                 egui::Rect::from_min_size(
//                     egui::pos2(current_offset, rect.top()),
//                     egui::vec2(child_size, rect.height()),
//                 )
//             } else {
//                 egui::Rect::from_min_size(
//                     egui::pos2(rect.left(), current_offset),
//                     egui::vec2(rect.width(), child_size),
//                 )
//             };

//             match child {
//                 LayoutNode::Pane(id) => {
//                     // let pane = self.panes.get_mut(id).unwrap();
//                     // if !pane.is_none(){
//                     self.show_pane(ui, id, child_rect, path.clone());
//                     // }
//                 }
//                 LayoutNode::Group(child_group) => {
//                     self.show_group(ui, child_group, path, child_rect);
//                 }
//             }

//             current_offset += child_size;
//             path.pop();
//         }
//     }

//     fn show_pane(
//         &mut self,
//         ui: &mut egui::Ui,
//         id: &String,
//         // pane: &mut Box<dyn Pane>,
//         rect: egui::Rect,
//         path: Vec<usize>,
//     ) {
//         let pane = self.panes.get_mut(id).unwrap();
//         let response = ui.allocate_rect(rect, egui::Sense::drag());
        
//         if response.dragged() {
//             if self.drag_state.is_none() {
//                 self.drag_state = Some(DragState {
//                     dragged_pane: pane.name().to_string(),
//                     original_group_path: path.clone(),
//                     drag_started: true,
//                     drop_target: None,
//                 });
//             }
//         }

//         // Handle drop targeting
//         if let Some(drag_state) = &mut self.drag_state {
//             if drag_state.dragged_pane != pane.name() {
//                 let hover_pos: Option<egui::Pos2> = ui.input(|i| {i.pointer.hover_pos().clone()});
//                 if let Some(pos) = hover_pos {
//                     if rect.contains(pos) {
//                         let relative_pos = (pos - rect.min) / rect.size();
//                         let position = if relative_pos.y < 0.25 {
//                             DropPosition::Above
//                         } else if relative_pos.y > 0.75 {
//                             DropPosition::Below
//                         } else if relative_pos.x < 0.25 {
//                             DropPosition::Left
//                         } else if relative_pos.x > 0.75 {
//                             DropPosition::Right
//                         } else {
//                             DropPosition::Center
//                         };

//                         drag_state.drop_target = Some(DropTarget {
//                             target_pane: pane.name().to_string(),
//                             group_path: path,
//                             position,
//                         });
//                     }
//                 }
//             }
//         }

//         // Show the actual pane content
//         let frame = egui::Frame::none()
//             .fill(ui.style().visuals.window_fill)
//             .stroke(ui.style().visuals.window_stroke)
//             .inner_margin(egui::Margin::same(4.0));

//         frame.show(ui, |ui| {
//             ui.set_min_size(rect.size());
//             pane.show(ui);
//         });
//     }

//     // fn is_valid_group
// }

// impl eframe::App for PaneManager {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
//             ui.horizontal(|ui| {
//                 ui.menu_button("File", |ui| {
//                     if ui.button("Save Layout").clicked() {
//                         self.save_layout();
//                         ui.close_menu();
//                     }
//                     if ui.button("Load Layout").clicked() {
//                         self.load_layout();
//                         ui.close_menu();
//                     }
//                 });
//             });
//         });

//         let any_down = ctx.input(|i| {i.pointer.any_down().clone()});
//         if !any_down {// && drag_state.drag_started {
//             self.handle_drop();
//         }

//         egui::CentralPanel::default().show(ctx, |ui| {

//             let mut root_group = if let LayoutNode::Group(c) = <std::option::Option<LayoutNode> as Clone>::clone(&self.layout.root).unwrap() {c} else { unreachable!() };

//             // if let Some(LayoutNode::Group(root_group)) = &self.layout.root {
            


//             // // let root = LayoutNode::Group(&mut self.layout.root.unwrap());

//             // // if LayoutNode::Group(root_group).unwrap()



//                 let mut path = Vec::new();
//                 self.show_group(ui, &root_group, &mut path, ui.available_rect_before_wrap());
            
//         });

//         // Handle drag end
//         // if let Some(drag_state) = &self.drag_state {

//         // }
//     }
// }
