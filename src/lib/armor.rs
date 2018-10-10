
#[derive(Serialize, Deserialize, Debug)]
pub struct Armor {
    #[serde(rename="Hitpoints")]
    pub hitpoints: u32,
    #[serde(rename="Resistances")]
    pub resistances: [f32; 4],
    #[serde(rename="Repair")]
    pub repair: u32,
    #[serde(rename="Interval")]
    pub interval: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    #[serde(rename="Armor")]
    payload: Armor,
}

#[test]
fn deserialize() {
    use serde_json;
    let json = r#"{
        "Armor" : {
            "Hitpoints" : 1400,
            "Resistances" : [66.0, 23.5, 36.2, 44.8],
            "Repair" : 0,
            "Interval" : 1.0
        }
    }"#;

    let result: Result<Wrapper, _> = serde_json::from_str(&json);
    assert!(result.is_ok());

    let armor: Armor = result.unwrap().payload;
    assert_eq!(armor.hitpoints, 1400);
    assert_eq!(armor.resistances, [66.0, 23.5, 36.2, 44.8]);
    assert_eq!(armor.repair, 0);
    assert_eq!(armor.interval, 1.0);
}
