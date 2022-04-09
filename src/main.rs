// #[macro_use]
extern crate serde_derive;

use std::io::{self, Write};
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    io::stdout().flush().ok();
    io::stdin().read_line(&mut miner_addr).ok();
    print!("Difficulty: ");
    io::stdout().flush().ok();
    io::stdin().read_line(&mut difficulty).ok();
    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("We need an integer");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        println!("Enter your choice: ");
        io::stdout().flush().ok();
        choice.clear();
        io::stdin().read_line(&mut choice).ok();
        println!();

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting!");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut reciever = String::new();
                let mut amount = String::new();

                print!("Enter sender address: ");
                io::stdout().flush().ok();
                io::stdin().read_line(&mut sender).ok();
                print!("Enter reciever address:");
                io::stdout().flush().ok();
                io::stdin().read_line(&mut reciever).ok();
                print!("Enter amount: ");
                io::stdout().flush().ok();
                io::stdin().read_line(&mut amount).ok();

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    reciever.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed"),
                }
            }
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();

                match res {
                    true => println!("Block generated successfully!"),
                    false => println!("Block generated failed!"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("Enter new difficulty: ");
                io::stdout().flush().ok();
                io::stdin().read_line(&mut new_diff).ok();
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Difficulty is updated!"),
                    false => println!("Difficulty's update is failed!"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush().ok();
                io::stdin().read_line(&mut new_reward).ok();
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Reward is updated!"),
                    false => println!("Reward's update is failed!"),
                }
            }
            _ => println!("\tinvalid option please retry\t"),
        }
    }
}
