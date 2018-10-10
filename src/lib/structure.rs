
#[derive(Serialize, Deserialize, Debug)]
pub struct Structure {
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
    #[serde(rename="Structure")]
    payload: Structure,
}

#[test]
fn deserialize() {
    use serde_json;
    let json = r#"{
        "Structure" : {
            "Hitpoints" : 1000,
            "Resistances": [60.0, 60.0, 60.0, 60.0],
            "Repair" : 0,
            "Interval" : 1.0
        }
    }"#;

    let result: Result<Wrapper, _> = serde_json::from_str(&json);
    assert!(result.is_ok());

    let armor: Structure = result.unwrap().payload;
    assert_eq!(armor.hitpoints, 1000);
    assert_eq!(armor.resistances, [60.0, 60.0, 60.0, 60.0]);
    assert_eq!(armor.repair, 0);
    assert_eq!(armor.interval, 1.0);
}
