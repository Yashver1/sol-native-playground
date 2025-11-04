use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey, system_program,
};

pub fn assert_owner_program(program_id: &Pubkey, account: &AccountInfo) -> ProgramResult {
    if account.key != &system_program::id() && account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    Ok(())
}
