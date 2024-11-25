use egui::{Color32, InputState, Rect};
use eframe::egui_glow;
use egui_glow::glow;
use std::sync::Arc;
use glam::{Vec3, Mat4, Quat};
use std::fs::File;
// use std::path::Path;
use std::io::{BufReader, BufRead};
use crate::pane_manager::{Pane, PaneMode, PaneState, PsudoCreationContext};
use std::sync::Mutex;
use egui::FontId;
use egui::Align2;
// use egui::Pos2;
use std::time::Instant;
use egui::Stroke;
use egui::Ui;

// Shader sources updated for 3D rendering with fixed-point positions
const VERTEX_SHADER: &str = r#"
    #version 330 core
    layout (location = 0) in ivec3 position;  // Using unsigned ints for position
    layout (location = 1) in ivec4 color;     // Using unsigned ints for color
    
    uniform mat4 u_view_projection;
    uniform float u_position_scale;  // Scale factor to convert from uint to world space
    uniform float u_point_size_scale;  // Added point size scaling

    out vec4 v_color;
    
    void main() {
        // Convert uint positions to world space
        vec3 worldPos = vec3(position) * u_position_scale;
        gl_Position = u_view_projection * vec4(worldPos, 1.0);
        gl_PointSize = max(u_point_size_scale * 10.0 * (1.0 - gl_Position.z / gl_Position.w), 1.0);
        v_color = vec4(color) / 255.0;  // Convert uint colors to float
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 330 core
    in vec4 v_color;
    out vec4 FragColor;
    
    void main() {
        // Create circular points
        vec2 coord = gl_PointCoord * 2.0 - 1.0;
        float r = dot(coord, coord);
        if (r > 1.0) discard;
        // if (coord.x > 1.0) discard;
        // if (coord.y > 1.0) discard;
        
        // Apply simple lighting based on depth
        // float depth = gl_FragCoord.z;
        FragColor = v_color;
    }
"#;

// Camera controller for 3D navigation
#[derive(Clone)]
pub struct Camera {
    position: Vec3,
    pub orientation: Quat,
    distance: f32,
    pub point_size_scale: f32,
    // last_pos: Option<Pos2>,
}


impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 5.0),
            orientation: Quat::IDENTITY,
            distance: 5.0,
            point_size_scale: 0.1,
            // last_pos: None,
        }
    }

    // pub fn reset(&mut self) {
    //     self.position = Vec3::new(0.0, 0.0, 5.0);
    //     self.orientation = Quat::IDENTITY;
    //     self.distance = 5.0;
    //     // self.point_size_scale = 0.1;
    //     self.update_view();
    // }

    pub fn update(&mut self, i: InputState) {
            let mut changed = false;
            
            if i.pointer.secondary_down() && !i.modifiers.shift {
                let delta = i.pointer.delta();
                
                let rotation_speed = 0.01;
                let pitch = delta.y * rotation_speed;
                let yaw = delta.x * rotation_speed;
                
                let pitch_rotation = Quat::from_axis_angle(Vec3::X, -pitch);
                let yaw_rotation = Quat::from_axis_angle(Vec3::Y, -yaw);
                let roll_rotation = Quat::from_axis_angle(Vec3::Z, 0.);
                
                self.orientation = self.orientation * pitch_rotation * yaw_rotation * roll_rotation;
                self.orientation = self.orientation.normalize();
                
                changed = true;
            } // else if i.pointer.secondary_down() && i.modifiers.shift {
            //     let cur_pos = i.pointer.latest_pos();

            //     if let Some(last_pos) = self.last_pos {
            //         let last_angle = f32::atan2(last_pos.y, last_pos.x);
            //         if let Some(cur_pos) = cur_pos {
            //             let cur_angle = f32::atan2(cur_pos.y, cur_pos.x);

            //             println!("{}",cur_angle - last_angle);

            //             let pitch_rotation = Quat::from_axis_angle(Vec3::X, 0.);
            //             let yaw_rotation = Quat::from_axis_angle(Vec3::Y, 0.);
            //             let roll_rotation = Quat::from_axis_angle(Vec3::Z,cur_angle-last_angle);

            //             self.orientation = self.orientation * pitch_rotation * yaw_rotation * roll_rotation;
            //             self.orientation = self.orientation.normalize();

            //             changed = true;

            //         }
            //     }

            //     self.last_pos = cur_pos;
            // }

            let zoom_delta = i.smooth_scroll_delta.x + i.smooth_scroll_delta.y;
            if zoom_delta != 0. {
                if i.modifiers.shift {
                    // self.point_size_scale =  (self.point_size_scale * (1. - zoom_delta * 0.001));
                    let scale_delta = zoom_delta * 0.01;
                    self.point_size_scale = (self.point_size_scale + scale_delta).clamp(0.1, 1000.0);
                    // println!("{}", self.point_size_scale);
                } else {
                    self.distance *= (1.0 - zoom_delta * 0.001).max(0.1);
                }
                changed = true;
            }
            
            if i.pointer.primary_down() {
                let delta = i.pointer.delta();
                let pan_speed = self.distance * 0.001;
            
                
                // Get camera-relative right and up vectors
                let right = self.get_right();
                let up = self.get_up();
                
                // Move camera in the camera plane
                let pan = right * (-delta.x * pan_speed) + up * (delta.y * pan_speed);
                self.position += pan;
                
                changed = true;
            }
            
            
   
        if changed {
            self.update_view();
        }
    }

    fn get_right(&self) -> Vec3 {
        self.orientation * Vec3::X
    }

    fn get_up(&self) -> Vec3 {
        self.orientation * Vec3::Y
    }

    fn get_forward(&self) -> Vec3 {
        self.orientation * -Vec3::Z
    }

    fn update_view(&mut self) {
        // Ensure orientation stays normalized
        self.orientation = self.orientation.normalize();
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        // Calculate view position by moving back from target along view direction
        let forward = self.get_forward();
        let view_pos = self.position - forward * self.distance;
        
        Mat4::look_at_rh(
            view_pos,
            self.position,
            self.get_up()
        )
    }

    // pub fn set_point_size_scale(&mut self, scale: f32) {
    //     self.point_size_scale = scale.clamp(0.1, 10.0);
    // }

}


// PLY parsing structures
#[derive(Debug)]
struct PlyHeader {
    vertex_count: usize,
    has_colors: bool,
    is_binary: bool,
}

// #[derive(Debug)]
// pub struct PlyPoint {
//     position: (i32, i32, i32),
//     color: Color32,
// }


#[derive(Default)]
pub struct PointRenderer {
    pub gl:      Option<Arc<glow::Context>>,
    program: Option<glow::Program>,
    vao:     Option<glow::VertexArray>,
    vbo:     Option<glow::Buffer>,
    points:  Option<Vec<i32>>,
    // capacity: usize,
    pub camera: Option<Camera>,
}

// impl Defalt for PointRenderer {
//     fn default() -> Self {
//         Self {
//             gl:      Option<Arc<glow::Context>>,
//             program: Option<glow::Program>,
//             vao:     Option<glow::VertexArray>,
//             vbo:     Option<glow::Buffer>,
//             points:  Option<Vec<i32>>,
//             // capacity: usize,
//             camera: Option<Camera>,
//         }
//         }
//     }
// }

impl PointRenderer {
    pub fn init(&mut self, gl: Option<Arc<glow::Context>>, initial_capacity: usize) {
        use glow::HasContext;

        let gl = gl.unwrap();
        
        let program = unsafe {
            let program = gl.create_program().expect("Cannot create program");
            
            let vertex_shader = gl.create_shader(glow::VERTEX_SHADER)
                .expect("Cannot create vertex shader");
            gl.shader_source(vertex_shader, VERTEX_SHADER);
            gl.compile_shader(vertex_shader);
            
            let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER)
                .expect("Cannot create fragment shader");
            gl.shader_source(fragment_shader, FRAGMENT_SHADER);
            gl.compile_shader(fragment_shader);
            
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);
            
            program
        };
        
        let vao = unsafe {
            let vao = gl.create_vertex_array().expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vao));
            vao
        };
        
        let vbo = unsafe {
            let vbo = gl.create_buffer().expect("Cannot create vertex buffer");
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            
            // Position (3) + Color (4) = 7 u32s per vertex
            let buffer_size = initial_capacity * 7 * std::mem::size_of::<i32>();
            gl.buffer_data_size(glow::ARRAY_BUFFER, buffer_size as i32, glow::DYNAMIC_DRAW);
            
            // Position attribute (uvec3)
            gl.vertex_attrib_pointer_i32(0, 3, glow::INT, 28, 0);
            gl.enable_vertex_attrib_array(0);
            
            // Color attribute (uvec4)
            gl.vertex_attrib_pointer_i32(1, 4, glow::INT, 28, 12);
            gl.enable_vertex_attrib_array(1);
            
            vbo
        };
        
        self.gl = Some(gl);
        self.program = Some(program);
        self.vao = Some(vao);
        self.vbo = Some(vbo);
        self.points = Some(Vec::with_capacity(initial_capacity * 7));
            // capacity: initial_capacity,
        self.camera = Some(Camera::new());
    }
    
    pub fn add_point(&mut self, x: i32, y: i32, z: i32, color: Color32) {
        let [r, g, b, a] = color.to_array();
        self.points.as_mut().as_mut().expect("Not Initialised").extend_from_slice(&[x, y, z, r as i32, g as i32, b as i32, a as i32]);
    }
    
    pub fn clear(&mut self) {
        self.points.as_mut().as_mut().expect("Not Initialised").clear();
    }
    
    pub fn render(&mut self, rect: Rect, input_state: Option<InputState>) {
        use glow::HasContext;
        
        // Update camera
        if let Some(i) = input_state{
            self.camera.as_mut().expect("Not Initialised").update(i);
        }
        
        unsafe {
            self.gl.as_mut().expect("Not Initialised").use_program(self.program);
            
            // Set up view-projection matrix
            let aspect = rect.width() / rect.height();
            let projection = Mat4::perspective_rh(45.0f32.to_radians(), aspect, 0.1, 1000.0);
            let view = self.camera.as_mut().expect("Not Initialised").get_view_matrix();
            let view_projection = projection * view;
            
            let location = self.gl.as_mut().expect("Not Initialised").get_uniform_location(*self.program.as_mut().expect("Not Initialised"), "u_view_projection")
                .expect("Cannot get uniform location");
            self.gl.as_mut().expect("Not Initialised").uniform_matrix_4_f32_slice(Some(&location), false, &view_projection.to_cols_array());
            
            // Set position scale factor (converts uint positions to world space)
            let scale_location = self.gl.as_mut().expect("Not Initialised").get_uniform_location(*self.program.as_mut().expect("Not Initialised"), "u_position_scale")
                .expect("Cannot get scale uniform location");
            self.gl.as_mut().expect("Not Initialised").uniform_1_f32(Some(&scale_location), 0.001); // Adjust this value to scale your point cloud
            
            let point_size_location = self.gl.as_mut().expect("Not Initialised").get_uniform_location(*self.program.as_mut().expect("Not Initialised"), "u_point_size_scale")
                .expect("Cannot get point size scale location");
            self.gl.as_mut().expect("Not Initialised").uniform_1_f32(Some(&point_size_location), self.camera.as_mut().expect("Not Initialised").point_size_scale);

            self.gl.as_mut().expect("Not Initialised").bind_vertex_array(self.vao);
            self.gl.as_mut().expect("Not Initialised").bind_buffer(glow::ARRAY_BUFFER, Some(*self.vbo.as_mut().expect("Not Initialised")));
            
            self.gl.as_mut().expect("Not Initialised").buffer_sub_data_u8_slice(
                glow::ARRAY_BUFFER,
                0,
                bytemuck::cast_slice(&self.points.as_mut().expect("Not Initialised")),
            );
            
            self.gl.as_mut().expect("Not Initialised").enable(glow::PROGRAM_POINT_SIZE);
            self.gl.as_mut().expect("Not Initialised").enable(glow::DEPTH_TEST);


            self.gl.as_mut().expect("Not Initialised").clear_depth_f32(1.0);
            self.gl.as_mut().expect("Not Initialised").depth_func(glow::LESS);
            self.gl.as_mut().expect("Not Initialised").depth_mask(true);

            // self.gl.clear_color(0.3, 0.3, 0.3, 1.0);
            self.gl.as_mut().expect("Not Initialised").clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            
            self.gl.as_mut().expect("Not Initialised").draw_arrays(glow::POINTS, 0, (self.points.as_mut().as_mut().expect("Not Initialised").len() / 7) as i32);
            
            self.gl.as_mut().expect("Not Initialised").disable(glow::DEPTH_TEST);
            self.gl.as_mut().expect("Not Initialised").disable(glow::PROGRAM_POINT_SIZE);
        }
    }











    // Add method to load points from PLY file
    pub fn load_ply(&mut self, path: String) -> Result<Vec<(i32, i32, i32, Color32)>, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Parse header
        let header = Self::parse_ply_header(&mut lines)?;
        
        // Clear existing points
        self.clear();
        
        // Reserve capacity
        self.points.as_mut().as_mut().expect("Not Initialised").reserve(header.vertex_count * 7);
        
        // Parse vertices based on format
        if header.is_binary {
            return Err("Binary PLY files not yet supported".to_string());
        } else {
            self.parse_ascii_ply_data(lines, header)
        }

    }

    fn parse_ply_header<B: BufRead>(lines: &mut std::io::Lines<B>) -> Result<PlyHeader, String> {
        let mut vertex_count = 0;
        let mut has_colors = false;
        let mut is_binary = false;
        let in_header = true;
        
        while in_header {
            let line = lines.next()
                .ok_or("Unexpected end of file").unwrap().unwrap()
                .trim().to_string();
                
            match line.as_str() {
                "ply" => continue,
                "format ascii 1.0" => is_binary = false,
                "format binary_little_endian 1.0" => is_binary = true,
                "end_header" => break,
                _ => {
                    if line.starts_with("element vertex ") {
                        vertex_count = line.split_whitespace()
                            .last()
                            .ok_or("Invalid vertex count")?
                            .parse()
                            .map_err(|_| "Invalid vertex count")?;
                    } else if line.starts_with("property") && line.contains("red") {
                        has_colors = true;
                    }
                }
            }
        }
        
        Ok(PlyHeader {
            vertex_count,
            has_colors,
            is_binary,
        })
    }

    fn parse_ascii_ply_data<B: BufRead>(
        &mut self,
        lines: std::io::Lines<B>,
        header: PlyHeader,
    ) -> Result<Vec<(i32, i32, i32, Color32)>, String> {
        let mut vec: Vec<(i32, i32, i32, Color32)> = Vec::new();

        for line in lines.take(header.vertex_count) {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            if parts.len() < 3 {
                return Err("Invalid vertex data".to_string());
            }
            
            // Parse position
            let x = parts[0].parse::<f32>().map_err(|_| "Invalid X coordinate")?;
            let y = parts[1].parse::<f32>().map_err(|_| "Invalid Y coordinate")?;
            let z = parts[2].parse::<f32>().map_err(|_| "Invalid Z coordinate")?;
            
            // Convert to fixed point (scale by 1000 for better precision)
            let x = (x * 1000.0) as i32;
            let y = (y * 1000.0) as i32;
            let z = (z * 1000.0) as i32;
            
            // Parse colors if present
            let color = if header.has_colors && parts.len() >= 6 {
                let r = parts[3].parse::<u8>().unwrap_or(255);
                let g = parts[4].parse::<u8>().unwrap_or(255);
                let b = parts[5].parse::<u8>().unwrap_or(255);
                Color32::from_rgb(r, g, b)
            } else {
                Color32::WHITE
            };
            
            vec.push((x,y,z,color));

            // self.add_point(x, y, z, color);
        }
        
        Ok(vec)
    }

}

impl Drop for PointRenderer {
    fn drop(&mut self) {
        // Clean up GPU resources
    }
}



















#[derive(serde::Serialize, serde::Deserialize)]
pub struct PointRendererPane {
    #[serde(skip)]
    renderer: Arc<Mutex<PointRenderer>>,
    #[serde(skip)]
    points: Vec<(i32, i32, i32, Color32)>,
    #[serde(skip)]
    file_dialog_open: bool,
    #[serde(skip)]
    cur_path: String,
}

// impl Default for PointRenderer {
//     fn default() -> Self {
//         Self {}
//     }
// }

#[typetag::serde]
impl Pane for PointRendererPane {
    fn new() -> PaneState where Self: Sized {
        let renderer = PointRenderer::default();
        let mut s = Self {
            renderer: Arc::new(Mutex::new(renderer)),
            points: Vec::new(),
            file_dialog_open: false,
            cur_path: "./".to_string(),
        };
        PaneState {
            id: s.name().to_string(),
            mode: PaneMode::Hidden,
            pane: Box::new(s),
        }
    }
    fn init(&mut self, pcc: &PsudoCreationContext){
        self.renderer.lock().expect("Renderer Not Initialized").init(pcc.gl.clone(), 1_000_000);
    }
    fn name(&mut self) -> &str {"Point Cloud"}
    fn render(&mut self, ui: &mut Ui){

        let max_rect = ui.max_rect();

        let renderer = self.renderer.clone();
        if renderer.lock().expect("Renderer Not Initialized").gl.is_none() {
            return;
            // renderer.lock().expect("Renderer Not Initialized").init(ui.ctx()., 1_000_000);
        }
        renderer.lock().expect("Renderer Not Initialized").clear();

        if self.file_dialog_open {
        egui::Window::new("Load PLY File")
            .show(ui.ctx(), |ui| {
                ui.label("Enter PLY file path:");
                ui.text_edit_singleline(&mut self.cur_path); // Add proper path handling
                
                ui.horizontal(|ui| {
                    if ui.button("Load").clicked() {
                        let renderer = &mut renderer.lock().expect("Renderer Not Initialized");
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

        let start_time = Instant::now();

        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2 { x: max_rect.width(), y: max_rect.height() }, egui::Sense::drag());
    

        let input_state: Option<InputState> = ui.input(|input_state| 
            if response.hovered() { //&& response.has_focus() {
                Some(input_state.clone())
            }else{None}
        );

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

        // let painter = ui.painter();

        for &(x, y, z, color) in &self.points {
            renderer.lock().expect("Renderer Not Initialized").add_point(x, y, z, color);
        }

        let o = <std::option::Option<Camera> as Clone>::clone(&renderer.lock().expect("Renderer Not Initialized").camera).unwrap().orientation.clone();

        let cb = egui_glow::CallbackFn::new(move |_info, _painter| {
            renderer.lock().expect("Renderer Not Initialized").render(max_rect, input_state.clone());
        });

        let callback = egui::PaintCallback {
            rect: max_rect,
            callback: Arc::new(cb),
        };

        ui.painter().add(callback);


        let line_length:f32 = 20.;

        // if let Some(input_state) = input_state {
        //     if input_state.pointer.any_down() {

            let pos1 = o.inverse()*glam::Vec3::X;
            let pos2 = o.inverse()*glam::Vec3::Y;
            let pos3 = o.inverse()*glam::Vec3::Z;

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
        // }}

        let end_time = Instant::now();

        // println!("{}", end_time.duration_since(start_time).as_millis());

        let text_size = 12.;

        ui.painter().text(max_rect.min, Align2::LEFT_TOP, 
            format!("{} ms",end_time.duration_since(start_time).as_millis()), 
            FontId::monospace(text_size), Color32::WHITE);

        ui.painter().text(max_rect.min + egui::Vec2 {x:0.,y:text_size}, Align2::LEFT_TOP, 
            format!("{} points", self.points.len()), 
            FontId::monospace(text_size), Color32::WHITE);
    }
    fn context_menu(&mut self, ui: &mut Ui) {
        if ui.button("Load PLY").clicked() {
            self.file_dialog_open = true;
        }
    }
}