
#[derive(Serialize, Deserialize, Debug)]
pub struct WarpScrambler {
    #[serde(rename="Range")]
    pub range: u32,
    #[serde(rename="Strength")]
    pub strength: u32,
    #[serde(rename="Duration")]
    pub duration: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    #[serde(rename="Warp Scrambler")]
    payload: WarpScrambler,
}

#[test]
fn deserialize() {
    use serde_json;
    let json = r#"{
        "Warp Scrambler" : {
            "Range" : 16000,
            "Strength" : 2,
            "Duration" : 10
        }
    }"#;

    let result: Result<Wrapper, _> = serde_json::from_str(&json);
    assert!(result.is_ok());

    let scrambler: WarpScrambler = result.unwrap().payload;
    assert_eq!(scrambler.range, 16000);
    assert_eq!(scrambler.strength, 2);
    assert_eq!(scrambler.duration, 10);
}
