
#[derive(Serialize, Deserialize, Debug)]
pub struct StasisWebifier {
    #[serde(rename="Range")]
    pub range: u32,
    #[serde(rename="SpeedReduction")]
    pub reduction: i32,
    #[serde(rename="Duration")]
    pub duration: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    #[serde(rename="Stasis Webifier")]
    payload: StasisWebifier,
}

#[test]
fn deserialize() {
    use serde_json;
    let json = r#"{
        "Stasis Webifier" : {
            "Range" : 18000,
            "SpeedReduction" : -60,
            "Duration" : 20
        }
    }"#;

    let result: Result<Wrapper, _> = serde_json::from_str(&json);
    assert!(result.is_ok());

    let webifier: StasisWebifier = result.unwrap().payload;
    assert_eq!(webifier.range, 18000);
    assert_eq!(webifier.reduction, -60);
    assert_eq!(webifier.duration, 20);
}
