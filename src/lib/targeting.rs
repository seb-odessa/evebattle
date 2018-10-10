
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SensorType {
    Gravimetric,
    Radar,
    Ladar,
    Magnetometric
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Targeting {
    #[serde(rename="Attack Range")]
    pub attack_range: u32,
    #[serde(rename="Signature Radius")]
    pub signature: f32,
    #[serde(rename="Scan Resolution")]
    pub resolution: f32,
    #[serde(rename="Attack Targets")]
    pub attack_targets: u32,
    #[serde(rename="Locked Targets")]
    pub lock_targets: u32,
    #[serde(rename="Targeting Speed")]
    pub lock_speed: f32,
    #[serde(rename="Sensor Type")]
    pub sensor_type: SensorType,
    #[serde(rename="Sensor Strength")]
    pub sensor_strength: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    #[serde(rename="Targeting")]
    payload: Targeting,
}

#[test]
fn deserialize() {
    use serde_json;
    let json = r#"{
        "Targeting" : {
            "Attack Range" : 50000,
            "Signature Radius" : 18.0,
            "Scan Resolution" : 2000.0,
            "Attack Targets" : 6,
            "Locked Targets" : 6,
            "Targeting Speed" : 3.0,
            "Sensor Type" : "Magnetometric",
            "Sensor Strength" : 30
        }
    }"#;

    let result: Result<Wrapper, _> = serde_json::from_str(&json);
    assert!(result.is_ok());

    let targeting: Targeting = result.unwrap().payload;
    assert_eq!(targeting.attack_range, 50000);
    assert_eq!(targeting.signature, 18.0);
    assert_eq!(targeting.resolution, 2000.0);
    assert_eq!(targeting.attack_targets, 6);
    assert_eq!(targeting.lock_targets, 6);
    assert_eq!(targeting.lock_speed, 3.0);
    assert_eq!(targeting.sensor_type, SensorType::Magnetometric);
    assert_eq!(targeting.sensor_strength, 30);
}
