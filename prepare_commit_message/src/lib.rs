use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

pub mod emojis_attacher;

pub struct CommitMsgParams {
    commit_msg_file: String,
    commit_source: String,
    sha1: String,
    commit_msg_content: String,
}

impl CommitMsgParams {
    pub fn new() -> Self {
        let mut args = args().skip(1);
        let commit_msg_file = match args.next() {
            Some(x) => x,
            None => String::new(),
        };
        let commit_source = match args.next() {
            Some(x) => x,
            None => String::new(),
        };
        let sha1 = match args.next() {
            Some(x) => x,
            None => String::new(),
        };
        let commit_msg_content = read_to_string(Path::new(&commit_msg_file)).expect(&format!("Could not read commit message from {}", commit_source));

        CommitMsgParams { commit_msg_file, commit_source, sha1, commit_msg_content }
    }
}
