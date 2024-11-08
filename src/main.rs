mod network;
mod constants;
mod protocol;
mod utils;
mod program;
mod tests;

use std::io::{self, Write};
use env_logger::Env;
use log::{info, error};

use crate::program::{run_program};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting the Monero handshake program\n");

    println!("Welcome to the Monero handshaker!\n");
    println!("Please select which chain you would like to connect to.");
    println!("1. Mainnet");
    println!("2. TestNet");
    println!("3. StageNet");
    println!("4. Exit program");
    print!("Please select an option: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    let chain: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number between 1 and 4.");
            return Ok(());
        }
    };

    if chain == 4 {
        error!("Exiting program...");
        return Ok(());
    }

    let _ = run_program(chain).await;

    Ok(())
}