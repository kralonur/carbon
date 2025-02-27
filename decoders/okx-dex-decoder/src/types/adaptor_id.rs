

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum AdaptorID {
    Bridge0,
    Bridge1,
    Bridge2,
    Bridge3,
    Bridge4,
    Bridge5,
    Bridge6,
    Bridge7,
    Bridge8,
    Bridge9,
    Bridge10,
    Bridge11,
    Bridge12,
    Bridge13,
    Bridge14,
    Bridge15,
    Bridge16,
    Bridge17,
    Cctp,
    Bridge19,
    Bridge20,
    Wormhole,
    Meson,
    Bridge23,
    Bridge24,
    Bridge25,
    Bridge26,
    Bridge27,
    Bridge28,
    Bridge29,
    Bridge30,
    Bridge31,
    Bridge32,
    Bridge33,
    Debridgedln,
}


