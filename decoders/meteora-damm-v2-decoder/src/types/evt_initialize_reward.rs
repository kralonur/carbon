use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtInitializeReward {
    pub pool: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub reward_index: u8,
    pub reward_duration: u64,
}
