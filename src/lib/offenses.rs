use turrets::Turrets;
use warp_scrambler::WarpScrambler;
use stasis_webifier::StasisWebifier;


#[derive(Serialize, Deserialize, Debug)]
pub struct Offenses {
    #[serde(rename="Turrets")]
    pub turrets: Option<Turrets>,
    #[serde(rename="Warp Scrambler")]
    pub scrambler: Option<WarpScrambler>,
    #[serde(rename="Stasis Webifier")]
    pub webifier: Option<StasisWebifier>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wrapper {
    #[serde(rename="Offenses")]
    payload: Offenses,
}

#[test]
fn deserialize() {
    use serde_json;
    let json = r#"{
        "Offenses" : {
            "Turrets" : {
                "Damage" : [0, 0, 462, 330],
                "Interval" : 3.0,
                "Range" : 12000,
                "Falloff" : 15000,
                "Tracking" : 1.5,
                "OptimalSignature" : 40
             },
            "Warp Scrambler" : {
                "Range" : 16000,
                "Strength" : 2,
                "Duration" : 10
            },
            "Stasis Webifier" : {
                "Range" : 18000,
                "SpeedReduction" : -90,
                "Duration" : 20
            }
        }
    }"#;

    let result: Result<Wrapper, _> = serde_json::from_str(&json);
    assert!(result.is_ok());

    let offenses: Offenses = result.unwrap().payload;
    assert!(offenses.turrets.is_some());
    assert!(offenses.scrambler.is_some());
    assert!(offenses.webifier.is_some());
}

#[test]
fn deserialize_without_turrets() {
    use serde_json;
    let json = r#"{
        "Offenses" : {
            "Warp Scrambler" : {
                "Range" : 16000,
                "Strength" : 2,
                "Duration" : 10
            },
            "Stasis Webifier" : {
                "Range" : 18000,
                "SpeedReduction" : -90,
                "Duration" : 20
            }
        }
    }"#;

    let result: Result<Wrapper, _> = serde_json::from_str(&json);
    assert!(result.is_ok());

    let offenses: Offenses = result.unwrap().payload;
    assert!(offenses.turrets.is_none());
    assert!(offenses.scrambler.is_some());
    assert!(offenses.webifier.is_some());
}

#[test]
fn deserialize_without_scrambler() {
    use serde_json;
    let json = r#"{
        "Offenses" : {
            "Turrets" : {
                "Damage" : [0, 0, 462, 330],
                "Interval" : 3.0,
                "Range" : 12000,
                "Falloff" : 15000,
                "Tracking" : 1.5,
                "OptimalSignature" : 40
             },
            "Stasis Webifier" : {
                "Range" : 18000,
                "SpeedReduction" : -90,
                "Duration" : 20
            }
        }
    }"#;

    let result: Result<Wrapper, _> = serde_json::from_str(&json);
    assert!(result.is_ok());

    let offenses: Offenses = result.unwrap().payload;
    assert!(offenses.turrets.is_some());
    assert!(offenses.scrambler.is_none());
    assert!(offenses.webifier.is_some());
}

#[test]
fn deserialize_without_webifier() {
    use serde_json;
    let json = r#"{
        "Offenses" : {
            "Turrets" : {
                "Damage" : [0, 0, 462, 330],
                "Interval" : 3.0,
                "Range" : 12000,
                "Falloff" : 15000,
                "Tracking" : 1.5,
                "OptimalSignature" : 40
             },
            "Warp Scrambler" : {
                "Range" : 16000,
                "Strength" : 2,
                "Duration" : 10
            }
        }
    }"#;

    let result: Result<Wrapper, _> = serde_json::from_str(&json);
    assert!(result.is_ok());

    let offenses: Offenses = result.unwrap().payload;
    assert!(offenses.turrets.is_some());
    assert!(offenses.scrambler.is_some());
    assert!(offenses.webifier.is_none());
}