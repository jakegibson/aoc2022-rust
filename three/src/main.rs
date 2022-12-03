use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;
use std::str;
use std::iter::FromIterator;
fn main() {
    println!("Day 3");
    part_two();
}



fn part_one() {
    let alphabet: Vec<&str> = vec![
        "a", "b", "c", "d", "e", 
        "f", "g", "h", "i", "j", 
        "k", "l", "m", "n", "o",
        "p", "q", "r", "s", "t", 
        "u", "v", "w", "x", "y", 
        "z", "A", "B", "C", "D",
        "E", "F", "G", "H", "I",
        "J", "K", "L", "M", "N", 
        "O", "P", "Q", "R", "S",
        "T", "U", "V", "W", "X",
        "Y", "Z"
    ];
    let mut total_score = 0;
    let mut score_map: HashMap<&str, i32> = HashMap::new();
    for (i, letter) in alphabet.iter().enumerate() {
        let s = (i as i32) + 1;
        score_map.insert(letter, s);
    }
    let file_path = "input.txt";
    let file = File::open(file_path).unwrap();
    let file_reader = BufReader::new(file);
    
 
    for line in file_reader.lines(){
        let line_value = line.unwrap();
        let midpoint = line_value.len() / 2;
        let pack: Vec<char>= line_value.chars().collect();
        let pack_one: HashSet<char> = HashSet::from_iter(pack[0..midpoint].iter().cloned());

        for (i, ch) in pack.iter().enumerate() {
            if i >= midpoint {
                let duplicate = pack_one.contains(ch);
                if duplicate {
                    let k = ch.to_string();
                    let score = score_map[k.as_str()];
                    total_score += score;
                    break;
                }
            }
        }
    }

   println!("total score {total_score}")
}


fn part_two() {
    let alphabet: Vec<&str> = vec![
        "a", "b", "c", "d", "e", 
        "f", "g", "h", "i", "j", 
        "k", "l", "m", "n", "o",
        "p", "q", "r", "s", "t", 
        "u", "v", "w", "x", "y", 
        "z", "A", "B", "C", "D",
        "E", "F", "G", "H", "I",
        "J", "K", "L", "M", "N", 
        "O", "P", "Q", "R", "S",
        "T", "U", "V", "W", "X",
        "Y", "Z"
    ];
    let mut total_score = 0;
    let mut score_map: HashMap<&str, i32> = HashMap::new();
    for (i, letter) in alphabet.iter().enumerate() {
        let s = (i as i32) + 1;
        score_map.insert(letter, s);
    }
    let file_path = "input.txt";
    let file = File::open(file_path).unwrap();
    let file_reader = BufReader::new(file);
    
    let mut group = vec![];
    for (i, line) in file_reader.lines().enumerate(){
        let line_value = line.unwrap();
        group.push(line_value);
        if (i + 1) % 3 == 0 {
            for ch in group[0].chars() {
                if group[1].contains(ch) && group[2].contains(ch) {
                    let k = ch.to_string();
                    let score = score_map[k.as_str()];
                    total_score += score;
                    break;
                }
            }
            group = vec![]
        }

    }

   println!("total score {total_score}")
}