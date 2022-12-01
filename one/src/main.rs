use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
fn main() {
    println!("Hello, world!");
    //part_one();
    part_two();
}

fn get_file_lines() -> BufReader<File> {
    let file_path = "input.txt";
    let file = File::open(file_path).unwrap();
    // let reader = BufReader::new(file);
    let reader = BufReader::new(file);
    return reader;
}

fn part_one() {
    let reader = get_file_lines();
    let mut current_sum = 0;
    let mut max_sum = 0;
    for line in reader.lines() {
        let calories = match line.unwrap().parse::<i32>() {
            Ok(cal) => {
                current_sum += cal;
                cal
            },
            Err(error) => {
                if current_sum > max_sum {
                    max_sum = current_sum;
                }
                current_sum = 0;
                0
            }
        };
        println!("{}", calories);
    }
    println!("max calories {max_sum}");
}

fn part_two() {
    let reader = get_file_lines();
    let mut current_sum = 0;
    let mut max_sum = 0;
    let mut totals:Vec<i32> = vec![];
    for line in reader.lines() {
        let calories = match line.unwrap().parse::<i32>() {
            Ok(cal) => {
                current_sum += cal;
                cal
            },
            Err(error) => {
                totals.push(current_sum);
                current_sum = 0;
                0
            }
        };
        
    }
    totals.sort();
    let l = totals.len();
    let top_three = totals[l-1] + totals[l-2] + totals[l-3];
    println!("top three {top_three}");
}