use authentication::{get_users, save_users, LoginRole, User};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,
    /// Add a user.
    Add {
        /// Username
        username: String,

        /// Password
        password: String,

        /// Optional - mark as an admin
        #[arg(long)]
        admin: Option<bool>,
    },
    /// Delete a user
    Delete {
        /// Username
        username: String,
    },
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Login Action");
    println!("{:-<40}", "");

    let users = get_users();
    users
        .iter()
        .for_each(|user| {
            println!("{:<20}{:<20?}", user.username, user.role);
        });
}

fn add_user(username: String, password: String) {
    let mut users = get_users();
    users.push(User {
        username, password,
        role: LoginRole::Admin,
    });
    save_users(&users);
}

fn delete_user(username: String) {
    let mut users = get_users();
    users.retain(|user| { user.username != username });
    save_users(&users);
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users();
        }
        Some(Commands::Add { username, password, admin }) => {
            add_user(username, password);
        }
        Some(Commands::Delete { username }) => {
            delete_user(username);
        }
        None => {
            println!("Run with --help to see instructions");
            std::process::exit(0);
        }
    }
}