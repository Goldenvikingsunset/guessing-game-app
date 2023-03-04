use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::time::{Instant};

fn main() {
    let mut high_scores: Vec<(String, u32, u128)> = Vec::new();

    loop {
        println!("Welcome to guess the number!");
        println!("Select a difficulty level (1-5):");

        let mut difficulty_level = String::new();

        io::stdin()
            .read_line(&mut difficulty_level)
            .expect("Failed to read line");

        let difficulty_level: u32 = match difficulty_level.trim().parse() {
            Ok(num) if num >= 1 && num <= 5 => num,
            _ => {
                println!("Invalid difficulty level!");
                continue;
            }
        };

        let range = match difficulty_level {
            1 => 1..=10,
            2 => 1..=50,
            3 => 1..=100,
            4 => 1..=500,
            5 => 1..=1000,
            _ => unreachable!(),
        };

        let secret_number = rand::thread_rng().gen_range(range);

        let mut score = 0;
        let start_time = Instant::now();

        println!("Please enter your name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");

        loop {
            println!("What number do you guess?");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            score += 1;

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    let elapsed_time = start_time.elapsed();
                    let elapsed_time_secs = elapsed_time.as_secs();
                    let elapsed_time_millis = elapsed_time.subsec_millis();
                    let total_time_millis = elapsed_time_secs * 1000 + u64::from(elapsed_time_millis);

                    let score_with_time_bonus = score + (total_time_millis / 1000) as u32;
                    println!("Congratulations, {}! You win!", name.trim());
                    println!("Your score is: {} ({} seconds)", score_with_time_bonus, total_time_millis / 1000);

                    high_scores.push((name.trim().to_string(), score_with_time_bonus, total_time_millis.into()));
                    high_scores.sort_by(|a, b| a.1.cmp(&b.1).then(a.2.cmp(&b.2)));
                    high_scores.truncate(10);

                    println!("High Scores:");
                    for (name, score, time) in &high_scores {
                        println!("{} - {} ({} seconds)", name, score, time / 1000);
                    }

                    break;
                }
            }
        }

        println!("Do you want to play again? (y/n)");

        let mut play_again = String::new();

        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.trim().to_lowercase() != "y" {
            println!("Thanks for playing!");
            break;
        }
    }
}

