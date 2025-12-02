use aoc2025::common::read_input;
use itertools::Itertools;
use regex::Regex;

const DAY: u32 = 2;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<(i64, i64)> {
    let pattern = Regex::new(r"(\d+)-(\d+)").unwrap();
    input
        .first()
        .unwrap()
        .split(",")
        .map(|rule| {
            let captures = pattern.captures(rule).unwrap();
            let start = &captures[1].parse::<i64>().unwrap();
            let end = &captures[2].parse::<i64>().unwrap();
            (*start, *end)
        })
        .collect()
}

// Parse sample input
fn _sample_input() -> Vec<(i64, i64)> {
    let data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<(i64, i64)> {
    process_input(read_input(DAY))
}

// Returns numbers that have exactly two repeating halves
fn check_2_repeats(start: i64, end: i64) -> Vec<i64> {
    (start..=end)
        .filter(|i| {
            let s = i.to_string();
            if s.len() % 2 == 0 {
                let (left, right) = s.split_at(s.len() / 2);
                if left == right {
                    return true;
                }
            }
            false
        })
        .collect()
}

// Returns numbers that consist of pattern repeating 2+ times
fn check_n_patterns(start: i64, end: i64) -> Vec<i64> {
    //println!("Generating IDs for range {start}-{end}");
    let start_str = start.to_string();
    let end_str = end.to_string();
    let max_pattern = (end_str.len() / 2).try_into().unwrap();
    (1..10u64.pow(max_pattern))
        .filter_map(|p| {
            let pattern = p.to_string();
            let repeats = end_str.len() / pattern.len();
            let res = pattern.repeat(repeats);
            let val = res.parse::<i64>().unwrap();
            if val >= start && val <= end && repeats > 1 {
                //println!("1: {val} (pattern {pattern})");
                return Some(val);
            }

            let repeats2 = start_str.len() / pattern.len();
            let res2 = pattern.repeat(repeats2);
            let val2 = res2.parse::<i64>().unwrap();
            if val2 >= start && val2 <= end && repeats2 > 1 {
                //println!("2: {val2} (pattern {pattern})");
                return Some(val2);
            }
            None
        })
        .collect()
}

fn part1(input: &Vec<(i64, i64)>) -> i64 {
    input
        .iter()
        .map(|r| check_2_repeats(r.0, r.1).iter().sum::<i64>())
        .sum()
}

fn part2(input: &Vec<(i64, i64)>) -> i64 {
    input
        .iter()
        .flat_map(|r| check_n_patterns(r.0, r.1))
        .unique()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(1227775554, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(4174379265, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(28844599675, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(48778605167, part2(&input));
    }
}
