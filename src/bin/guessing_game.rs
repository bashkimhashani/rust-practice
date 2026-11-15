use rand::RngExt;
use std::io;


fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(1..=100);
    //println!("The secret number is {}", secret_number);

    loop {
        print!("Input your guess: ");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a valid number!");

        println!("You guessed: {}", guess);

        if guess < secret_number {
            println!("Try a bigger number!");
        } else if guess > secret_number {
            println!("Try a smaller number");
        } else {
            println!("You guessed correctly!!");
            break;
        }
    }
}