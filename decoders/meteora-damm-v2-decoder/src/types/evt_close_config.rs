use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtCloseConfig {
    pub config: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}
