
#[derive(Serialize, Deserialize, Debug)]
pub struct Turrets {
    #[serde(rename="Damage")]
    pub damage: [u32; 4],
    #[serde(rename="Interval")]
    pub inteval: f32,
    #[serde(rename="Range")]
    pub range: u32,
    #[serde(rename="Falloff")]
    pub falloff: u32,
    #[serde(rename="Tracking")]
    pub tracking: f32,
    #[serde(rename="OptimalSignature")]
    pub signature: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    #[serde(rename="Turrets")]
    payload: Turrets,
}

#[test]
fn deserialize() {
    use serde_json;
    let json = r#"{
            "Turrets" : {
                "Damage" : [0, 192, 38, 330],
                "Interval" : 2.0,
                "Range" : 500,
                "Falloff" : 10000,
                "Tracking" : 1.5,
                "OptimalSignature" : 40
            }
         }"#;

    let result: Result<Wrapper, _> = serde_json::from_str(&json);
    assert!(result.is_ok());

    let turrets: Turrets = result.unwrap().payload;
    assert_eq!(turrets.damage, [0, 192, 38, 330]);
    assert_eq!(turrets.inteval, 2.0);
    assert_eq!(turrets.range, 500);
    assert_eq!(turrets.falloff, 10000);
    assert_eq!(turrets.tracking, 1.5);
    assert_eq!(turrets.signature, 40);
}
