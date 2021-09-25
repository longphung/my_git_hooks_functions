use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

use prepare_commit_message::CommitMsgParams;

pub fn append_to_git_message(messages: &Vec<String>, commit_message_params: &CommitMsgParams) {
    let mut commit_message_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&commit_message_params.commit_msg_file)
        .expect("Unable to open git commit message file: .git/COMMIT_EDITMSG");
    let mut commit_template_data = vec![];
    match commit_message_file.read_to_end(&mut commit_template_data) {
        Err(_) => {
            println!("No template found")
        }
        _ => {}
    };

    let write_error_message = "Error while preparing commit message!";

    commit_message_file.set_len(0).expect(write_error_message);
    commit_message_file
        .seek(SeekFrom::Start(0))
        .expect(write_error_message);

    for message in messages {
        println!("message: {}", message);
        if let Err(_) = commit_message_file.write(message.as_bytes()) {
            println!("{}", write_error_message);
        }
    }

    if &commit_message_params.commit_source != "message" {
        commit_message_file
            .write("\n".as_bytes())
            .expect(write_error_message);
    }
    commit_message_file
        .write(&commit_template_data.as_slice())
        .expect(write_error_message);
}
