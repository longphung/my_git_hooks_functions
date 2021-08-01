mod append_to_git_message;

use append_to_git_message::append_to_git_message;
use prepare_commit_message::emojis_attacher::EmojisAttacher;
use prepare_commit_message::CommitMsgParams;

fn main() {
    let mut messages = vec![];
    let CommitMsgParams { commit_source, .. } = CommitMsgParams::new();

    if commit_source == "template" || commit_source == "message" {
        messages.push(EmojisAttacher::init());
    }

    append_to_git_message(&messages);
}
