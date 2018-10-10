
#[derive(Serialize, Deserialize, Debug)]
pub struct Movement {
    #[serde(rename="Orbit Range")]
    pub orbit_range: u32,
    #[serde(rename="Orbit Speed")]
    pub orbit_speed: u32,
    #[serde(rename="Chase Speed")]
    pub chase_speed: u32,
    #[serde(rename="Angular Velocity")]
    pub angular_velocity: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    #[serde(rename="Movement")]
    payload: Movement,
}

#[test]
fn deserialize() {
    use serde_json;
    let json = r#"{
        "Movement" : {
            "Orbit Range" : 11000,
            "Orbit Speed" : 1200,
            "Chase Speed" : 2600,
            "Angular Velocity" : 0.10909091
        }
    }"#;

    let result: Result<Wrapper, _> = serde_json::from_str(&json);
    assert!(result.is_ok());

    let movement: Movement = result.unwrap().payload;
    assert_eq!(movement.orbit_range, 11000);
    assert_eq!(movement.orbit_speed, 1200);
    assert_eq!(movement.chase_speed, 2600);
    assert_eq!(movement.angular_velocity, 0.10909091);
}
