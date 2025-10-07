use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PoolFeeParameters {
    pub base_fee: BaseFeeParameters,
    pub padding: [u8; 3],
    pub dynamic_fee: Option<DynamicFeeParameters>,
}
