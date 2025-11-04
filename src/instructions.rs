use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum NotesInstructions {
    InitialiseNotesAccount { initial_text: String },
    Write { text: String },
    Append { text: String },
    Erase { len: usize },
    Read,
}
