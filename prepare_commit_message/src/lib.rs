use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

pub mod emojis_attacher;

pub struct CommitMsgParams {
    pub commit_msg_file: String,
    pub commit_source: String,
    pub sha1: String,
    pub commit_msg_content: String,
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
        let commit_msg_content = read_to_string(Path::new(&commit_msg_file)).expect(&format!(
            "Could not read commit message from {}",
            commit_source
        ));

        Self {
            commit_msg_file,
            commit_source,
            sha1,
            commit_msg_content,
        }
    }
}
