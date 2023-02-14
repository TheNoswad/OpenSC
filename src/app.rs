use std::{fs::File, path::PathBuf, sync::{Arc, Mutex}, num::NonZeroU64};

use eframe::{glow, egui_glow};
use egui::DragValue;

use xmltree::Element;

use crate::{world::World, furniture::FurnitureDesigns};

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
    //rendering: Arc<Mutex<FurnitureRender>>,

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
        //let gl = cc.gl.as_ref().expect("You need to enable the `glow` feature to use eframe with OpenGL");
        


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
            //rendering: Arc::new(Mutex::new(FurnitureRender::new(gl))),
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
    // fn custom_painting(&mut self, ui: &mut egui::Ui) {
    //     let (rect, response) =
    //         ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());

    //     self.angle += response.drag_delta().x * 0.01;

    //     // Clone locals so we can move them into the paint callback:
    //     let angle = self.angle;

    //     // The callback function for WGPU is in two stages: prepare, and paint.
    //     //
    //     // The prepare callback is called every frame before paint and is given access to the wgpu
    //     // Device and Queue, which can be used, for instance, to update buffers and uniforms before
    //     // rendering.
    //     //
    //     // You can use the main `CommandEncoder` that is passed-in, return an arbitrary number
    //     // of user-defined `CommandBuffer`s, or both.
    //     // The main command buffer, as well as all user-defined ones, will be submitted together
    //     // to the GPU in a single call.
    //     //
    //     // The paint callback is called after prepare and is given access to the render pass, which
    //     // can be used to issue draw commands.
    //     let cb = egui_wgpu::CallbackFn::new()
    //         .prepare(move |device, queue, _encoder, paint_callback_resources| {
    //             let resources: &FurnitureRenderResources = paint_callback_resources.get().unwrap();
    //             resources.prepare(device, queue, angle);
    //             Vec::new()
    //         })
    //         .paint(move |_info, render_pass, paint_callback_resources| {
    //             let resources: &FurnitureRenderResources = paint_callback_resources.get().unwrap();
    //             resources.paint(render_pass);
    //         });

    //     let callback = egui::PaintCallback {
    //         rect,
    //         callback: Arc::new(cb),
    //     };

    //     ui.painter().add(callback);
    // }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, value, dropped_files, picked_path, element, world, angle, .. } = self;

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

            if ui.button("Update Mesh").clicked() {
                println!("Updating mesh start"); // WORKS HERRE WTF
                let mesh = self.furnituredsg.as_ref().unwrap().values[1].get_mesh();
                let (rect, response) =
                        ui.allocate_exact_size(egui::Vec2::splat(1.0), egui::Sense::focusable_noninteractive());
                let callback = egui::PaintCallback {
                    rect,
                    callback: std::sync::Arc::new(egui_glow::CallbackFn::new(
                        move |info, painter| {
                            with_three_d(painter.gl(), |three_d| {
                                println!("Updating meshcb");
                                three_d.update_mesh(&mesh)
                            });
                        },
                    )),
                };
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
            
            egui::ScrollArea::both().show(ui, |ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    let (rect, response) =
                        ui.allocate_exact_size(egui::Vec2::splat(512.0), egui::Sense::drag());
    
                    self.angle += response.drag_delta().x * 0.01;
    
                    // Clone locals so we can move them into the paint callback:
                    let angle = self.angle;
    
                    let callback = egui::PaintCallback {
                        rect,
                        callback: std::sync::Arc::new(egui_glow::CallbackFn::new(
                            move |info, painter| {
                                with_three_d(painter.gl(), |three_d| {
                                    three_d.frame(
                                        FrameInput::new(&three_d.context, &info, painter),
                                        angle,
                                    );
                                });
                            },
                        )),
                    };
                    ui.painter().add(callback);
                });
                ui.label("Drag to rotate!");
            });
            
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

    // fn on_exit(&mut self, gl: Option<&glow::Context>) {
    //     if let Some(gl) = gl {
    //         self.rendering.lock().unwrap().destroy(gl);
    //     }
    // }
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

// struct FurnitureRender {
//     // Shader program
//     program: glow::Program,
//     // Mesh to render
//     mesh: rendering::Mesh,
//     // Vertex buffer
//     vbo: glow::Buffer,
//     // Vertex array. Describes the layout of the vertex buffer
//     vao: glow::VertexArray,
//     // Index buffer
//     ibo: glow::Buffer,

//     // If the mesh needs to be updated
//     update: bool,
// }

// impl FurnitureRender {
//     fn new(gl: &glow::Context) -> Self {
//         use glow::HasContext as _;

//         let shader_version = if cfg!(target_arch = "wasm32") {
//             "#version 300 es"
//         } else {
//             "#version 330"
//         };

//         unsafe {
//             let vert = include_str!("vertex.vert");
//             let frag = include_str!("fragment.frag");
//             let program = rendering::create_program(gl, vert, frag);
//             gl.use_program(Some(program));

//             // Create a vertex buffer and vertex array object
//             let (vbo, vao) = create_vertex_buffer(&gl);

//             let ibo = create_index_buffer(&gl);

//             // Upload some uniforms
//             set_uniform(&gl, program, "blue", 0.8);

//             gl.clear_color(0.1, 0.2, 0.3, 1.0);

            
//             // let vertex_array = gl
//             //     .create_vertex_array()
//             //     .expect("Cannot create vertex array");

//             Self {
//                 program,
//                 mesh: rendering::Mesh::empty(),
//                 vbo,
//                 vao,
//                 ibo,
//                 update: false
//             }
//         }
//     }

//     fn destroy(&self, gl: &glow::Context) {
//         use glow::HasContext as _;
//         unsafe {
//             gl.delete_program(self.program);
//             gl.delete_vertex_array(self.vao);
//             gl.delete_buffer(self.vbo);
//         }
//     }

//     fn paint(&mut self, gl: &glow::Context, angle: f32) {
//         use glow::HasContext as _;

//         if self.update {
//             self.update = false;
//             self.update_mesh(gl);
//         }

//         unsafe {
//             //gl.clear(glow::COLOR_BUFFER_BIT);

//             // Use the program
//             gl.use_program(Some(self.program));

//             // Bind the vertex array
//             gl.bind_vertex_array(Some(self.vao));

//             // Bind the index buffer
//             gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ibo));

//             // Draw the mesh
//             gl.draw_elements(glow::TRIANGLES, 3, glow::UNSIGNED_INT, 0);


//             //gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 3);


//             // gl.use_program(Some(self.program));
//             // gl.uniform_1_f32(
//             //     gl.get_uniform_location(self.program, "u_angle").as_ref(),
//             //     angle,
//             // );
//             // gl.bind_vertex_array(Some(self.vertex_array));
//             // gl.draw_arrays(glow::TRIANGLES, 0, 3);
//         }
//     }

//     fn update_mesh(&mut self, gl: &glow::Context) {
//         println!("Updating mesh");
//         use glow::HasContext as _;
        
//         //dbg!(&self.mesh);
//         unsafe {
//             // Bind the vertex array
//             gl.bind_vertex_array(Some(self.vao));

//             // Bind the vertex buffer
//             gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));

//             // Upload the vertex data
//             gl.buffer_data_u8_slice(
//                 glow::ARRAY_BUFFER,
//                 &self.mesh.get_vertices_slice(),
//                 glow::STATIC_DRAW,
//             );

//             // Bind the index buffer
//             gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ibo));

//             // Upload the index data
//             gl.buffer_data_u8_slice(
//                 glow::ELEMENT_ARRAY_BUFFER,
//                 &self.mesh.get_indices_slice(),
//                 glow::STATIC_DRAW,
//             );
//         }
//     }
// }


/// We get a [`glow::Context`] from `eframe` and we want to construct a [`ThreeDApp`].
///
/// Sadly we can't just create a [`ThreeDApp`] in [`MyApp::new`] and pass it
/// to the [`egui::PaintCallback`] because [`glow::Context`] isn't `Send+Sync` on web, which
/// [`egui::PaintCallback`] needs. If you do not target web, then you can construct the [`ThreeDApp`] in [`MyApp::new`].
fn with_three_d<R>(gl: &std::sync::Arc<glow::Context>, f: impl FnOnce(&mut FurnitureView) -> R) -> R {
    use std::cell::RefCell;
    thread_local! {
        pub static THREE_D: RefCell<Option<FurnitureView>> = RefCell::new(None);
    }

    THREE_D.with(|three_d| {
        let mut three_d = three_d.borrow_mut();
        let three_d = three_d.get_or_insert_with(|| FurnitureView::new(gl.clone()));
        f(three_d)
    })
}

///
/// Translates from egui input to three-d input
///
pub struct FrameInput<'a> {
    screen: three_d::RenderTarget<'a>,
    viewport: three_d::Viewport,
    scissor_box: three_d::ScissorBox,
}

impl FrameInput<'_> {
    pub fn new(
        context: &three_d::Context,
        info: &egui::PaintCallbackInfo,
        painter: &egui_glow::Painter,
    ) -> Self {
        use three_d::*;

        // Disable sRGB textures for three-d
        #[cfg(not(target_arch = "wasm32"))]
        #[allow(unsafe_code)]
        unsafe {
            use glow::HasContext as _;
            context.disable(glow::FRAMEBUFFER_SRGB);
        }

        // Constructs a screen render target to render the final image to
        let screen = painter.intermediate_fbo().map_or_else(
            || {
                RenderTarget::screen(
                    context,
                    info.viewport.width() as u32,
                    info.viewport.height() as u32,
                )
            },
            |fbo| {
                RenderTarget::from_framebuffer(
                    context,
                    info.viewport.width() as u32,
                    info.viewport.height() as u32,
                    fbo,
                )
            },
        );

        // Set where to paint
        let viewport = info.viewport_in_pixels();
        let viewport = Viewport {
            x: viewport.left_px.round() as _,
            y: viewport.from_bottom_px.round() as _,
            width: viewport.width_px.round() as _,
            height: viewport.height_px.round() as _,
        };

        // Respect the egui clip region (e.g. if we are inside an `egui::ScrollArea`).
        let clip_rect = info.clip_rect_in_pixels();
        let scissor_box = ScissorBox {
            x: clip_rect.left_px.round() as _,
            y: clip_rect.from_bottom_px.round() as _,
            width: clip_rect.width_px.round() as _,
            height: clip_rect.height_px.round() as _,
        };
        Self {
            screen,
            scissor_box,
            viewport,
        }
    }
}

///
/// Based on the `three-d` [Triangle example](https://github.com/asny/three-d/blob/master/examples/triangle/src/main.rs).
/// This is where you'll need to customize
///
use three_d::*;
pub struct FurnitureView {
    context: Context,
    camera: Camera,
    model: Gm<Mesh, ColorMaterial>,
}

impl FurnitureView {
    pub fn new(gl: std::sync::Arc<glow::Context>) -> Self {
        let context = Context::from_gl_context(gl).unwrap();
        // Create a camera
        let camera = Camera::new_perspective(
            Viewport::new_at_origo(1, 1),
            vec3(0.0, 0.0, 2.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            0.1,
            10.0,
        );

        // Create a CPU-side mesh consisting of a single colored triangle
        let positions = vec![
            vec3(0.5, -0.5, 0.0),  // bottom right
            vec3(-0.5, -0.5, 0.0), // bottom left
            vec3(0.0, 0.5, 0.0),   // top
        ];
        let colors = vec![
            Color::new(255, 0, 0, 255), // bottom right
            Color::new(0, 255, 0, 255), // bottom left
            Color::new(0, 0, 255, 255), // top
        ];
        let cpu_mesh = CpuMesh {
            positions: Positions::F32(positions),
            colors: Some(colors),
            ..Default::default()
        };

        // Construct a model, with a default color material, thereby transferring the mesh data to the GPU
        let model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());
        Self {
            context,
            camera,
            model,
        }
    }

    pub fn frame(&mut self, frame_input: FrameInput<'_>, angle: f32) -> Option<glow::Framebuffer> {
        // Ensure the viewport matches the current window viewport which changes if the window is resized
        self.camera.set_viewport(frame_input.viewport);

        // Set the current transformation of the triangle
        self.model
            .set_transformation(Mat4::from_angle_y(radians(angle)));

        // Get the screen render target to be able to render something on the screen
        frame_input
            .screen
            // Clear the color and depth of the screen render target
            .clear_partially(frame_input.scissor_box, ClearState::depth(1.0))
            // Render the triangle with the color material which uses the per vertex colors defined at construction
            .render_partially(frame_input.scissor_box, &self.camera, &[&self.model], &[]);

        frame_input.screen.into_framebuffer() // Take back the screen fbo, we will continue to use it.
    }

    pub fn update_mesh(&mut self, mesh: &CpuMesh) {
        self.model = Gm::new(Mesh::new(&self.context, mesh), ColorMaterial::default());
        println!("Updated mesh");
    }
}