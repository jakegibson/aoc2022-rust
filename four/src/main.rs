use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Day 4");
    part_one();
}



fn part_one() {
 
    let file_path = "input.txt";
    let file = File::open(file_path).unwrap();
    let file_reader = BufReader::new(file);
    
    let mut dup_count = 0;
    for line in file_reader.lines(){
        let line_value = line.unwrap();
        let mut assigment_bounds: Vec<i32> = vec![];
        for assignment in line_value.split(",") {
            assignment.split("-").for_each(|a| assigment_bounds.push(a.parse::<i32>().unwrap()));
        }
        println!("assignment bounds {:?}", assigment_bounds);
        if assigment_bounds[0] >= assigment_bounds[2] && assigment_bounds[1] <= assigment_bounds[3] {
            dup_count += 1;
        } else if  assigment_bounds[2] >= assigment_bounds[0] && assigment_bounds[3] <= assigment_bounds[1] {
            dup_count += 1;
        }

      
    }

    println!("dup count {dup_count}")
}


fn part_two() {
 
    let file_path = "input.txt";
    let file = File::open(file_path).unwrap();
    let file_reader = BufReader::new(file);
    
    let mut dup_count = 0;
    for line in file_reader.lines(){
        let line_value = line.unwrap();
        let mut assigment_bounds: Vec<i32> = vec![];
        for assignment in line_value.split(",") {
            assignment.split("-").for_each(|a| assigment_bounds.push(a.parse::<i32>().unwrap()));
        }
        println!("assignment bounds {:?}", assigment_bounds);
        let assignment_one = assigment_bounds[0]..=assigment_bounds[1];
        let assignment_two = assigment_bounds[2]..=assigment_bounds[3];
        if assignment_one.contains(&assigment_bounds[2]) || assignment_one.contains(&assigment_bounds[3]){
            dup_count += 1;
        } else if  assignment_two.contains(&assigment_bounds[0]) || assignment_two.contains(&assigment_bounds[1]){
            dup_count += 1;
        }

      
    }

    println!("dup count {dup_count}")
}
