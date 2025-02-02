use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::rng();
    let number : u8 = rng.random_range(1..10);
    println!("I have a number in my mind(between 0, 10), let's see if you can guess it.");
    println!("You only have 5 guesses.");
    let mut hearts : u8 = 5;
    let mut gussed_num : u8;
    let mut result : bool = false;
    loop{
        println!("Guess no: {}\nWhat's your guess?", (6-hearts));
        let mut buffer : String = String::new();
        io::stdin().read_line(&mut buffer).expect("Error reading input");
        gussed_num = buffer.trim().parse().expect("Invalid input, please input only numbers.");
        if gussed_num==number{
            result = true;
            println!("Your Guess is correct.");
            println!("Score = {}", hearts*20);
        }
        else{
            hearts -= 1;
            println!("Wrong Guess.");
        }
        if hearts<1{
            break;
        }
    }
    if result==true {
        println!("You won the game.");
    }
    else{
        println!("You lost the game.");
    }
}
