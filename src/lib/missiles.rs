
#[derive(Serialize, Deserialize, Debug)]
pub struct Missiles {
    #[serde(rename="Damage")]
    pub damage: [u32; 4],
    #[serde(rename="Interval")]
    pub inteval: f32,
    #[serde(rename="Range")]
    pub range: u32,
    #[serde(rename="Explosion Velocity")]
    pub velocity: u32,
    #[serde(rename="Explosion Radius")]
    pub radius: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    #[serde(rename="Missiles")]
    payload: Missiles,
}

#[test]
fn deserialize() {
    use serde_json;
    let json = r#"{
        "Missiles" : {
            "Damage" : [165, 0, 0, 0],
            "Interval" : 3.0,
            "Range" : 10125,
            "Explosion Velocity" : 150,
            "Explosion Radius" : 20
        }
    }"#;

    let result: Result<Wrapper, _> = serde_json::from_str(&json);
    assert!(result.is_ok());

    let missiles: Missiles = result.unwrap().payload;
    assert_eq!(missiles.damage, [165, 0, 0, 0]);
    assert_eq!(missiles.inteval, 3.0);
    assert_eq!(missiles.range, 10125);
    assert_eq!(missiles.velocity, 150);
    assert_eq!(missiles.radius, 20);
}
