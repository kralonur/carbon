use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x58ce005b3caf9776")]
pub struct CreateTokenBadge {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateTokenBadgeInstructionAccounts {
    pub token_badge: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateTokenBadge {
    type ArrangedAccounts = CreateTokenBadgeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let token_badge = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(CreateTokenBadgeInstructionAccounts {
            token_badge,
            token_mint,
            admin,
            system_program,
            event_authority,
            program,
        })
    }
}
