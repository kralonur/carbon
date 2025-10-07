use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SplitPositionParameters {
    pub unlocked_liquidity_percentage: u8,
    pub permanent_locked_liquidity_percentage: u8,
    pub fee_a_percentage: u8,
    pub fee_b_percentage: u8,
    pub reward_0_percentage: u8,
    pub reward_1_percentage: u8,
    pub padding: [u8; 16],
}
