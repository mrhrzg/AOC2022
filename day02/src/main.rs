use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn evaluate_ttt(input: &str) -> u32 {
    let mut points = match input.chars().nth(2).unwrap() {
        'X' => 1, // A rock
        'Y' => 2, // B paper
        'Z' => 3, // C scissors
        _ => panic!("second players character was not X, Y or Z"),
    };

    points += match input {
        "A Y" | "B Z" | "C X" => 6,
        "A X" | "B Y" | "C Z" => 3,
        "A Z" | "B X" | "C Y" => 0,
        _ => panic!("invalid string"),
    };
    points
}

fn process(func: &dyn Fn(&str) -> u32, input: &str) -> u32 {
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let points_combined: u32 = lines.map(|l| func(&l.unwrap())).sum();

    println!("{points_combined} points all together.");
    points_combined
}

fn evaluate_from_result(input: &str) -> u32 {
    let mut points = match input.chars().nth(2).unwrap() {
        'X' => 0, // loss
        'Y' => 3, // draw
        'Z' => 6, // win
        _ => panic!("second players character was not X, Y or Z"),
    };

    points += match input {
        "A Y" | "B X" | "C Z" => 1, // all cases I choose a stone
        "A Z" | "B Y" | "C X" => 2, // all cases I choose a paper
        "A X" | "B Z" | "C Y" => 3, // all cases I choose scissors
        _ => panic!("invalid string"),
    };
    points
}

fn main() {
    let input = "day02/input.txt";
    process(&evaluate_ttt, input);

    process(&evaluate_from_result, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        assert_eq!(process(&evaluate_ttt, "input_sample.txt"), 15)
    }
    #[test]
    fn test_sample_b() {
        assert_eq!(process(&evaluate_from_result, "input_sample.txt"), 12)
    }
}
