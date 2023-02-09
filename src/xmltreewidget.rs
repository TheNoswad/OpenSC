
use egui::{DragValue, TextEdit, Checkbox};
use egui_extras::{TableBuilder, Column};
use xmltree::Element;

/// Draw the XML tree widget.
pub fn xml_tree_widget(element: &mut Element, ui: &mut egui::Ui, id_idx: &mut usize) {
    ui.push_id(id_idx.clone(), |ui| {
        *id_idx += 1;
        ui.collapsing(&element.name, |ui| {
            // Add attributes
            for attribute in &element.attributes {
                ui.label(format!("@{}: {}", attribute.0, attribute.1));
            }
            // Add children
            for (i, child) in &mut element.children.iter_mut().enumerate() {
                let child = child.as_mut_element().unwrap();
                //ui.auto_id_with(&child.name + &i.to_string());
                //ui.add
                //ui.auto_id_with(child.name.clone() + &i.to_string());
                xml_tree_widget(child, ui, id_idx);
            }
        });
    });
}

/// Draw the Project XML widget.
pub fn project_xml_widget(element: &mut Element, ui: &mut egui::Ui, id_idx: &mut usize) {
    ui.push_id(id_idx.clone(), |ui| {
        *id_idx += 1;

        // Check if the element is a Values element.
        // If it is, use the Name attribute as the name of the Collapsing Header.
        let name;
        if element.name == "Values" {
            name = element.attributes.get("Name").unwrap();
        } else {
            name = &element.name;
        }

        // If the element is a Value element, list the attributes as labels.
        if element.name == "Value" {
            let valuetype = Types::from_str(element.attributes.get("Type").unwrap().as_str());

            value_widget(element, ui, valuetype);
            // match valuetype {
            //     Types::double => {
            //         let mut value = element.attributes.get_mut("Value").unwrap().parse::<f64>().unwrap();
            //         ui.horizontal(|ui| {
            //             ui.label(element.attributes.get("Name").unwrap());
            //             if ui.add(DragValue::new(&mut value)).changed() {
            //                 *element.attributes.get_mut("Value").unwrap() = value.to_string();
            //             }
            //         });
            //     },
            //     Types::long => {
            //         let mut value = element.attributes.get_mut("Value").unwrap().parse::<i64>().unwrap();
            //         ui.add(DragValue::new(&mut value));
            //     },
            //     Types::float => {
            //         let mut value = element.attributes.get_mut("Value").unwrap().parse::<f32>().unwrap();
            //         ui.add(DragValue::new(&mut value));
            //     },
            //     _ => {}
            //     // Types::string => {
            //     //     ui.label(format!("@{}: {}", attribute.0, attribute.1));
            //     // },
            //     // Types::Unknown => {
            //     //     ui.label(format!("@{}: {}", attribute.0, attribute.1));
            //     // }
            // // for attribute in element.attributes.iter_mut() {
            // //     }
            //     // match element.attributes.get("Type").unwrap().as_str() {
            //     //     "double" => {
            //     //         let hgjfhgfj = element.attributes.get_mut("Value").unwrap().parse::<f64>().unwrap();
            //     //         let mut value = &mut element.attributes.get("Value").unwrap().parse::<f64>().unwrap();
            //     //         ui.add(DragValue::new(value));
            //     //     }, 
            //     //     _ => {
            //     //         ui.label(format!("@{}: {}", attribute.0, attribute.1));
            //     //     }
            //     // }
            //     // ui.label(format!("@{}: {}", attribute.0, attribute.1));
            // }
        // Not a Values element, so list the children as Collapsing Headers.
        // An element named Value should never have children.
        } else {
            ui.collapsing(name, |ui| {
                // Add attributes
                for attribute in &element.attributes {
                    ui.label(format!("@{}: {}", attribute.0, attribute.1));
                }
    
                // Add children
                for (i, child) in &mut element.children.iter_mut().enumerate() {
                    let child = child.as_mut_element().unwrap();
                    project_xml_widget(child, ui, id_idx);
                }
            });
        }
    });
}

enum Types {
    double,
    long,
    float,
    string,
    int,
    bool,
    Vector2,
    GameStartingPositionMode,
    GameTerrainGenerationMode,
    GameTimeOfDayMode,
    Unknown
}

impl Types {
    fn from_str(s: &str) -> Types {
        match s {
            "double" => Types::double,
            "long" => Types::long,
            "float" => Types::float,
            "string" => Types::string,
            "int" => Types::int,
            "bool" => Types::bool,
            "Vector2" => Types::Vector2,
            "GameStartingPositionMode" => Types::GameStartingPositionMode,
            "GameTerrainGenerationMode" => Types::GameTerrainGenerationMode,
            "GameTimeOfDayMode" => Types::GameTimeOfDayMode,
            _ => Types::Unknown
        }
    }
}

fn value_widget(element: &mut Element, ui: &mut egui::Ui, valuetype: Types) {
    match valuetype {
        Types::double => {
            let mut value = element.attributes.get("Value").unwrap().parse::<f64>().unwrap();
            ui.horizontal(|ui| {
                ui.label(element.attributes.get("Name").unwrap());
                if ui.add(DragValue::new(&mut value)).changed() {
                    *element.attributes.get_mut("Value").unwrap() = value.to_string();
                }
            });
        },
        Types::long => {
            let mut value = element.attributes.get("Value").unwrap().parse::<i64>().unwrap();
            ui.horizontal(|ui| {
                ui.label(element.attributes.get("Name").unwrap());
                if ui.add(DragValue::new(&mut value)).changed() {
                    *element.attributes.get_mut("Value").unwrap() = value.to_string();
                }
            });
        },
        Types::float => {
            let mut value = element.attributes.get("Value").unwrap().parse::<f32>().unwrap();
            ui.horizontal(|ui| {
                ui.label(element.attributes.get("Name").unwrap());
                if ui.add(DragValue::new(&mut value)).changed() {
                    *element.attributes.get_mut("Value").unwrap() = value.to_string();
                }
            });
            // ui.add(DragValue::new(&mut value));
        },
        Types::int => {
            let mut value = element.attributes.get("Value").unwrap().parse::<i32>().unwrap();
            ui.horizontal(|ui| {
                ui.label(element.attributes.get("Name").unwrap());
                if ui.add(DragValue::new(&mut value)).changed() {
                    *element.attributes.get_mut("Value").unwrap() = value.to_string();
                }
            });
        },
        Types::bool => {
            // TODO - This is a hack to get around the fact that the XML file has "True" and "False" instead of "true" and "false".
            // Rust's bool::parse() function only accepts "true" and "false".
            let mut value = element.attributes.get("Value").unwrap().to_lowercase().parse::<bool>().unwrap();
            ui.horizontal(|ui| {
                ui.label(element.attributes.get("Name").unwrap());
                if ui.add(Checkbox::new(&mut value, "")).changed() {
                    *element.attributes.get_mut("Value").unwrap() = value.to_string();
                }
            });
        },
        Types::string => {
            let mut value = element.attributes.get("Value").unwrap().to_string();
            ui.horizontal(|ui| {
                ui.label(element.attributes.get("Name").unwrap());
                if ui.add(TextEdit::singleline(&mut value)).changed() {
                    *element.attributes.get_mut("Value").unwrap() = value.to_string();
                }
            });
        },
        // Todo - Add a Vector2 widget.
        Types::Vector2 => {
            vector2_widget(element, ui);
        },

        _ => {
            println!("Unknown type {}" , element.attributes.get("Type").unwrap());
            ui.label("Unknown type!");
            // We don't know what type this is, so just display it as a string.
            let mut value = element.attributes.get("Value").unwrap().to_string();
            ui.horizontal(|ui| {
                ui.label(element.attributes.get("Name").unwrap());
                if ui.add(TextEdit::singleline(&mut value)).changed() {
                    *element.attributes.get_mut("Value").unwrap() = value.to_string();
                }
            });
        }
    }
}

fn vector2_widget(element: &mut Element, ui: &mut egui::Ui) {
    // TODO
    // Some wacky garbage to clean up later
    let binding = element.attributes.get("Value").unwrap().to_string();
    let mut value = binding.split(",").collect::<Vec<&str>>();
    // let binding = element.attributes.get("Value").unwrap().to_string();
    // let mut value = binding.split(",").collect::<Vec<&str>>();

    let mut vector2 = vec![value[0].parse::<f32>().unwrap(), value[1].parse::<f32>().unwrap()];
    ui.horizontal(|ui| {
        ui.label(element.attributes.get("Name").unwrap());
        ui.label("X:");
        if ui.add(DragValue::new(&mut vector2[0])).changed() {
            let fvalue = vector2.iter().map(|x| x.to_string() + ",").collect::<String>();
            *element.attributes.get_mut("Value").unwrap() = fvalue//value.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
        }
        ui.label("Y:");
        if ui.add(TextEdit::singleline(&mut value[1])).changed() {
            *element.attributes.get_mut("Value").unwrap() = value.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
        }
    });
}