
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
fn main() {
    println!("Day 6");
    //part one input is 4 and 14 for part two
    find_marker(14);
}

fn find_marker(seq_length: usize) {
    // read in file as a String
    let file_path = "input.txt";
    let file = File::open(file_path).unwrap();
    let file_reader = BufReader::new(file);
    let mut lines = file_reader.lines();
    let mut line = lines.next().unwrap().unwrap();
    let mut index = 0;
    // detect the first instance of 4 characters without duplicates in the line String
    for (i, c) in line.chars().enumerate() {
        if i < seq_length {
            continue;
        }
        let window = &line[i - seq_length ..=i];
        let mut seen = HashSet::new();
       
        for c in window.chars() {
          
            seen.insert(c);
        }
        println!("set: {:?}", seen);
        if seen.len() == seq_length {
            // found four consecutive characters with no duplicates at index i
            index = i+1;
            break;
        }
    }
    println!("index {}", index);

}