use aoc2025::common::read_input;
use regex::Regex;

const DAY: u32 = 1;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<i64> {
    let re = Regex::new(r"^([LR])(\d+)$").unwrap();
    input
        .iter()
        .map(|s| {
            let caps = re.captures(s).unwrap();
            let dir = &caps[1];
            let dist = &caps[2].parse::<i64>().unwrap();
            if dir == "L" { -dist } else { *dist }
        })
        .collect()
}

// Parse sample input
fn _sample_input() -> Vec<i64> {
    let data = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
    .split('\n')
    .map(|s| s.to_string())
    .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<i64> {
    process_input(read_input(DAY))
}

fn part1(input: &Vec<i64>) -> i64 {
    let mut pos = 50;
    let mut zeros = 0;
    for change in input {
        pos += *change % 100 + 100;
        pos = pos % 100;
        if pos == 0 {
            zeros += 1;
        }
        if pos < 0 || pos >= 100 {
            println!("Position out of bounds: {pos}");
        }
    }
    zeros
}

fn part2(input: &Vec<i64>) -> i64 {
    let mut pos = 50;
    let mut zeros = 0;
    for change in input {
        let start = pos;
        pos += *change % 100 + 100;
        pos = pos % 100;

        // Calculate when we pass zero
        let full_cycles = (*change / 100).abs();
        zeros += full_cycles;
        if start == 0 {
            // Already on zero
            continue;
        } else if pos == 0 {
            // Turn ends at zero
            if start != pos {
                //println!("Landing on zero from {start}");
                zeros += 1;
            }
        } else if *change > 0 {
            // Clockwise
            if start > pos {
                //println!("Passed zero between {start} and {pos}");
                zeros += 1;
            }
        } else {
            // Counter-clockwise
            if start < pos {
                //println!("Passed zero between {start} and {pos} CC");
                zeros += 1;
            }
        }

        //println!("Change: {change}, Position: {pos}, Full cycles: {full_cycles}, Zeros: {zeros}");
        if pos < 0 || pos >= 100 {
            println!("Position out of bounds: {pos}");
        }
    }
    zeros
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
        assert_eq!(6, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(989, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(5941, part2(&input));
    }
}

// 5949 too high
