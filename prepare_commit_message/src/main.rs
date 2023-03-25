mod append_to_git_message;

use std::{fs, env};
use append_to_git_message::append_to_git_message;
use prepare_commit_message::emojis_attacher::EmojisAttacher;
use prepare_commit_message::CommitMsgParams;

fn main() {
    let mut messages = vec![];
    let commit_message_params = CommitMsgParams::new();

    println!("{:#?}", commit_message_params);
    let entries = fs::read_dir(".git").expect("No .git directory found");
    // check if any of the entries have the string "rebase"
    let is_rebase = entries
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .any(|entry| entry.contains("rebase"));
    print!("{}", is_rebase);

    if !is_rebase && (commit_message_params.commit_source == "template"
        || commit_message_params.commit_source == "message")
    {
        messages.push(EmojisAttacher::init());
    }

    append_to_git_message(&messages, &commit_message_params);
}
