use std::io;
use crate::CommitMsgParams;

pub struct EmojisAttacher;

impl EmojisAttacher {
    pub fn init() -> String {
        println!("Available types:");

        let mut count = 0;
        let messages_types = get_message_types();
        let mut available_options: Vec<usize> = vec![];
        for message_type in &messages_types {
            count += 1;
            available_options.push(count);
            println!("{}. {}", count, message_type);
        }

        println!("Choose a type by No.:");

        let mut selection = String::new();

        loop {
            io::stdin()
                .read_line(&mut selection)
                .expect("Error while reading input");

            println!("Selection: {}", selection);

            match selection.trim().parse::<usize>() {
                Ok(selection) => {
                    if selection > messages_types.len() {
                        println!(
                            "\nThere are only {} options. Choose again.\n",
                            available_options.len()
                        );
                        continue;
                    }

                    let mapped_selection = &messages_types[selection - 1];

                    if mapped_selection == "<empty>" {
                        return String::new();
                    }

                    return format!("{}:\n", mapped_selection);
                }
                Err(_) => {
                    println!("\nNot a 1 valid choice. Choose again.\n");
                }
            }
        }
    }
}

fn get_message_types() -> Vec<String> {
    let commit_msg_params = CommitMsgParams::new();

    let mut types_list_trigger = false;
    let mut types: Vec<Vec<&str>> = vec![];

    for line in commit_msg_params.commit_msg_content.split("\n") {
        if line.contains("<type>") {
            types_list_trigger = true;
            continue;
        } else if types_list_trigger && line.contains("</type>") {
            break;
        } else if !types_list_trigger {
            continue;
        }

        types.push(line[2..].split(":").collect());
    }

    types.push(vec!["<empty>"]);

    let types: Vec<String> = types.into_iter().map(|x| String::from(x[0])).collect();

    types
}
