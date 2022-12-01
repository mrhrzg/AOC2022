use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let f = File::open("day01/input.txt").unwrap();
    let reader = BufReader::new(f);
    let lines = reader.lines();
    let mut all_carrieres = Vec::new();
    let mut max_candidate = 0_usize;
    let mut sum_current_carrier = 0_usize;
    for line in lines {
        if line.as_ref().unwrap().is_empty(){
            max_candidate = max_candidate.max(sum_current_carrier);
            all_carrieres.push(sum_current_carrier);
            sum_current_carrier = 0;
        } 
        else {
            let weight = line.unwrap().parse::<usize>().unwrap();
            sum_current_carrier += weight;
        }
    }
    
    if sum_current_carrier != 0 {
            max_candidate = max_candidate.max(sum_current_carrier);
            all_carrieres.push(sum_current_carrier);
    } 
    
    println!("The Elf carrying the most carries {max_candidate} Calories.");
    
    all_carrieres.sort();
    let top3: usize = all_carrieres.iter().rev().take(3).sum();

    println!("The Elf carrying the 3 heaviest loads carry all together {top3} Calories.");
}

