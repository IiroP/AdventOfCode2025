use aoc2025::common::read_input;

const DAY: u32 = 5;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> (Vec<(i64, i64)>, Vec<i64>) {
    let range_pattern = regex::Regex::new(r"^(\d+)-(\d+)$").unwrap();
    let number_pattern = regex::Regex::new(r"^(\d+)$").unwrap();
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut numbers: Vec<i64> = Vec::new();
    input.iter().for_each(|row| {
        if let Some(caps) = range_pattern.captures(row) {
            let start: i64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let end: i64 = caps.get(2).unwrap().as_str().parse().unwrap();
            ranges.push((start, end));
        } else if let Some(caps) = number_pattern.captures(row) {
            let number: i64 = caps.get(1).unwrap().as_str().parse().unwrap();
            numbers.push(number);
        }
    });
    (ranges, numbers)
}

// Parse sample input
fn _sample_input() -> (Vec<(i64, i64)>, Vec<i64>) {
    let data = "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
    .split('\n')
    .map(|s| s.to_string())
    .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> (Vec<(i64, i64)>, Vec<i64>) {
    process_input(read_input(DAY))
}

fn part1(input: &(Vec<(i64, i64)>, Vec<i64>)) -> i64 {
    let (ranges, numbers) = input;
    numbers
        .iter()
        .filter(|n| {
            ranges
                .iter()
                .any(|(start, end)| **n >= *start && **n <= *end)
        })
        .count() as i64
}

fn part2(input: &(Vec<(i64, i64)>, Vec<i64>)) -> i64 {
    let (ranges, _) = input;

    // Sort ranges and merge overlapping ones
    let mut merged_ranges: Vec<(i64, i64)> = Vec::new();
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    for range in sorted_ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if range.0 <= last.1 {
                last.1 = last.1.max(range.1);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    // Calculate fresh ingredients
    merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(3, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(14, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(761, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(345755049374932, part2(&input));
    }
}

// 321734886414250 too low
