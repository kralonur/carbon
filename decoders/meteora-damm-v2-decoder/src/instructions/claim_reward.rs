use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x955fb5f25e5a9ea2")]
pub struct ClaimReward {
    pub reward_index: u8,
    pub skip_reward: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimRewardInstructionAccounts {
    pub pool_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub reward_vault: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub user_token_account: solana_pubkey::Pubkey,
    pub position_nft_account: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimReward {
    type ArrangedAccounts = ClaimRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let reward_vault = next_account(&mut iter)?;
        let reward_mint = next_account(&mut iter)?;
        let user_token_account = next_account(&mut iter)?;
        let position_nft_account = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(ClaimRewardInstructionAccounts {
            pool_authority,
            pool,
            position,
            reward_vault,
            reward_mint,
            user_token_account,
            position_nft_account,
            owner,
            token_program,
            event_authority,
            program,
        })
    }
}
