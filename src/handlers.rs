use borsh::BorshSerialize;
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    program::invoke, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar,
};

use crate::state::NotesState;
use crate::utils::assert_owner_program;

pub fn process_initialise_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    text: &str,
) -> ProgramResult {
    let mut account_iter = accounts.iter();
    let payer_account = next_account_info(&mut account_iter)?;
    let notes_account = next_account_info(&mut account_iter)?;
    let _system_program_account = next_account_info(&mut account_iter)?;

    let max_size = 200;
    let rent = Rent::get()?.minimum_balance(200);

    msg!("Required Rent {}", rent);

    invoke(
        &system_instruction::create_account(
            payer_account.key,
            notes_account.key,
            rent,
            max_size,
            program_id,
        ),
        accounts,
    )?;

    let notes_state = NotesState {
        content: text.to_string(),
        cursor_pos: text.len(),
    };

    let mut notes_account_data = &mut notes_account.data.borrow_mut()[..];
    notes_state.serialize(&mut notes_account_data)?;

    msg!("Initialised Account. Starting text: {}", text);
    Ok(())
}
