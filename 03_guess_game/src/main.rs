use rand::Rng;
use std::io;
fn main() {
    println!("Welcome to our guessing game");
    println!("You have to find the secret number");

    // Create a thread_rng, which is seeded by the operating system
    let mut rng = rand::thread_rng();
    // Generate a random number between 0 and 9
    let n: u8 = rng.gen_range(0..10);

   

    let mut chances = 3;

    println!("You have only {} chances",chances);

    let mut game: bool = true;
    while game{

        // takes input to find the secrect number
    
        let mut ask = String::new();
        io::stdin().read_line(&mut ask).expect("Failed to get input!.");
        let ask:u8 = ask.trim().parse().expect("Please enter a integer");
   
        if ask == n{
            println!("You win!");
            game = false;
        }
        else if ask < n{
            println!("Too low!");
        }
        else{
            println!("Too high!");
        }
        
        if ask != n{
            chances = chances - 1;
            println!("You have only {} chances left!", chances);
            }
            if chances == 0{
                println!("You lost the game!...");
                println!("The secrect number was {}", n);
                game = false;
            }
    }
}
