use borsh::from_slice;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{handlers::process_initialise_instruction, instructions::NotesInstructions};

pub mod handlers;
pub mod instructions;
pub mod state;

entrypoint!(process_instruction);

fn process_instruction(
    __program_id: &Pubkey,
    __accounts: &[AccountInfo],
    __instruction_data: &[u8],
) -> ProgramResult {
    let instruction: NotesInstructions =
        from_slice(__instruction_data).map_err(|_e| ProgramError::InvalidInstructionData)?;
    match instruction {
        NotesInstructions::InitialiseNotesAccount { initial_text } => {
            process_initialise_instruction(__program_id, __accounts, &initial_text)?
        }

        _ => return Err(ProgramError::InvalidInstructionData),
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use borsh::BorshSerialize;
    use litesvm::LiteSVM;
    use solana_sdk::{
        entrypoint::ProgramResult,
        instruction::{AccountMeta, Instruction},
        message::Message,
        signature::Keypair,
        signer::Signer,
        system_program,
        transaction::Transaction,
    };

    use crate::instructions::NotesInstructions;

    #[test]
    fn test_process_initialise_instruction() -> ProgramResult {
        let mut svm = LiteSVM::new();
        let payer_account = Keypair::new();

        let program = Keypair::new();
        let program_id = program.pubkey();

        svm.airdrop(&payer_account.pubkey(), 1_000_000_000_000)
            .unwrap();
        svm.add_program_from_file(program_id, "./target/deploy/notes.so")
            .unwrap();

        let notes_account = Keypair::new();
        let initial_text = "Hello World".to_string();
        let instruct = NotesInstructions::InitialiseNotesAccount { initial_text };
        let mut buf = Vec::<u8>::new();
        instruct.serialize(&mut buf).unwrap();

        let init_instruction = Instruction::new_with_bytes(
            program_id,
            buf.as_slice(),
            vec![
                AccountMeta::new(payer_account.pubkey(), true),
                AccountMeta::new(notes_account.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        let init_message = Message::new(&[init_instruction], Some(&payer_account.pubkey()));
        let init_transaction = Transaction::new(
            &[payer_account, notes_account],
            init_message,
            svm.latest_blockhash(),
        );

        let result = svm.send_transaction(init_transaction).unwrap();
        let logs = result.logs;

        println!("Logs:\n {:#?}", logs);
        Ok(())
    }
}
