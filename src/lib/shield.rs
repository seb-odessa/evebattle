
#[derive(Serialize, Deserialize, Debug)]
pub struct Shield {
    #[serde(rename="Hitpoints")]
    pub hitpoints: u32,
    #[serde(rename="Resistancies")]
    pub resistancies: [f32; 4],
    #[serde(rename="Recharge")]
    pub recharge: f32,
    #[serde(rename="Boost")]
    pub boost: u32,
    #[serde(rename="Interval")]
    pub interval: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    #[serde(rename="Shield")]
    payload: Shield,
}

#[test]
fn deserialize() {
    use serde_json;
    let json = r#"{
        "Shield" : {
            "Hitpoints" : 2300,
            "Resistancies" : [35.1, 67.6, 61.1, 48.1],
            "Recharge" : 8.83,
            "Boost" : 175,
            "Interval" : 2.5
        }
    }"#;

    let result: Result<Wrapper, _> = serde_json::from_str(&json);
    assert!(result.is_ok());

    let shield: Shield = result.unwrap().payload;
    assert_eq!(shield.hitpoints, 2300);
    assert_eq!(shield.resistancies, [35.1, 67.6, 61.1, 48.1]);
    assert_eq!(shield.recharge, 8.83);
    assert_eq!(shield.boost, 175);
    assert_eq!(shield.interval, 2.5);
}
