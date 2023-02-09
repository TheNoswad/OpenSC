use std::path::PathBuf;

use egui::Context;

use crate::xmltreewidget::{xml_tree_widget, project_xml_widget};

/// A Survivalcraft world
pub struct World {
    /// The project that is currently loaded.
    pub project: Option<Project>

}

impl World {
    /// Creates a new world.
    pub fn new() -> Self {
        Self {
            project: None,
        }
    }

    /// Loads a project from a file.
    pub fn load_project(&mut self, path: &PathBuf) -> Result<(), String> {
        let project = Project::load(path)?;
        self.project = Some(project);
        Ok(())
    }

    pub fn draw(&mut self, ctx: &Context) {
        if let Some(project) = &mut self.project {
            egui::SidePanel::left("project").show(ctx, |ui| {
                project.draw(ui);
            });
        }
    }
}


pub struct Project {
    pub xml: xmltree::Element,
//     subsytems: Vec<Subsystem>,
//     entities: Vec<Entity>,
}

impl Project {
    /// Loads a project from a file.
    pub fn load(path: &PathBuf) -> Result<Self, String> {
        let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        let reader = std::io::BufReader::new(file);
        let element = xmltree::Element::parse(reader).map_err(|e| e.to_string())?;
        Ok(Self {
            xml: element,
        })
    }

    /// Draw the project editor.
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        ui.label("Project");
        ui.separator();
        ui.label("XML");
        ui.separator();
        let mut id_index: usize = 0;
        project_xml_widget(&mut self.xml, ui, &mut id_index)
        //ui.code(format!("{:#?}", self.xml));
    }
}

struct Subsystem {

}