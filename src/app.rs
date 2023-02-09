use std::{fs::File, path::PathBuf};

use egui::DragValue;
use xmltree::Element;

use crate::world::World;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    #[serde(skip)]
    element: Option<Element>,
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    /// A world that is opened in the app.
    world: Option<World>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            dropped_files: vec![],
            picked_path: None,
            element: None,
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            world: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

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

        //let data = include_str!("../World/Project.xml");
        
    
    //     let mut names_element = Element::parse(File::open(&self.picked_path.as_ref().unwrap()).unwrap()).unwrap();
    
    //     for element in names_element.children {
    //         let nelement = element.as_element().unwrap();
    //         if nelement.name == "Entities" {
    //             break;
    //         }
    //         println!("{:#?}", nelement);
    // }

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                // File tab:
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            open_handler(path, &mut self.world);
                        }
                    }
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


fn open_handler(path: PathBuf, world: &mut Option<World>) {
    println!("Opening file: {:?}", path);

    // Match the file extension and create a variable the has the file extension
    let extension = match path.extension().unwrap().to_str().unwrap() {
        "xml" => FileType::XML,
        _ => FileType::Unknown,
    };

    // Open XML file
    if extension == FileType::XML {
        println!("Opening xml file");
        if world.is_some() {
            panic!("World already exists. Fix this later.");
        } else {
            let mut newworld = World::new();
            newworld.load_project(&path).unwrap();
            *world = Some(newworld);
        }
    } else {
        println!("Unknown file type");
    }
}

#[derive(PartialEq, Eq)]
enum FileType {
    XML,
    Unknown,
}