// Note that this is not functional, it is just a placeholder to create a 2.0.0 compatible SDK
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, instruction::Instruction,
    pubkey::Pubkey,
};

declare_id!("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV");

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(noop);

pub fn noop(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}

pub fn instruction(data: Vec<u8>) -> Instruction {
    Instruction {
        program_id: crate::id(),
        accounts: vec![],
        data,
    }
}
