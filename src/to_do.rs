use crate::validator::Command;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, Read};

pub fn add(to_do: Command) -> std::io::Result<()> {
    
    let mut file = OpenOptions::new()
        .write(true) // Open the file in write mode
        .create(true) // Create the file if it doesn't exist
        .append(true) // Append to the file if it exists
        .open("todo.txt")?;

    // Write some data to the file
    write!(
        file,
        "{}",
        String::from("todo: ") + &to_do.description + &String::from("\n")
    )?;

    // Close the file
    file.flush()?;

    Ok(())
}

pub fn all() -> io::Result<String> {
    // Open the file
    let mut file = File::open("todo.txt")?;

    // Read the file contents into a string
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}
