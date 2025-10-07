use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdd93e4cf8cd41177")]
pub struct SplitPosition2 {
    pub numerator: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SplitPosition2InstructionAccounts {
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

impl carbon_core::deserialize::ArrangeAccounts for SplitPosition2 {
    type ArrangedAccounts = SplitPosition2InstructionAccounts;

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

        Some(SplitPosition2InstructionAccounts {
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
