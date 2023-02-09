use std::io;

pub fn fizzbuzz_until() {
    println!("Welcome to FizzBuzz, please enter a number:");

    let mut fizzbuzz_limit = String::new();

    io::stdin()
        .read_line(&mut fizzbuzz_limit)
        .expect("Failed to read line");

    let fizzbuzz_limit: u32 = match fizzbuzz_limit.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("");

    for i in 1..=fizzbuzz_limit {
        match Some(i) {
            Some(x) if x % 3 == 0 => {
                println!("Fizz");
                continue;
            },
            Some(x) if x % 5 == 0 => println!("Buzz"),
            _ => println!("{i}")
        }
        // if i % 3 {
        //     println!("Fizz")
        // }
        // if i % 5 {
        //     println!("Buzz")
        // }
    }
}
