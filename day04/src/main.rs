#[derive(Debug)]
struct Range {
    start: u32,
    stop: u32,
}

impl Range {
    fn new(s: &str) -> Self {
        let ae: Vec<&str> = s.split('-').collect();
        let ae: Vec<u32> = ae
            .iter()
            .map(|s| s.parse().expect("not a number?"))
            .collect();
        Range {
            start: ae[0],
            stop: ae[1],
        }
    }

    fn contains(&self, other: &Range, partial: bool) -> bool {
        if partial {
            other.start <= self.start && self.start <= other.stop
                || other.start <= self.stop && self.stop <= other.stop
        } else {
            self.start <= other.start && self.stop >= other.stop
        }
    }
}

fn calc(input: &str, partial: bool) -> bool {
    if input.is_empty() {
        return false;
    }
    let (r1, r2) = input.split_once(',').expect("no comma in line");
    let range1 = Range::new(r1);
    let range2 = Range::new(r2);
    range1.contains(&range2, partial) || range2.contains(&range1, partial)
}

fn process(input: &[&str], partial: bool) -> usize {
    let overlaps: usize = input
        .iter()
        .map(|l| calc(*l, partial))
        .filter(|b| *b)
        .count();

    println!("{overlaps} overlaps all together.");
    overlaps
}

fn main() {
    let lines_raw: &str = include_str!("../input.txt");
    let lines = lines_raw.split('\n').collect::<Vec<&str>>();
    process(&lines, false);
    process(&lines, true);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let lines_raw: &str = include_str!("../input_sample.txt");
        let lines = lines_raw.split('\n').collect::<Vec<&str>>();
        assert_eq!(process(&lines, false), 2)
    }
    #[test]
    fn test_sample_partial() {
        let lines_raw: &str = include_str!("../input_sample.txt");
        let lines = lines_raw.split('\n').collect::<Vec<&str>>();
        assert_eq!(process(&lines, true), 4)
    }
}
