use solana_program::account_info::AccountInfo;
use solana_program::entrypoint;
use solana_program::entrypoint_deprecated::ProgramResult;
use solana_program::pubkey::Pubkey;
use crate::processor::Processor;

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)
}