
use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    println!("Day 2");
    // part_one();
    part_two();
}



fn part_one() {
    let file_path = "input.txt";
    let file = File::open(file_path).unwrap();
    let file_reader = BufReader::new(file);
    let mut total_points = 0;
    for (i, line) in file_reader.lines().enumerate() {
        let round_points = match line.unwrap().as_str() {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => panic!("wut"),
        };
        total_points += round_points;
    }
    println!("total points {total_points}")
}



fn part_two() {
    let file_path = "input.txt";
    let file = File::open(file_path).unwrap();
    let file_reader = BufReader::new(file);
    let mut total_points = 0;
    let letters = vec!["X", "Y", "Z", "X", "Y"];
    let indexes = HashMap::from([
        ("A", 3),
        ("B", 1),
        ("C", 2),
    ]);
    for (i, line) in file_reader.lines().enumerate() {
        let l = line.unwrap();
        let mut value_str_iter = l.split_whitespace();
        let oponent_move = value_str_iter.next().unwrap();
        let strategy = value_str_iter.next().unwrap();
        // Winning move is the letter to the left and losing to the right
        // Rock -> loses to Paper, loses to -> Scissors -> (with wrap around) to rock
        let my_move = match strategy {
            "X" => letters[indexes[oponent_move] - 1],
            "Y" => letters[indexes[oponent_move]],
            "Z" => letters[indexes[oponent_move] + 1],
            _ => panic!("Done messed up Arod")
        };
        
        // Squish back together so I can reuse counter from part one
        let play = format!("{} {}", oponent_move, my_move);

        let round_points = match play.as_str() {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => panic!("wut"),
        };
        total_points += round_points;
    
    }
    println!("total points {total_points}")
}