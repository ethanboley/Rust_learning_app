
// importing packages

use std::io::{self, Write};
use rand::Rng;

// use of struct to define... essentially the guessing game

struct NumGuess {
    small: i32,
    big: i32,
}

// use of impl to handle methods for guessing

impl NumGuess {
    fn new(small: i32, big: i32) -> Self {
        NumGuess {
            small: if small < big { small } else { big },
            big: if big > small { big } else { small },
        }
    }

    fn guess(&mut self, num: i32, guess: i32, guesses: i32) -> i32 {
        if guess == num {
            println!("Correct!");
            return self.big - self.small - guesses;
        } else if guess < num {
            if guess >= num - 10 {
                println!("Close, higher!");
                return 0;
            }
            println!("Higher!");
            return 0;
        } else if guess > num {
            if guess - 10 <= num {
                println!("Close, lower!");
                return 0;
            }
            println!("Lower!");
            return 0;
        } else {
            println!("Nope!");
            return 0;
        }
    }

    fn gen_num(&self) -> i32 {
        rand::thread_rng().gen_range(self.small..self.big)
    }
}

// use of struct to define player class

struct Player {
    name: String,
    score: i32,
}

// usage of impl for player class

impl Player {
    fn new(name: String) -> Self {
        Player {
            name,
            score: 0,
        }
    }

    fn ask_playing(&self) -> String {
        println!("Still playing {}? You have {} points!", self.name, self.score);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }
}

// main function with game loop inside

fn main() {
    println!("Welcome, what is your name?");
    let mut name = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();
    let mut pc = Player::new(name);
    loop { 
        // game loop using the loop loop kind of loop
        println!("Enter a small number and hit enter");
        io::stdout().flush().unwrap();
        let mut small = String::new();
        io::stdin().read_line(&mut small).expect("Failed to read line");
        let small: i32 = match small.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Enter a bigger number and hit enter");
        io::stdout().flush().unwrap();
        let mut big = String::new();
        io::stdin().read_line(&mut big).expect("Failed to read line");
        let big: i32 = match big.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let mut guesser = NumGuess::new(small, big);
        let num = guesser.gen_num();
        let mut guesses = 0;
        loop {
            // now after everything has been defined start guessing with another loop loop implementation
            print!("Enter a number to guess: ");
            io::stdout().flush().unwrap();
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            let score = guesser.guess(num, guess, guesses);
            if score != 0 {
                pc.score += score;
                println!("{} got {} points for a total of {} points!", pc.name, score, pc.score);
                break;
            }
        }
        let user = pc.ask_playing();
        if user == "n" || user == "" || user == "N" || user == "no" || user == "No" || user == "NO" || user == "0" || user == "false" {
            println!("Have a good day then!");
            break;
        }
    }
}
