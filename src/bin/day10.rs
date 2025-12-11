use aoc2025::common::read_input;
use itertools::Itertools;
use regex::Regex;
use z3::{Optimize, ast::Int};

const DAY: u32 = 10;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)> {
    let pattern = Regex::new(r"\[([.#]+)\] (.+) \{(.+)\}").unwrap();
    let button_pattern = Regex::new(r"\(([\d,]+)\)").unwrap();
    input
        .iter()
        .map(|row| {
            let caps = pattern.captures(row).unwrap();
            let grid = &caps[1]
                .chars()
                .map(|c| {
                    if c == '#' {
                        return true;
                    }
                    false
                })
                .collect::<Vec<bool>>();
            let buttons_str = &caps[2];
            let joltages_str = &caps[3];
            let buttons: Vec<Vec<usize>> = button_pattern
                .captures_iter(buttons_str)
                .map(|bcaps| {
                    bcaps[1]
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();
            let joltages: Vec<usize> = joltages_str
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            (grid.clone(), buttons, joltages)
        })
        .collect()
}

// Parse sample input
fn _sample_input() -> Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)> {
    let data = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)> {
    process_input(read_input(DAY))
}

fn fewest_presses(row: &(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)) -> i64 {
    let (grid, buttons, _) = row;
    (0..buttons.len())
        .find(|k| {
            buttons.iter().combinations(*k).any(|a| {
                let presses = a.iter().flat_map(|&a| a.clone()).collect::<Vec<usize>>();
                let valid = (0..grid.len()).all(|i| {
                    if grid[i] {
                        presses.iter().filter(|&&p| p == i).count() % 2 == 1
                    } else {
                        presses.iter().filter(|&&p| p == i).count() % 2 == 0
                    }
                });
                valid
            })
        })
        .unwrap_or(0) as i64
}

fn part1(input: &Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)>) -> i64 {
    input.iter().map(|row| fewest_presses(row)).sum()
}

// Solve part2 for single row using Z3
fn solve_row(buttons: &Vec<Vec<usize>>, joltages: &Vec<usize>) -> i64 {
    let presses = (0..buttons.len())
        .map(|k| {
            let name = format!("presses_{}", k);
            Int::fresh_const(&name)
        })
        .collect::<Vec<_>>();
    let solver = Optimize::new();
    let total_presses = Int::add(presses.as_slice());
    solver.minimize(&total_presses);

    // Button press constraints
    for press in &presses {
        solver.assert(&press.ge(0));
    }

    // Joltage constraints
    for (i, &joltage) in joltages.iter().enumerate() {
        let relevant_buttons = buttons
            .iter()
            .enumerate()
            .filter_map(|(j, b)| if b.contains(&i) { Some(j) } else { None })
            .map(|i| &presses[i])
            .collect::<Vec<_>>();
        let sum_expr = Int::add(relevant_buttons.as_slice());
        solver.assert(&sum_expr.eq(joltage as i64));
    }
    if let z3::SatResult::Sat = solver.check(&[]) {
        let model = solver.get_model().unwrap();
        presses
            .iter()
            .map(|a| model.eval(a, true).unwrap().as_i64().unwrap())
            .sum()
    } else {
        panic!("No solution found");
    }
}

fn part2(input: &Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)>) -> i64 {
    input
        .iter()
        .map(|row| solve_row(&row.1, &row.2))
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(7, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(33, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(444, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(16513, part2(&input));
    }
}
