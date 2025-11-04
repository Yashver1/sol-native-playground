use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct NotesState {
    pub content: String,
    pub cursor_pos: usize,}
