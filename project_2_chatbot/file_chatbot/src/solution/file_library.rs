use kalosm::language::*;

// Look at the docs for std::fs
// https://doc.rust-lang.org/std/fs/index.html
// std::fs provides functions that write to a file, read from a file,
// check if a file exists, etc.
use std::fs;

// LlamaChatSession provides helpful functions for loading and storing sessions.
// Look at https://docs.rs/kalosm/latest/kalosm/language/trait.ChatSession.html#saving-and-loading-sessions
// for some examples!

// Implement this
pub fn save_chat_session_to_file(filename: &str, session: &LlamaChatSession) {
    // look at fs::write(...)
    unimplemented!("Saving chat session to file {filename}");
}

// Implement this
pub fn load_chat_session_from_file(filename: &str) -> Option<LlamaChatSession> {
    // using option so that can return none if the loading fails (chats idea)
    let bytes = fs::read(filename).ok()?;
    //takes the bytes from the 'filename' and stores bytes in variable, if 'filename' doesn't exist return None
    //the ok() converts the result into an option, and the ? converts the option into a value
    // we need type to be bytes in order to put into from_bytes so .ok()? is necessary
    let session = LlamaChatSession::from_bytes(&bytes).ok()?;
    // converts the raw bytes into a LlamaChatSession object (stores conversation history)
    //stores the LlamaChatSession object in session
    //if this fails, return None
    Some(session)
    //if both succeed,
}