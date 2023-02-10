use std::{path::PathBuf, io::Read, fs::File};

use unicode_bom::Bom;
use xmltree::Element;

pub struct FurnitureDesigns {
    pub values: Vec<FurnitureDesign>,
    xml: Option<Element>
}

impl FurnitureDesigns {
    /// Creates a new empty list of furniture designs.
    pub fn new() -> Self {
        Self {
            values: vec![],
            xml: None
        }
    }

    /// Creates a new FurnitureDesigns from a file.
    pub fn new_from_file(path: &PathBuf) -> Self {
        let mut designs = Self::new();
        designs.load(path);
        designs
    }

    /// Loads FurnitureDesigns from a file.
    fn load(&mut self, path: &PathBuf) {
        // Read the file to a string and parse it as XML.
        let mut file = std::fs::File::open(path).map_err(|e| e.to_string()).unwrap();

        // Check for UTF-8 BOM and skip it if it exists.
        // This is necessary because the XML parser doesn't handle it.
        let bom = getbom(path);
        
        if bom != Bom::Null {
            let mut x = [0; 3];
            let _y = file.read_exact(&mut x);
        };

        let reader = std::io::BufReader::new(file);
        let element: Element = xmltree::Element::parse(reader).map_err(|e| e.to_string()).unwrap();
        self.xml = Some(element);

        for child in &self.xml.as_ref().unwrap().children {
            //dbg!(child);
            let index = child.as_element().unwrap().attributes.get("Name").unwrap().parse::<usize>().unwrap();
            dbg!(&child.as_element().unwrap().name);

            self.values.push(FurnitureDesign::parse(&child.as_element().unwrap()));

        }
    }


    pub fn parse(&mut self, element: Element) {
        for child in element.children {
            dbg!(child);
        }
    }

    // pub fn add(&mut self, design: FurnitureDesign) {
    //     self.0.push(design);
    // }

    // pub fn get(&self, index: usize) -> Option<&FurnitureDesign> {
    //     self.0.get(index)
    // }

    // pub fn get_mut(&mut self, index: usize) -> Option<&mut FurnitureDesign> {
    //     self.0.get_mut(index)
    // }

    // pub fn len(&self) -> usize {
    //     self.0.len()
    // }
}

#[derive(Debug)]
pub struct FurnitureDesign {
    name: String,
    terrainusecount: u32,
    resolution: u32,
    interactionmode: FurnitureInteractionMode,
    linkeddesign: Option<u32>,
    values: Vec<RleEntry>
}

impl FurnitureDesign {
    fn parse(element: &Element) -> Self {
        let mut name = "";
        let mut terrainusecount = 0;
        let mut resolution = 0;
        let mut interactionmode = FurnitureInteractionMode::Unknown;
        let mut linkeddesign: Option<u32> = None;
        let mut values: Option<Vec<RleEntry>> = None; // Parse as Vec<u32> later.

        //dbg!(&element);
        element.children.iter().for_each(|child| {
            let item = child.as_element().unwrap();
            match item.attributes.get("Name").unwrap().as_str() {
                "Name" => name = item.attributes.get("Value").unwrap(),
                "TerrainUseCount" => terrainusecount = item.attributes.get("Value").unwrap().parse::<u32>().unwrap(),
                "Resolution" => resolution = item.attributes.get("Value").unwrap().parse::<u32>().unwrap(),
                "InteractionMode" => interactionmode = match item.attributes.get("Value").unwrap().as_str() {
                    "None" => FurnitureInteractionMode::None,
                    "MultiState" => FurnitureInteractionMode::MultiState,
                    _ => FurnitureInteractionMode::Unknown
                },
                "LinkedDesign" => linkeddesign = Some(item.attributes.get("Value").unwrap().parse::<u32>().unwrap()),
                "Values" => values = Some(FurnitureDesign::parse_values(item.attributes.get("Value").unwrap())),
                _ => panic!("Unknown attribute: {}", item.attributes.get("Name").unwrap())
            }
        });

        dbg!(&values);

        FurnitureDesign {
            name: name.to_string(),
            terrainusecount,
            resolution,
            interactionmode,
            linkeddesign,
            values: values.unwrap()
        }
    }

    fn parse_values(values: &str) -> Vec<RleEntry> {
        let mut newvalues = vec![];
        for value in values.split(',').filter(|&x| !x.is_empty()) {
            newvalues.push(RleEntry::deserealize(value));
        }

        newvalues
    }
}

#[derive(Debug)]
pub enum FurnitureInteractionMode {
    None,
    MultiState,
    Unknown
}

pub struct FurnitureWidget {

}

fn getbom(path: &PathBuf) -> Bom {
    let mut file = File::open(path).unwrap();
    Bom::from(&mut file)
}

#[derive(Debug)]
pub struct RleEntry {
    length: u32,
    value: u32
}

impl RleEntry {
    fn deserealize(str: &str) -> RleEntry {
        dbg!(&str);
        let entry: Vec<u32> = str.split("*")
            .filter(|&x| !x.is_empty()) // Remove empty strings. For some reason there is whitespace at the end of the string.
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

            dbg!(&entry);
        
        RleEntry {
            length: *entry.get(0).unwrap(),
            value: *entry.get(1).unwrap(),
        }
    }
}