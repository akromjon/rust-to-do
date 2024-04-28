mod to_do;
mod usage;
mod validator;
fn main() {
    let command: Result<validator::Command, i32> = validator::validate();

    match command {
        
        Ok(command) => {

            if command.command == String::from("add") {

                let _ = to_do::add(command);

            } else {

                let contents = to_do::all();


                match contents {
                    Ok(contents) => println!("{contents}"),
                    Err(error) => println!("{error}"),
                }
            }
        }

        Err(error_code) => println!("Error occurred: {}", error_code),
    }
}
