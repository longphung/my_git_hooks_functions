mod append_to_git_message;

use std::{fs};
use append_to_git_message::append_to_git_message;
use prepare_commit_message::emojis_attacher::EmojisAttacher;
use prepare_commit_message::CommitMsgParams;

fn main() {
    let mut messages = vec![];
    let commit_message_params = CommitMsgParams::new();

    println!("{:#?}", commit_message_params);
    // Skip if no .git directory is found
    let entries = fs::read_dir(".git");
    let mut is_rebase = false;

    if entries.is_ok() {
        // check if any of the entries have the string "rebase"
        is_rebase = entries
            .expect("Error reading .git directory")
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .any(|entry| entry.contains("rebase"));
        print!("{}", is_rebase);
    }

    if !is_rebase && (commit_message_params.commit_source == "template"
        || commit_message_params.commit_source == "message")
    {
        messages.push(EmojisAttacher::init());
    }

    append_to_git_message(&messages, &commit_message_params);
}
