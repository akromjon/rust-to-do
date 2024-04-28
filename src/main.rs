mod usage;
mod validator;
fn main() {
    
    let command: Result<validator::Command, i32> = validator::validate();

    match command {
        Ok(command) => println!("You are running {}", command.command),
        Err(error_code) => println!("Error occurred: {}", error_code),
    }
}
