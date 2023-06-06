use std::fs::File;
use std::io::{self, BufRead};
use ndarray::prelude::*;
use ndarray::Array1; // Added import for ndarray

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("vec1.txt")?;
    let reader = io::BufReader::new(file);

    // Read the lines and parse the numbers
    let numbers: Vec<f64> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse().ok())
        .collect();

    // Convert the Vec<f64> to ndarray array object
    let array = Array1::from_vec(numbers); // Using Array1 directly without "ndarray::"

    // Print the array
    println!("{:?}", array);

    Ok(())
}

