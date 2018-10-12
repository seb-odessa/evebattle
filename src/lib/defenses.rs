use shield::Shield;
use armor::Armor;
use structure::Structure;

#[derive(Serialize, Deserialize, Debug)]
pub struct Defenses {
    #[serde(rename="Shield")]
    pub shield: Shield,
    #[serde(rename="Armor")]
    pub armor: Armor,
    #[serde(rename="Structure")]
    pub structure: Structure,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    #[serde(rename="Defenses")]
    payload: Defenses,
}

#[test]
fn deserialize() {
    use serde_json;
    let json = r#"{
        "Defenses" : {
            "Shield" : {
                "Hitpoints" : 1000,
                "Resistances": [12.5, 56.3, 47.5, 30],
                "Recharge" : 3.84,
                "Boost" : 0,
                "Interval" : 1.0
            },
            "Armor" : {
                "Hitpoints" : 2500,
                "Resistances": [68.5, 43.3, 59.0, 59.0],
                "Repair" : 195,
                "Interval" : 3.0
            },
            "Structure" : {
                "Hitpoints" : 2000,
                "Resistances": [60.0, 60.0, 60.0, 60.0],
                "Repair" : 0,
                "Interval" : 1.0
            }
        }
    }"#;

    let result: Result<Wrapper, _> = serde_json::from_str(&json);
    assert!(result.is_ok());
}

