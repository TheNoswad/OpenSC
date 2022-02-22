use serde::Deserialize;
use serde_tuple::Deserialize_tuple;

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
pub struct BlocksDataCSV {
    #[serde(rename = "Class Name")]
    pub ClassName: Option<String>,
    pub DefaultDisplayName: Option<String>,
    pub DefaultCategory: Option<String>,
    pub Behaviors: Option<String>,
    pub DisplayOrder: Option<String>,
    pub DefaultIconBlockOffset: Option<String>,
    pub DefaultIconViewOffset: Option<String>,
    pub DefaultIconViewScale: Option<String>,
    pub FirstPersonScale: Option<String>,
    pub FirstPersonOffset: Option<String>,
    pub FirstPersonRotation: Option<String>,
    pub InHandScale: Option<String>,
    pub InHandOffset: Option<String>,
    pub InHandRotation: Option<String>,
    pub CraftingId: Option<String>,
    pub DefaultCreativeData: Option<String>,
    pub IsCollidable: Option<String>,
    pub IsPlaceable: Option<String>,
    pub IsDiggingTransparent: Option<String>,
    pub IsPlacementTransparent: Option<String>,
    pub DefaultIsInteractive: Option<String>,
    pub IsEditable: Option<String>,
    pub IsNonDuplicable: Option<String>,
    pub IsGatherable: Option<String>,
    pub HasCollisionBehavior: Option<String>,
    pub KillsWhenStuck: Option<String>,
    pub IsFluidBlocker: Option<String>,
    pub IsTransparent: Option<String>,
    pub DefaultShadowStrength: Option<String>,
    pub LightAttenuation: Option<String>,
    pub DefaultEmittedLightAmount: Option<String>,
    pub ObjectShadowStrength: Option<String>,
    pub DefaultDropContent: Option<String>,
    pub DefaultDropCount: Option<String>,
    pub DefaultExperienceCount: Option<String>,
    pub RequiredToolLevel: Option<String>,
    pub MaxStacking: Option<String>,
    pub SleepSuitability: Option<String>,
    pub FrictionFactor: Option<String>,
    pub Density: Option<String>,
    pub NoAutoJump: Option<String>,
    pub NoSmoothRise: Option<String>,
    pub FuelHeatLevel: Option<String>,
    pub FuelFireDuration: Option<String>,
    pub DefaultSoundMaterialName: Option<String>,
    pub ShovelPower: Option<String>,
    pub QuarryPower: Option<String>,
    pub HackPower: Option<String>,
    pub DefaultMeleePower: Option<String>,
    pub DefaultMeleeHitProbability: Option<String>,
    pub DefaultProjectilePower: Option<String>,
    pub ToolLevel: Option<String>,
    pub PlayerLevelRequired: Option<String>,
    pub Durability: Option<String>,
    pub IsAimable: Option<String>,
    pub IsStickable: Option<String>,
    pub AlignToVelocity: Option<String>,
    pub ProjectileSpeed: Option<String>,
    pub ProjectileDamping: Option<String>,
    pub ProjectileTipOffset: Option<String>,
    pub DisintegratesOnHit: Option<String>,
    pub ProjectileStickProbability: Option<String>,
    pub DefaultHeat: Option<String>,
    pub FireDuration: Option<String>,
    pub ExplosionResilience: Option<String>,
    pub DefaultExplosionPressure: Option<String>,
    pub DefaultExplosionIncendiary: Option<String>,
    pub IsExplosionTransparent: Option<String>,
    pub DigMethod: Option<String>,
    pub DigResilience: Option<String>,
    pub ProjectileResilience: Option<String>,
    pub DefaultNutritionalValue: Option<String>,
    pub DefaultSicknessProbability: Option<String>,
    pub FoodType: Option<String>,
    pub DefaultRotPeriod: Option<String>,
    pub DefaultTextureSlot: Option<u8>,
    pub DestructionDebrisScale: Option<String>,
    pub DefaultDescription: Option<String>,
}

/// Parse the blocksdata into a vec
pub fn load_blockdata_csv() -> Vec<BlocksDataCSV> {
    let mut blocksdata: Vec<BlocksDataCSV> = vec![];
    let mut rdr = csv::ReaderBuilder::new()
        //.flexible(true)
        .delimiter(b';')
        .from_path("BlocksData.txt")
        .unwrap();
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        match result {
            Ok(unwrappedresult) => {
                let record: BlocksDataCSV = unwrappedresult;

                // This is just to ckeck if the entry is empty.
                match record.ClassName {
                    Some(_) => {
                        blocksdata.push(record);
                    },
                    None => {
                        println!("Empty csv class name")
                    },
                }
                
            },
            Err(err) => {
                println!("error {}", err)
            },
        }
    }
    dbg!(&blocksdata[0]);
    dbg!(&blocksdata[1]);
    dbg!(&blocksdata[2]);

    return blocksdata
}

#[derive(Debug, Deserialize)]
pub struct BlocksData {
    #[serde(rename = "Block")]
    pub Blocks: Vec<Block>
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub BlockId: u8,
    pub Name: String,
    pub IsFluidBlocker: bool,
    pub LightAttenuation: u8,
    pub EmittedLightAmount: u8,
    pub MaxStacking: u8,
    pub QuarryPower: f32,
    pub ShovelPower: f32,
    pub HackPower: f32,
    pub DigResilience: f32,
    pub IsAimable: bool,
    pub WeaponPower: f32,
    pub NutritionalValue: f32,
    #[serde(deserialize_with = "tags_deserialize")]
    pub TextureLocation: Vec<f32>,
    pub AverageToolLongevity: i32
}

#[derive(Debug, Deserialize)]
pub struct TextureLocation {
    x: u8,
    y: u8
}

pub fn load_blocksdata_xml() -> BlocksData {
    let blocksdataxml = include_str!("BlocksData.xml");
    let doc: BlocksData = quick_xml::de::from_str(blocksdataxml).unwrap();
    return doc
}

fn tags_deserialize<'de, D>(deserializer: D) -> Result<Vec<f32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let str_sequence = String::deserialize(deserializer)?;

    let ret: Vec<f32> = str_sequence.split(",").map(|s| s.trim().parse::<f32>().unwrap()).collect::<Vec<_>>();
    dbg!(&ret);

    // let ret: (&str, &str) = 
    //     str_sequence.split_once(',').unwrap();

    //let thing: Vec<u32> = str_sequence.split(',').chars().filter_map(|a| a.to_digit(10)).collect();
    // wtf have i created
    //let thing: Vec<u32> = str_sequence.chars().filter_map(|a| a.to_digit(10)).collect();

    // let numbers: Vec<i32> = 
    // str_sequence
    //       .lines().next().unwrap()
    //       .trim().split(',')
    //       .map(|s| s.strip_prefix(" ").unwrap().parse().unwrap())
    //       .collect();

    // dbg!(&numbers);
    // let mut vec: Vec<u8> = vec![];
    // for i in thing {
    //     vec.push(i)
    //     //vec.push(i.parse::<u8>().unwrap())
    // }
    Ok(
        ret
    )
    // Ok(str_sequence
    //     .split(',')
    //     .map(|item| item.to_owned())
    //     .collect())
}