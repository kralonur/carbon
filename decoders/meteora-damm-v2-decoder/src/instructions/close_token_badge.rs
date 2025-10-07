use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6c92566eb3fe0a68")]
pub struct CloseTokenBadge {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseTokenBadgeInstructionAccounts {
    pub token_badge: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseTokenBadge {
    type ArrangedAccounts = CloseTokenBadgeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let token_badge = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;
        let rent_receiver = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(CloseTokenBadgeInstructionAccounts {
            token_badge,
            admin,
            rent_receiver,
            event_authority,
            program,
        })
    }
}
