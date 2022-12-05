use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
fn main() {
    println!("Day 5");
    part_two();
}



fn part_one() {
    let mut original_order: HashMap<&str, Vec<char>> = HashMap::from([
        ("1", vec!['H', 'T', 'Z', 'D']),
        ("2", vec!['Q', 'R', 'W', 'T', 'G', 'C', 'S']),
        ("3", vec!['P', 'B', 'F', 'Q', 'N', 'R', 'C', 'H']),
        ("4", vec!['L', 'C', 'N', 'F', 'H', 'Z']),
        ("5", vec!['G', 'L', 'F', 'Q', 'S']),
        ("6", vec!['V', 'P', 'W', 'Z', 'B', 'R', 'C', 'S']),
        ("7", vec!['Z', 'F', 'J']),
        ("8", vec!['D', 'L', 'V', 'Z', 'R', 'H', 'Q']),
        ("9", vec!['B', 'H', 'G', 'N', 'F', 'Z', 'L', 'D']),

    ]);
    let file_path = "input.txt";
    let file = File::open(file_path).unwrap();
    let file_reader = BufReader::new(file);
    for line in file_reader.lines(){
        
        let line_value = line.unwrap();
        
        let mut values = line_value.split_whitespace();
        values.next();
        let move_count = values.next().unwrap().parse::<i32>().unwrap();
        values.next();
        let from_col = values.next().unwrap();
        values.next();
        let to_col = values.next().unwrap();
        let l = original_order[from_col].len();
        let mut start = original_order[from_col].len() as i32  - move_count;
        if start < 0 {
            start = 0;
        }
        println!("len {} move {}, start {}", l, move_count, start);

        let  crates;
        {
        let from_col_vector =  original_order.get_mut(from_col).unwrap();
        crates = from_col_vector.split_off(start as usize);
        }
        let to_col_vector = original_order.get_mut(to_col).unwrap();
        
        crates.into_iter().rev().for_each( |c| to_col_vector.push(c) );
   
    }
    original_order.into_iter().for_each(|(k, v)| {
        println!("{}: {}", k, v.into_iter().collect::<String>());
    });
}

// Only had to drop .rev() call for part_two
fn part_two() {
    let mut original_order: HashMap<&str, Vec<char>> = HashMap::from([
        ("1", vec!['H', 'T', 'Z', 'D']),
        ("2", vec!['Q', 'R', 'W', 'T', 'G', 'C', 'S']),
        ("3", vec!['P', 'B', 'F', 'Q', 'N', 'R', 'C', 'H']),
        ("4", vec!['L', 'C', 'N', 'F', 'H', 'Z']),
        ("5", vec!['G', 'L', 'F', 'Q', 'S']),
        ("6", vec!['V', 'P', 'W', 'Z', 'B', 'R', 'C', 'S']),
        ("7", vec!['Z', 'F', 'J']),
        ("8", vec!['D', 'L', 'V', 'Z', 'R', 'H', 'Q']),
        ("9", vec!['B', 'H', 'G', 'N', 'F', 'Z', 'L', 'D']),

    ]);
    let file_path = "input.txt";
    let file = File::open(file_path).unwrap();
    let file_reader = BufReader::new(file);
    for line in file_reader.lines(){
        
        let line_value = line.unwrap();
        
        let mut values = line_value.split_whitespace();
        values.next();
        let move_count = values.next().unwrap().parse::<i32>().unwrap();
        values.next();
        let from_col = values.next().unwrap();
        values.next();
        let to_col = values.next().unwrap();
        let l = original_order[from_col].len();
        let mut start = original_order[from_col].len() as i32  - move_count;
        if start < 0 {
            start = 0;
        }
        println!("len {} move {}, start {}", l, move_count, start);

        let  crates;
        {
        let from_col_vector =  original_order.get_mut(from_col).unwrap();
        crates = from_col_vector.split_off(start as usize);
        }
        let to_col_vector = original_order.get_mut(to_col).unwrap();
        
        crates.into_iter().for_each( |c| to_col_vector.push(c) );
   
    }
    original_order.into_iter().for_each(|(k, v)| {
        println!("{}: {}", k, v.into_iter().collect::<String>());
    });
}
