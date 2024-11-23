// use egui::{FontFamily, FontId, RichText, Visuals};
use eframe::egui_glow;
use std::{sync::Arc, time::Instant};
use egui::{accesskit::TextAlign, mutex::Mutex, Align2, Color32, FontId, Pos2, Stroke};
use egui_glow::glow;

use crate::{panes::Pane, point_cloud_renderer::PointRenderer, PaneManager};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    // #[serde(skip)]
    renderer: Arc<Mutex<PointRenderer>>,
    points: Vec<(i32, i32, i32, Color32)>,
    file_dialog_open: bool,
    cur_path: String,
}


impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Option<Self> {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Some(Self {
            renderer: Arc::new(Mutex::new(PointRenderer::new(cc.gl.clone(), 1_000_000))),
            points: Vec::new(),
            file_dialog_open: false,
            cur_path: "./".to_string(),
        })
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_switch(ui);
                
                if ui.button("Load PLY").clicked() {
                    self.file_dialog_open = true;
                }
            });
        });

        if self.file_dialog_open {
        egui::Window::new("Load PLY File")
            .show(ctx, |ui| {
                ui.label("Enter PLY file path:");
                ui.text_edit_singleline(&mut self.cur_path); // Add proper path handling
                
                ui.horizontal(|ui| {
                    if ui.button("Load").clicked() {
                        let renderer = &mut self.renderer.lock();
                            // Add proper path handling and error reporting
                            let ply = renderer.load_ply(self.cur_path.clone());
                            if let Err(e) = ply {
                                eprintln!("Failed to load PLY: {}", e);
                            }else{
                                // self.renderer.lock().camera.reset();
                                self.points = ply.unwrap();
                            }
                        
                        self.file_dialog_open = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.file_dialog_open = false;
                    }
                });
            });
        }


        egui::CentralPanel::default().show(ctx, |ui| {
            // egui::scroll_area::ScrollArea::vertical().show(ui, |ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    self.custom_painting(ui.max_rect(), ui);
                });
            // })
        });

    }

    // fn on_exit(&mut self, gl: Option<&glow::Context>) {
    //     if let Some(gl) = gl {
    //         self.rotating_triangle.lock().destroy(gl);
    //     }
    // }

    // Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }
}

impl App {
    fn custom_painting(&mut self, max_rect: egui::Rect, ui: &mut egui::Ui) {

        let start_time = Instant::now();

        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2 { x: max_rect.width(), y: max_rect.height() }, egui::Sense::drag());

        let input_state = ui.input(|input_state| {input_state.clone()});

        // ui.painter();.

        // println!("{}",response.drag_motion().x);

        // let response = Box::new(response);

        // let ui = ui.to_owned();

        // self.anglex += response.drag_motion().x * 0.01;
        // self.angley += response.drag_motion().y * 0.01;

        // Clone locals so we can move them into the paint callback:
        if self.points.is_empty() {
           let radius = 1000i32;
           for i in 0..100000 {
            //    let theta = (i as f32 * 0.1).sin() * std::f32::consts::PI;
            //    let phi = (i as f32 * 0.1).cos() * std::f32::consts::PI;
               
               let x = (radius as f32 * (i as f32).cos()) as i32;
               let y = (radius as f32 * (i as f32).sin()) as i32;
               let z = (i as f32 * 0.05) as i32;
               
                // let x = (i as f32 * 0.1) as u32;
                // let y = (i as f32 * 0.1) as u32 ;
                // let z = (i as f32 * 0.1) as u32;

               // Color based on position
               let color = Color32::from_rgba_premultiplied(
                   ((x as f32 / radius as f32) * 255.0) as u8,
                   ((y as f32 / radius as f32) * 255.0) as u8,
                   ((z as f32 / radius as f32) * 255.0) as u8,
                   255,
               );
               
               self.points.push((x, y, z, color));
           }
        }

        let renderer = self.renderer.clone();
        renderer.lock().clear();

        // let painter = ui.painter();

        for &(x, y, z, color) in &self.points {
            renderer.lock().add_point(x, y, z, color);
        }

        let o = renderer.lock().camera.orientation.clone();

        let cb = egui_glow::CallbackFn::new(move |_info, _painter| {
            renderer.lock().render(rect, input_state.clone());
        });

        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(cb),
        };

        ui.painter().add(callback);

        let pos1 = o.inverse()*glam::Vec3::X;
        let pos2 = o.inverse()*glam::Vec3::Y;
        let pos3 = o.inverse()*glam::Vec3::Z;

        let line_length:f32 = 20.;

        ui.painter().line_segment([rect.center(), rect.center() + egui::Vec2{ x: line_length*pos1.x, y: -line_length*pos1.y,}], Stroke {
            width: 1.5,
            color: Color32::RED,
        });


        ui.painter().line_segment([rect.center(), rect.center() + egui::Vec2{ x: line_length*pos2.x, y: -line_length*pos2.y,}], Stroke {
            width: 1.5,
            color: Color32::BLUE,
        });


        ui.painter().line_segment([rect.center(), rect.center() + egui::Vec2{ x: line_length*pos3.x, y: -line_length*pos3.y,}], Stroke {
            width: 1.5,
            color: Color32::GREEN,
        });

        let end_time = Instant::now();

        println!("{}", end_time.duration_since(start_time).as_millis());

        ui.painter().text(Pos2 {x:0.,y:0.}, Align2::LEFT_TOP, format!("{}",end_time.duration_since(start_time).as_millis()), FontId::monospace(12.), Color32::WHITE);



    }
}