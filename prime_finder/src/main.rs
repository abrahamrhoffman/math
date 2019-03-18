use std::io;
use std::io::Write;

fn is_prime(x: &i32) {
    // Create a vector composed of numbers from 1 -> 9
    let mut first_nums = Vec::new();
    let mut num: i32 = 1;
    while num != 10 {
        first_nums.push(num);
        num = num + 1;
    }
    // Test x to see if it is prime
    let nine: i32 = 9;
    for ele in first_nums.iter() {
        if ele == &nine {
            println!("{}", ele);
        } else {
            let result = (x/ele) as f64;
            print!("{},", result);
            io::stdout().flush().unwrap();
        }
    }
}

fn main() {
    // Create a vector of 100 numbers
    let mut reals = Vec::new();
    let mut num: i32 = 1;
    while num != 101 {
        reals.push(num);
        num = num + 1;
    }
    // Test each number to see if it is prime
    for ele in reals.iter() {
        is_prime(ele);
    }
}