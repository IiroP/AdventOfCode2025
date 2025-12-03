use aoc2025::common::read_input;

const DAY: u32 = 3;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<String> {
    input
}

// Parse sample input
fn _sample_input() -> Vec<String> {
    let data = "987654321111111
811111111111119
234234234234278
818181911112111"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<String> {
    process_input(read_input(DAY))
}

// Find the next digit for max number
fn find_next(bank: &str, skip: usize, remaining: usize) -> (char, usize) {
    let max = bank
        .chars()
        .skip(skip)
        .take(bank.len() - remaining - skip)
        .max()
        .unwrap();
    let index = bank.chars().skip(skip).position(|c| c == max).unwrap() + skip;
    (max, index)
}

fn part1(input: &Vec<String>) -> i64 {
    input
        .iter()
        .map(|bank| {
            // Find first maximum digit, and then the next maximum digit after it
            let max = bank.chars().take(bank.len() - 1).max().unwrap();
            let first_index = bank.chars().position(|c| c == max).unwrap();
            let second = bank.chars().skip(first_index + 1).max().unwrap();
            max.to_digit(10).unwrap() as i64 * 10 + second.to_digit(10).unwrap() as i64
        })
        .sum()
}

fn part2(input: &Vec<String>) -> i64 {
    input
        .iter()
        .map(|bank| {
            (0..12)
                .rev()
                .fold(("".to_string(), 0), |(acc, skip), r| {
                    let (next, index) = find_next(bank, skip, r);
                    (format!("{acc}{next}"), index + 1)
                })
                .0
                .parse::<i64>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(357, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(3121910778619, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(17330, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(171518260283767, part2(&input));
    }
}
