#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_address);
    print!("difficuly: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let difficulty = difficulty.trim().parse::<u32>()
        .expect("we need an integer");

    let mut chain = blockchain::Chain::new(
        miner_address.trim().to_string(),
        difficulty
    );

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("enter your choice: ");

        io::stdout().flush();
        choice.clear();
        
        io::stdin().read_line(&mut choice);
        println!();

        match choice.trim().parse().unwrap() {
            0 => 
            {
                println!("exiting!");
                process::exit(0);
            },
            1 =>
            {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("enter sender address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                print!("enter receiver address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);
                print!("enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }
            },
            2 =>
            {
                println!("generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("block generated successfully"),
                    false => println!("block generation failed"),
                }
            },
            3 =>
            {
                let mut new_difficulty = String::new();
                print!("enter new difficulty");
                io::stdout().flush();
                io::stdin().read_line(&mut new_difficulty);
                let res = chain.update_difficulty(
                    new_difficulty.trim().parse().unwrap()
                );

                match res {
                    true => println!("updated difficulty"),
                    false => println!("failed to update difficulty"),
                }
            },
            4 =>
            {
                let mut new_reward = String::new();
                print!("enter new reward");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(
                    new_reward.trim().parse().unwrap()
                );

                match res {
                    true => println!("updated reward"),
                    false => println!("failed to update reward"),
                }
            },
            _ => println!("invalid option, please retry"),
        }
    }
}
