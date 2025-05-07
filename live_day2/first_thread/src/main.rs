use std::path::Path;
use anyhow::Context;
use serde::Deserialize;

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("myfile.txt");
    std::fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let contents = maybe_read_a_file()?;
    Ok(contents.to_uppercase())
}

#[derive(Deserialize, Debug)]
struct User {
    name: String,
    password: String,
}

type MyError  = Box<dyn std::error::Error + Sync + Send>;

fn load_users() -> Result<Vec<User>, UsersError> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)
        .inspect_err(|err|{
          println!("Oh dear");  
        }  )
        .context("This really wasn't meant to happen")
        .map_err(|e| UsersError::FileNotFound)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)
        .map_err(|_| UsersError::InvalidJson)?;
    Ok(users)
}

use thiserror::Error;

#[derive(Debug, Error)]
enum UsersError {
    #[error("File does not exist")]
    FileNotFound,
    #[error("Invalid JSON")]
    InvalidJson,
}

fn main() {
    match load_users() {
        Ok(text) => println!("File contents: {text:?}"),
        Err(e) => {
            println!("An error occurred: {e:?}");
        },
    }
}