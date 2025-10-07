use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtFundReward {
    pub pool: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub mint_reward: solana_pubkey::Pubkey,
    pub reward_index: u8,
    pub amount: u64,
    pub transfer_fee_excluded_amount_in: u64,
    pub reward_duration_end: u64,
    pub pre_reward_rate: u128,
    pub post_reward_rate: u128,
}
