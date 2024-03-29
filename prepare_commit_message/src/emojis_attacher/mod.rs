use std::io;

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

        loop {
            let mut selection = String::new();
            io::stdin()
                .read_line(&mut selection)
                .expect("Error while reading input");

            let trimmed_selection = selection.trim();

            println!("Selection: {}", trimmed_selection);

            match (*trimmed_selection).parse::<usize>() {
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

                    return format!("{}: ", mapped_selection);
                }
                Err(_) => {
                    println!("\nNot a valid choice. Choose again.\n");
                }
            }
        }
    }
}

fn get_message_types() -> Vec<String> {
    let mut types_list_trigger = false;
    let mut types: Vec<Vec<&str>> = vec![];
    let emojis_template = String::from_utf8_lossy(include_bytes!("../../data/emojis_template.txt"));

    for line in emojis_template.split("\n") {
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
