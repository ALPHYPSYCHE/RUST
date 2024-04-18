use rand::Rng;
use std::io;

fn main() {
    println!(" ");
    println!("########### WELCOME TO MY GAME PLATFORM ###########");

    let name = greeting();
    let eligible = age(&name);

    if eligible {
        let mut balance = 100; 
        menu(&name, &mut balance);
    } else {
        println!(" ");
    }
}

fn greeting() -> String {
    println!(" ");
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you.";
    io::stdin().read_line(&mut name).expect("Did not receive input");
    println!(" ");
    println!("Hello {}! {}", name.trim(), greeting);
    name
}

fn age(name: &str) -> bool {
    println!(" ");
    println!("How old are you, {}?", name.trim());
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Did not receive input");
    let age: i32 = match age.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid age.");
            return false; // if parsing fails
        }
    };
    if age >= 18 {
        println!(" ");
        println!("You are over 18. You can play.");
        true
    } else {
        println!(" ");
        println!("Sorry {}, you are not eligible to play.", name.trim());
        false
    }
}

fn menu(name: &str, balance: &mut i32) {
    loop {
        println!(" ");
        println!("{}, Please choose a game (1-4), or enter 'q' to quit:", name.trim());
        println!(" ");
        println!("1. Tic Tac Toe (18$)");
        println!("2. Hangman (15$)");
        println!("3. Sudoku (20$)");
        println!("4. Snake (16$)");
        println!(" ");
        println!("Your current account balance is: {} $", balance);

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                if *balance >= 18 {
                    println!(" ");
                    println!("** You chose 1: Tic Tac Toe! **");
                    *balance -= 18;
                    println!("You have been charged 18$. Your remaining balance is {} $", balance);
                    // Tic Tac Toe game
                } else {
                    println!(" ");
                    println!("Insufficient balance! Please choose another game or 'q' to quit.");
                }
            }
            "2" => {
                if *balance >= 15 {
                    println!(" ");
                    println!("** You chose 2: Hangman! **");
                    *balance -= 15;
                    println!("You have been charged 15$. Your remaining balance is {} $", balance);
                    // Hangman game
                } else {
                    println!(" ");
                    println!("Insufficient Balance! Please Recharge Your Balance First!");
                }
            }
            "3" => {
                if *balance >= 20 {
                    println!(" ");
                    println!("** You chose 3: Sudoku! **");
                    *balance -= 20;
                    println!("You have been charged 20$. Your remaining balance is {} $", balance);
                    // Sudoku game
                } else {
                    println!(" ");
                    println!("Insufficient balance! Please choose another game or 'q' to quit.");
                }
            }
            "4" => {
                if *balance >= 16 {
                    println!(" ");
                    println!("** You chose 4: Snake! **");
                    *balance -= 16;
                    println!("You have been charged 16$. Your remaining balance is {} $", balance);
                    // Snake game
                } else {
                    println!(" ");
                    println!("Insufficient balance! Please choose another game or 'q' to quit.");
                }
            }
            "q" => {
                println!(" ");
                println!("** Quitting... **");
                break;
            }
            _ => {
                println!(" ");
                println!("Invalid choice! Please enter a number between 1 and 4, or 'q' to quit.");
            }
        }
    }
}
