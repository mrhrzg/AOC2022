use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn calc(input: &str) -> u32 {
    let len = &input.len();
    let first = &input[..len / 2];
    let second = &input[len / 2..];
    for c in first.chars() {
        if second.contains(c) {
            return value(c);
        }
    }
    panic!("should have found a match");
}

fn value(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn filelines(input: &str) -> std::io::Lines<BufReader<File>> {
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);
    reader.lines()
}

fn process(input: &str) -> u32 {
    let lines = filelines(input);

    let points_combined: u32 = lines.map(|l| calc(&l.unwrap())).sum();

    println!("{points_combined} points all together.");
    points_combined
}

fn groupbadgeprocess(input: &str) -> u32 {
    let lines = input.split('\n').collect::<Vec<&str>>();

    let points_combined = lines
        .as_slice()
        .chunks_exact(3)
        .map(|threes| groupbadge(threes))
        .sum();
    println!("group badge: {points_combined} points all together.");
    points_combined
}

fn groupbadge(rucksacks: &[&str]) -> u32 {
    for c in rucksacks[0].chars() {
        if rucksacks[1].contains(c) && rucksacks[2].contains(c) {
            //dbg!(value(c.clone()));
            return value(c);
        }
    }
    panic!("the 3 rucksacks did not have an overlapping value");
}

fn main() {
    let input = "day03/input.txt";
    process(input);
    let lines: &str = include_str!("../input.txt");
    groupbadgeprocess(lines);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        assert_eq!(process("input_sample.txt"), 157)
    }

    #[test]
    fn test_sample_2() {
        let lines: &str = include_str!("../input_sample.txt");
        assert_eq!(groupbadgeprocess(lines), 70)
    }
}
