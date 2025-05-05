use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn greet_user(name: &str)  -> String {
    format!("Hello {}", name)
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

pub fn get_default_users() -> Vec<User> {
    let users = vec![
        User::new("admin", "password", LoginRole::Admin),
        User::new("bob", "password", LoginRole::User),
    ];
    users
}

pub fn get_users() -> Vec<User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // Load the file
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: Vec<User> = serde_json::from_str(&users_json).unwrap();
        users
    } else {
        // Create a file and return it
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

pub fn save_users(users: &Vec<User>) {
    let users_path = Path::new("users.json");
    let users = get_default_users();
    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(users_path, users_json).unwrap();
}

pub fn get_users_hashmap() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("bob".to_string(), User::new("bob", "password", LoginRole::User));
    users
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users_hashmap = get_users_hashmap();
    if let Some(user) = users_hashmap.get(username) {}
    
    let users = get_users();
    if let Some(user) = users.iter().find(|user| user.username == username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role));
        } else {
            return Some(LoginAction::Denied);
        }
    }
    None
}
