use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtSetPoolStatus {
    pub pool: solana_pubkey::Pubkey,
    pub status: u8,
}
