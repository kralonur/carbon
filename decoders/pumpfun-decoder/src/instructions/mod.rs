use crate::PROGRAM_ID;

use super::PumpfunDecoder;
pub mod buy;
pub mod complete_event;
pub mod create;
pub mod create_event;
pub mod initialize;
pub mod sell;
pub mod set_params;
pub mod set_params_event;
pub mod trade_event;
pub mod withdraw;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum PumpfunInstruction {
    Buy(buy::Buy),
    Create(create::Create),
    Initialize(initialize::Initialize),
    Sell(sell::Sell),
    SetParams(set_params::SetParams),
    Withdraw(withdraw::Withdraw),
    CompleteEvent(complete_event::CompleteEvent),
    CreateEvent(create_event::CreateEvent),
    SetParamsEvent(set_params_event::SetParamsEvent),
    TradeEvent(trade_event::TradeEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for PumpfunDecoder {
    type InstructionType = PumpfunInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            PumpfunInstruction::Buy => buy::Buy,
            PumpfunInstruction::Create => create::Create,
            PumpfunInstruction::Initialize => initialize::Initialize,
            PumpfunInstruction::Sell => sell::Sell,
            PumpfunInstruction::SetParams => set_params::SetParams,
            PumpfunInstruction::Withdraw => withdraw::Withdraw,
            PumpfunInstruction::CompleteEvent => complete_event::CompleteEvent,
            PumpfunInstruction::CreateEvent => create_event::CreateEvent,
            PumpfunInstruction::SetParamsEvent => set_params_event::SetParamsEvent,
            PumpfunInstruction::TradeEvent => trade_event::TradeEvent,
        )
    }
}

#[cfg(test)]
mod tests {
    use carbon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
    use solana_sdk::{instruction::AccountMeta, pubkey};

    use super::*;

    #[test]
    fn test_decode_buy() {
        // Arrange
        let expected_ix = PumpfunInstruction::Buy(buy::Buy {
            amount: 2712969161192,
            max_sol_cost: 204000000,
        });
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                false,
            ),
            AccountMeta::new(
                pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("9p1PMtto471A7GvnRJVmDcuqUz3xDd1Lhu8vzrmpump"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HWxwYxr4AV5ytUyT8pvjCEiUrXhwpbx365VpvQ6Bd6MZ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AUfg9aTAix7YarkHXSBMUyQPCTq55Gg1Z2NTe6utwwzG"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4FLYmjhLuUb5ofNBo1PA9enF7HrPUSYUA1t55tUSFYa5"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5ztadiszGPmBeGVcvmtPyqiHRA8SpU8mqNzPV1WeV88F"),
                true,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                false,
            ),
        ];
        let expected_arranged_accounts = buy::BuyInstructionAccounts {
            global: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            fee_recipient: pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
            mint: pubkey!("9p1PMtto471A7GvnRJVmDcuqUz3xDd1Lhu8vzrmpump"),
            bonding_curve: pubkey!("HWxwYxr4AV5ytUyT8pvjCEiUrXhwpbx365VpvQ6Bd6MZ"),
            associated_bonding_curve: pubkey!("AUfg9aTAix7YarkHXSBMUyQPCTq55Gg1Z2NTe6utwwzG"),
            associated_user: pubkey!("4FLYmjhLuUb5ofNBo1PA9enF7HrPUSYUA1t55tUSFYa5"),
            user: pubkey!("5ztadiszGPmBeGVcvmtPyqiHRA8SpU8mqNzPV1WeV88F"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            event_authority: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
        };

        // Act
        let decoder = PumpfunDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/buy_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            buy::Buy::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
