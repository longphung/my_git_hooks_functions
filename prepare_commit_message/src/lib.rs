use std::env::args;

pub mod emojis_attacher;

pub struct PrepareCommitMsgParams {
    commit_msg_file: String,
    commit_source: String,
    sha1: String,
}

impl PrepareCommitMsgParams {
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

        PrepareCommitMsgParams { commit_msg_file, commit_source, sha1 }
    }

    pub fn get_commit_msg_file(&self) -> String {
        self.commit_msg_file.clone()
    }

    pub fn get_commit_source(&self) -> String {
        self.commit_source.clone()
    }

    pub fn get_sha1(&self) -> String {
        self.sha1.clone()
    }
}
