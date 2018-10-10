#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate lib;

use lib::targeting::SensorType;
use lib::targeting::Targeting;


fn main() {

    #[derive(Serialize, Deserialize, Debug)]
    struct Wrapper{
        #[serde(rename="Targeting")]
        payload: Targeting,
    };

    let payload = Targeting {
        attack_range: 50000,
        signature: 18.0,
        resolution: 2000.0,
        attack_targets: 6,
        lock_targets: 6,
        lock_speed: 3.0,
        sensor_type: SensorType::Magnetometric,
        sensor_strength: 30,
    };


    let point = Wrapper{payload};
    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);
    let deserialized: Wrapper = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
