use file_search::{cli, web};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    println!("Choose an option:");
    println!("1) Run CLI");
    println!("2) Run Web Server");
    print!("Your choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "1" => {
            cli::run();
        }
        "2" => {
            web::start().await;
        }
        _ => {
            println!("Invalid choice. Exiting.");
        }
    }
}
