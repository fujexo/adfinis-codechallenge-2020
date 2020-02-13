/*
 * Solver for the second challenge
 * */

use std::env;

fn main() {
    // Parse the arguments from cli
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("not enough arguments");
    }

    // Convert args string to Vec<&str>
    let v: Vec<&str> = args[1].split_terminator(',').collect();

    // convert the Vec<&str> to Vec<u32>
    let mut sequence = converttovect(v);
    sequence.sort_by(|a, b| a.cmp(b).reverse());

    let sequence2 = solve_vector(sequence.clone());
    let mut sequence3 = solve_vector(sequence2.clone());

    sequence3.reverse();

    println!("Key, Val");
    for (index, number) in sequence3.iter().enumerate() {
        println!("{}: {}, ", index + 1, number);
    }
}

fn solve_vector(mut sequence: Vec<u32>) -> Vec<u32> {
    let mut new_sequence: Vec<u32> = Vec::new();
    let mut sequence_lenght = sequence.len() - 1;

    while sequence_lenght > 0 {
        new_sequence.push(sequence[0] - sequence[1]);
        sequence.remove(0);
        sequence_lenght -= 1;
    }

    new_sequence
}

fn converttovect(input: Vec<&str>) -> Vec<u32> {
    let mut output: Vec<u32> = Vec::new();
    for i in input.iter() {
        output.push(i.parse().unwrap());
    }

    output
}
