use home;

use std::fs::read_to_string;
use std::io;
use std::path::Path;

pub struct EmojisAttacher;

impl EmojisAttacher {
    pub fn init() -> String {
        println!("Available types:");

        let mut count = 0;
        let messages_types = get_message_types();
        let mut available_options: Vec<usize> = vec![];
        for message_type in get_message_types() {
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
                    return format!("{}:\n", &messages_types[selection - 1]);
                }
                Err(_) => {
                    println!("\nNot a 1 valid choice. Choose again.\n");
                }
            }
        }
    }
}

fn get_message_types() -> Vec<String> {
    let mut git_message_path_string;
    match home::home_dir() {
        Some(path) => git_message_path_string = path,
        None => panic!("Impossible to get your home dir!"),
    }

    git_message_path_string.push(".gitmessage");

    let git_message = read_to_string(Path::new(&git_message_path_string.into_os_string()))
        .expect("Could not read gitmessage template");

    let mut types_list_trigger = false;
    let mut types: Vec<Vec<&str>> = vec![];

    for line in git_message.split("\n") {
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

    let types: Vec<String> = types.into_iter().map(|x| String::from(x[0])).collect();

    types
}
