use serde::Deserialize;

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
pub struct BlocksData {
    //#[serde(rename = "Class Name")]
    ClassName: Option<String>,
    DefaultDisplayName: Option<String>,
    DefaultCategory: Option<String>,
    Behaviors: Option<String>,
    DisplayOrder: Option<String>,
    DefaultIconBlockOffset: Option<String>,
    DefaultIconViewOffset: Option<String>,
    DefaultIconViewScale: Option<String>,
    FirstPersonScale: Option<String>,
    FirstPersonOffset: Option<String>,
    FirstPersonRotation: Option<String>,
    InHandScale: Option<String>,
    InHandOffset: Option<String>,
    InHandRotation: Option<String>,
    CraftingId: Option<String>,
    DefaultCreativeData: Option<String>,
    IsCollidable: Option<String>,
    IsPlaceable: Option<String>,
    IsDiggingTransparent: Option<String>,
    IsPlacementTransparent: Option<String>,
    DefaultIsInteractive: Option<String>,
    IsEditable: Option<String>,
    IsNonDuplicable: Option<String>,
    IsGatherable: Option<String>,
    HasCollisionBehavior: Option<String>,
    KillsWhenStuck: Option<String>,
    IsFluidBlocker: Option<String>,
    IsTransparent: Option<String>,
    DefaultShadowStrength: Option<String>,
    LightAttenuation: Option<String>,
    DefaultEmittedLightAmount: Option<String>,
    ObjectShadowStrength: Option<String>,
    DefaultDropContent: Option<String>,
    DefaultDropCount: Option<String>,
    DefaultExperienceCount: Option<String>,
    RequiredToolLevel: Option<String>,
    MaxStacking: Option<String>,
    SleepSuitability: Option<String>,
    FrictionFactor: Option<String>,
    Density: Option<String>,
    NoAutoJump: Option<String>,
    NoSmoothRise: Option<String>,
    FuelHeatLevel: Option<String>,
    FuelFireDuration: Option<String>,
    DefaultSoundMaterialName: Option<String>,
    ShovelPower: Option<String>,
    QuarryPower: Option<String>,
    HackPower: Option<String>,
    DefaultMeleePower: Option<String>,
    DefaultMeleeHitProbability: Option<String>,
    DefaultProjectilePower: Option<String>,
    ToolLevel: Option<String>,
    PlayerLevelRequired: Option<String>,
    Durability: Option<String>,
    IsAimable: Option<String>,
    IsStickable: Option<String>,
    AlignToVelocity: Option<String>,
    ProjectileSpeed: Option<String>,
    ProjectileDamping: Option<String>,
    ProjectileTipOffset: Option<String>,
    DisintegratesOnHit: Option<String>,
    ProjectileStickProbability: Option<String>,
    DefaultHeat: Option<String>,
    FireDuration: Option<String>,
    ExplosionResilience: Option<String>,
    DefaultExplosionPressure: Option<String>,
    DefaultExplosionIncendiary: Option<String>,
    IsExplosionTransparent: Option<String>,
    DigMethod: Option<String>,
    DigResilience: Option<String>,
    ProjectileResilience: Option<String>,
    DefaultNutritionalValue: Option<String>,
    DefaultSicknessProbability: Option<String>,
    FoodType: Option<String>,
    DefaultRotPeriod: Option<String>,
    DefaultTextureSlot: Option<String>,
    DestructionDebrisScale: Option<String>,
    DefaultDescription: Option<String>,
}

/// Parse the blocksdata into a vec
pub fn load_blockdata() -> Vec<BlocksData> {
    let mut blocksdata: Vec<BlocksData> = vec![];
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
                let record: BlocksData = unwrappedresult;
                blocksdata.push(record);
            },
            Err(err) => {
                println!("error {}", err)
            },
        }
    }
    return blocksdata
}