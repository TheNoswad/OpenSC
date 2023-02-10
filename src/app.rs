use std::{fs::File, path::PathBuf, sync::{Arc, Mutex}};

use eframe::{glow, egui_glow};
use egui::DragValue;
use xmltree::Element;

use crate::{world::World, furniture::FurnitureDesigns, rendering::{self, set_uniform, create_vertex_buffer}};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    //#[serde(skip)]
    element: Option<Element>,
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    //#[serde(skip)]
    value: f32,

    //#[serde(skip)]
    /// A world that is opened in the app.
    world: Option<World>,

    //#[serde(skip)]
    // todo make open files abstracted somehow
    furnituredsg: Option<FurnitureDesigns>,

    //#[serde(skip)]
    rendering: Arc<Mutex<FurnitureRender>>,

    angle: f32,
}

// impl Default for TemplateApp {
//     fn default() -> Self {
//         Self {
//             dropped_files: vec![],
//             picked_path: None,
//             element: None,
//             // Example stuff:
//             label: "Hello World!".to_owned(),
//             value: 2.7,
//             world: None,
//             furnituredsg: None,
//             rendering: todo!(),
//         }
//     }
// }

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc.gl.as_ref().expect("You need to enable the `glow` feature to use eframe with OpenGL");
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            rendering: Arc::new(Mutex::new(FurnitureRender::new(gl))),
            world: None,
            furnituredsg: None,
            dropped_files: vec![],
            picked_path: None,
            element: None,
            angle: 0.0,
            //..Default::default()
        }
    }
}

impl TemplateApp {
    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());

        self.angle += response.drag_delta().x * 0.01;

        // Clone locals so we can move them into the paint callback:
        let angle = self.angle;
        let rotating_triangle = self.rendering.clone();

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                rotating_triangle.lock().unwrap().paint(painter.gl(), angle);
            })),
        };
        ui.painter().add(callback);
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, value, dropped_files , ..} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        match self.world {
            Some(ref mut world) => {
                world.draw(ctx);
            }
            None => {

            }
        }
        

        // Top panel with a menu bar:
        // ***************************************************************************************
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                // File tab:
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            open_handler(path, &mut self.world, &mut self.furnituredsg);
                        }
                    }
                    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            // Process dropped files
            if !dropped_files.is_empty() {
                dbg!(&dropped_files[0].path);
                panic!()
            }

            
            

            ui.add(DragValue::new(value).speed(0.1).prefix("Value: "));

            ui.push_id("asd1", |ui| {
                ui.collapsing("heading", |ui| {
                    ui.label("This is a collapsible section");
                    ui.label("You can collapse it by clicking the arrow");
                    ui.label("You can also collapse it by clicking the header");
                });

            });

            ui.push_id("asd2", |ui| {
                ui.collapsing("heading", |ui| {
                    ui.label("This is a collapsible section");
                    ui.label("You can collapse it by clicking the arrow");
                    ui.label("You can also collapse it by clicking the header");
                });

            });

            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }



            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.custom_painting(ui);
            });
            ui.heading("eframe template");
            
        });

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     // The central panel the region left after adding TopPanel's and SidePanel's

        //     ui.heading("eframe template");
        //     ui.hyperlink("https://github.com/emilk/eframe_template");
        //     ui.add(egui::github_link_file!(
        //         "https://github.com/emilk/eframe_template/blob/master/",
        //         "Source code."
        //     ));
        //     egui::warn_if_debug_build(ui);
        // });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}


fn open_handler(path: PathBuf, world: &mut Option<World>, furniture: &mut Option<FurnitureDesigns>) {
    println!("Opening file: {:?}", path);

    // Match the file type and create a variable that has the file type
    let file_type = match path.file_name().unwrap().to_str().unwrap() {
        "Project.xml" => FileType::Project,
        "FurnitureDesigns.xml" => FileType::FurnitureDesigns,
        _ => FileType::Unknown,
    };



    // Match the file extension and create a variable the has the file extension
    let extension = match path.extension().unwrap().to_str().unwrap() {
        "xml" => FileExtension::XML,
        _ => FileExtension::Unknown,
    };

    match file_type {
        FileType::Project => {
            println!("Opening project file");
            if world.is_some() {
                panic!("World already exists. Fix this later.");
            } else {
                let mut newworld = World::new();
                newworld.load_project(&path).unwrap();
                *world = Some(newworld);
            }
        }
        FileType::FurnitureDesigns => {
            println!("Opening furniture designs file");
            *furniture = Some(FurnitureDesigns::new_from_file(&path));
        }
        FileType::Unknown => {
            println!("Unknown file type");
        }
    }

    // // Open XML file
    // if extension == FileExtension::XML {
    //     println!("Opening xml file");
    //     if world.is_some() {
    //         panic!("World already exists. Fix this later.");
    //     } else {
    //         let mut newworld = World::new();
    //         newworld.load_project(&path).unwrap();
    //         *world = Some(newworld);
    //     }
    // } else {
    //     println!("Unknown file type");
    // }
}

#[derive(PartialEq, Eq)]
enum FileExtension {
    XML,
    Unknown,
}

enum FileType {
    Project,
    FurnitureDesigns,
    Unknown,
}

struct FurnitureRender {
    program: glow::Program,
    vbo: glow::Buffer,
    vao: glow::VertexArray
}

impl FurnitureRender {
    fn new(gl: &glow::Context) -> Self {
        use glow::HasContext as _;

        let shader_version = if cfg!(target_arch = "wasm32") {
            "#version 300 es"
        } else {
            "#version 330"
        };

        unsafe {
            let vert = include_str!("vertex.vert");
            let frag = include_str!("fragment.frag");
            let program = rendering::create_program(gl, vert, frag);
            gl.use_program(Some(program));

            // Create a vertex buffer and vertex array object
            let (vbo, vao) = create_vertex_buffer(&gl);

            // Upload some uniforms
            set_uniform(&gl, program, "blue", 0.8);

            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            
            // let vertex_array = gl
            //     .create_vertex_array()
            //     .expect("Cannot create vertex array");

            Self {
                program,
                vbo,
                vao
            }
        }
    }

    fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
        }
    }

    fn paint(&self, gl: &glow::Context, angle: f32) {
        use glow::HasContext as _;
        unsafe {
            //gl.clear(glow::COLOR_BUFFER_BIT);
            gl.use_program(Some(self.program));
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 3);
            // gl.use_program(Some(self.program));
            // gl.uniform_1_f32(
            //     gl.get_uniform_location(self.program, "u_angle").as_ref(),
            //     angle,
            // );
            // gl.bind_vertex_array(Some(self.vertex_array));
            // gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }
}

