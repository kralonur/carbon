use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtLiquidityChange {
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub transfer_fee_included_token_a_amount: u64,
    pub transfer_fee_included_token_b_amount: u64,
    pub reserve_a_amount: u64,
    pub reserve_b_amount: u64,
    pub liquidity_delta: u128,
    pub token_a_amount_threshold: u64,
    pub token_b_amount_threshold: u64,
    pub change_type: u8,
}
