use crate::usage::display;
use std::env;
use std::process;

pub struct Command {
    pub command: String,
    pub description: String,
}

pub fn validate() -> Result<Command, i32> {
    let arguments: Vec<String> = env::args().collect();

    let command = arguments.get(1).unwrap_or_else(|| {
        display();

        process::exit(0);
    });

    let valid_command: Result<Command, i32> = match command.as_str() {
        "add" => {
            let arg_two = arguments.get(2).unwrap_or_else(|| {
                println!("add needs a description");

                process::exit(0);
            });

            Ok(Command {
                command: "add".to_string(),
                description: arg_two.as_str().to_string(),
            })
        }

        "list" => Ok(Command {
            command: "list".to_string(),
            description: "".to_string(),
        }),
        _ => {
            display();
            process::exit(0);
        }
    };

    return valid_command;
}
