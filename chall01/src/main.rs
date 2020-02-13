/*
 * This program calculates the multiplicative persistence of a number.
 * Usage: ./chall01 <number>
 * */
use std::env;

fn main() {
    // Parse the arguments from cli
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        calculate_range(args);
    } else if args.len() == 2 {
        calculate_number(args);
    } else {
        panic!("No argument found");
    }
}

fn calculate_range(args: Vec<String>) {
    let number1 = args[1].parse::<u32>().unwrap();
    let number2 = args[2].parse::<u32>().unwrap() + 1;

    let range: Vec<u32> = (number1..number2).collect();

    for (_index, number) in range.iter().enumerate() {
        let vect: Vec<u32> = calculate(*number);
        print!("\"{}\": {}, ", number, vect.len() + 1);
    }
}

fn calculate_number(args: Vec<String>) {
    let vect: Vec<u32> = calculate(args[1].parse::<u32>().unwrap());
    println!("{:?}", vect);
}

fn calculate(mut number: u32) -> Vec<u32> {
    // Vector for storing the results
    let mut vect: Vec<u32> = Vec::new();

    if number > 9 {
        while number > 9 {
            number = get_product(number);
            vect.push(number);
        }
    } else {
        vect.push(number);
    }

    vect
}

fn get_product(number: u32) -> u32 {
    let digits: Vec<_> = number
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    let mut result: u32 = 1;

    for item in digits.iter() {
        result *= item;
    }

    result
}
