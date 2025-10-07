use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtSplitPosition2 {
    pub pool: solana_pubkey::Pubkey,
    pub first_owner: solana_pubkey::Pubkey,
    pub second_owner: solana_pubkey::Pubkey,
    pub first_position: solana_pubkey::Pubkey,
    pub second_position: solana_pubkey::Pubkey,
    pub current_sqrt_price: u128,
    pub amount_splits: SplitAmountInfo,
    pub first_position_info: SplitPositionInfo,
    pub second_position_info: SplitPositionInfo,
    pub split_position_parameters: SplitPositionParameters2,
}
