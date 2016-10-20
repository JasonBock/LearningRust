extern crate collatz;
extern crate rand;

use rand::Rng;
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let value = if arguments.len() > 1 { 
        match arguments[1].trim().parse::<u64>() {
			Ok(num) => num,
			Err(_) => panic!("You did not enter in a valid unsigned 64-bit integer."),
		}
    } else { 
        rand::thread_rng().gen::<u64>()
    };

    println!("Starting value: {}", value);

    let results = collatz::collatz::analyze(value);

    println!("Sequence length: {}", results.sequence.len());        

    for sequence_value in &results.sequence {
        println!("{}", sequence_value);        
    }
}