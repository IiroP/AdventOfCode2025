use std::collections::{HashMap, HashSet};

use aoc2025::common::read_input;

const DAY: u32 = 7;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<Vec<char>> {
    input.iter().map(|row| row.chars().collect()).collect()
}

// Parse sample input
fn _sample_input() -> Vec<Vec<char>> {
    let data = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<Vec<char>> {
    process_input(read_input(DAY))
}

fn part1(input: &Vec<Vec<char>>) -> i64 {
    let start = input
        .first()
        .unwrap()
        .iter()
        .position(|&c| c == 'S')
        .unwrap();
    let mut processed = HashSet::new();
    let mut remaining = vec![(start, 0 as usize)];
    let mut splits = HashSet::new();
    let x_len = input[0].len();
    while !remaining.is_empty() {
        let pos = remaining.pop().unwrap();
        if processed.contains(&pos) {
            continue;
        }
        processed.insert(pos);
        let mut next = (pos.0, pos.1 + 1);
        while next.1 < input.len() && input[next.1][next.0] == '.' {
            // Continue beam
            next.1 += 1;
        }
        if next.1 < input.len() {
            // Reached a splitter
            splits.insert(next);
            if next.0 > 0 {
                remaining.push((next.0 - 1, next.1));
            }
            if next.0 + 1 < x_len {
                remaining.push((next.0 + 1, next.1));
            }
        }
    }
    splits.len() as i64
}

fn part2(input: &Vec<Vec<char>>) -> i64 {
    let start = input
        .first()
        .unwrap()
        .iter()
        .position(|&c| c == 'S')
        .unwrap();
    let mut processed = HashSet::new();
    let mut remaining = vec![((start, 0 as usize), (start, 0 as usize))];
    let mut edges = HashSet::new();
    let mut ends = HashSet::new();
    let x_len = input[0].len();
    while !remaining.is_empty() {
        let (pos, prev) = remaining.pop().unwrap();
        if processed.contains(&(pos, prev)) {
            continue;
        }
        processed.insert((pos, prev));
        let mut next = (pos.0, pos.1 + 1);
        while next.1 < input.len() && input[next.1][next.0] == '.' {
            // Continue beam
            next.1 += 1;
        }
        edges.insert((prev, next));
        if next.1 < input.len() {
            // Reached a splitter
            if next.0 > 0 {
                remaining.push(((next.0 - 1, next.1), next));
            }
            if next.0 + 1 < x_len {
                remaining.push(((next.0 + 1, next.1), next));
            }
        } else {
            ends.insert(next);
        }
    }

    let mut steps = HashMap::new();

    fn recursive(
        current: (usize, usize),
        ends: &HashSet<(usize, usize)>,
        edges: &HashSet<((usize, usize), (usize, usize))>,
        steps: &mut HashMap<(usize, usize), i64>,
    ) -> i64 {
        if ends.contains(&current) {
            return 1;
        }
        if let Some(&s) = steps.get(&current) {
            return s;
        }
        let result = edges
            .iter()
            .filter_map(|(from, to)| {
                if from == &current {
                    Some(recursive(*to, ends, edges, steps))
                } else {
                    None
                }
            })
            .sum();
        //println!("At {:?}, found {} paths", current, result);
        steps.insert(current, result);
        result
    }

    recursive((start, 0), &ends, &edges, &mut steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(21, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(40, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(1539, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(6479180385864, part2(&input));
    }
}
