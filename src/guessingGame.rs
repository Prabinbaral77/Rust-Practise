use rand::prelude::*;
use std::io;

fn main() {
    println!("Welcome to Gussing game");

    let fruits_list: [&str; 5] = ["apple", "mango", "banana", "orange", "graps"];
    loop {
        let mut rng: ThreadRng = thread_rng();
        let index: usize = rng.gen_range(0..fruits_list.len());
        let random_fruit: &str = fruits_list[index];
        println!("random fruit selected = {}", random_fruit);

        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let gues_fruit: String = input.trim().to_lowercase();
                if !fruits_list.contains(&gues_fruit.as_str()) {
                    println!("Fruits not found in the list");
                    continue;
                }
                if check_result(random_fruit, &gues_fruit) {
                    println!("Your guess is correct. You won!");
                    break;
                } else {
                    println!("Your guess is incorrect. You Loose!");
                    continue;
                }
            }
            Err(err) => {
                println!("Error {}", err);
            }
        }
    }
}

fn check_result(random_fruit: &str, user_guess: &str) -> bool {
    return random_fruit == user_guess;
}
