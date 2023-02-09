use std::io;
mod number_guess;
mod fizzbuzz;

fn main() {
    println!("Please select an option:");
    println!("1. Number Guesser");
    println!("2. FizzBuzz");

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Invalid input!");

    match option.as_str().trim(){
        "1" => number_guess::guess_the_number(),
        "2" => fizzbuzz::fizzbuzz_until(),
        _ => {}
    }
}
