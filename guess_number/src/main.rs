use rand::random;
use std::io;

fn main() {
    let secret_number: u32 = random::<u32>() % 10;
    let mut iterations: u32 = 0;

    println!("Guess the number!");
    loop {
        iterations += 1;
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please enter a number");

        if guess == secret_number {
            println!("You guessed it in {} moves", iterations);
            break;
        } else {
            let plus_ou_moins = if guess < secret_number { "it's more" } else { "it's less" };
            println!("Wrong guess. {}", plus_ou_moins);
        }
    }
}
