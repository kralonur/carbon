use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xacf1dd8aa11dfd2a")]
pub struct SplitPosition {
    pub params: SplitPositionParameters,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SplitPositionInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub first_position: solana_pubkey::Pubkey,
    pub first_position_nft_account: solana_pubkey::Pubkey,
    pub second_position: solana_pubkey::Pubkey,
    pub second_position_nft_account: solana_pubkey::Pubkey,
    pub first_owner: solana_pubkey::Pubkey,
    pub second_owner: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SplitPosition {
    type ArrangedAccounts = SplitPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let pool = next_account(&mut iter)?;
        let first_position = next_account(&mut iter)?;
        let first_position_nft_account = next_account(&mut iter)?;
        let second_position = next_account(&mut iter)?;
        let second_position_nft_account = next_account(&mut iter)?;
        let first_owner = next_account(&mut iter)?;
        let second_owner = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(SplitPositionInstructionAccounts {
            pool,
            first_position,
            first_position_nft_account,
            second_position,
            second_position_nft_account,
            first_owner,
            second_owner,
            event_authority,
            program,
        })
    }
}
