use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtClaimPartnerFee {
    pub pool: solana_pubkey::Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}
