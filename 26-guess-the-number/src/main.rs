use rand::{thread_rng, Rng};

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    fn get_number_range(&self) -> (i32, i32) {
        match self {
            Difficulty::Easy => (1, 10),
            Difficulty::Medium => (1, 100),
            Difficulty::Hard => (1, 1000),
        }
    }
}

impl std::str::FromStr for Difficulty {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" | "easy" | "Easy" | "EASY" | "E" => Ok(Difficulty::Easy),
            "m" | "medium" | "Medium" | "MEDIUM" | "M" => Ok(Difficulty::Medium),
            "h" | "hard" | "Hard" | "HARD" | "H" => Ok(Difficulty::Hard),
            _ => Err(String::from("Not valid Difficulty")),
        }
    }
}

fn main() {
    let stdin = std::io::stdin();

    println!("Let's play Guess the number");

    println!("Pick a difficulty level (easy(e), medium(m), hard(h))");
    println!("The number you have to guess will be based on difficulty you choose");
    println!("Easy: 1-10");
    println!("Medium: 1-100");
    println!("Hard: 1-1000");

    let mut difficulty = String::new();

    stdin.read_line(&mut difficulty).unwrap();

    let difficulty: Difficulty = difficulty
        .trim()
        .parse::<Difficulty>()
        .expect("Please enter e, m, or h");

    let secret = generate_guess(difficulty.get_number_range());

    println!("I have my number. What's your guess?");

    let mut guess;
    let mut guess_count = 0;

    loop {
        guess = String::new();

        stdin.read_line(&mut guess).unwrap();

        let guess = match guess.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a number. Try again");
                continue;
            }
        };

        guess_count += 1;

        if guess == secret {
            println!("You got it in {guess_count} guesses");
            break;
        } else if guess > secret {
            println!("Too high. Guess again: ");
        } else {
            println!("Too low. Guess again: ");
        }
    }
}

fn generate_guess((low, high): (i32, i32)) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(low, high + 1)
}
