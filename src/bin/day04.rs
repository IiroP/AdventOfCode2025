use std::collections::HashMap;

use aoc2025::common::read_input;

const DAY: u32 = 4;

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
    let data = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<String> {
    process_input(read_input(DAY))
}

fn neighbors(x: usize, y: usize, map: &Vec<String>, removed: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut result: Vec<(i64, i64)> = Vec::new();
    let height = map.len() as i64;
    let width = map[0].len() as i64;
    let x = x as i64;
    let y = y as i64;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < width && ny >= 0 && ny < height && !removed.contains(&(nx, ny)) {
                let symbol = map
                    .get(ny as usize)
                    .unwrap()
                    .chars()
                    .nth(nx as usize)
                    .unwrap();
                if symbol == '@' {
                    result.push((nx, ny));
                }
            }
        }
    }
    //println!("Neighbors of ({},{}) : {:?}", x, y, result);
    result
}

fn part1(input: &Vec<String>) -> i64 {
    let removed = Vec::new();
    (0..input.len())
        .map(|y| {
            (0..input[0].len())
                .filter_map(|x| {
                    let symbol = input[y].chars().nth(x).unwrap();
                    if symbol == '@' {
                        Some(neighbors(x, y, input, &removed).len())
                    } else {
                        None
                    }
                })
                .filter(|n| *n < 4)
                .count() as i64
        })
        .sum()
}

// A bit slow but works (~8.5s on debug build, ~0.5s on release)
fn part2(input: &Vec<String>) -> i64 {
    let mut rolls = HashMap::new();
    let mut removed = 0;
    // Initialize neighbor map
    (0..input.len()).for_each(|y| {
        (0..input[0].len()).for_each(|x| {
            let symbol = input[y].chars().nth(x).unwrap();
            if symbol == '@' {
                let n = neighbors(x, y, input, &Vec::new());
                rolls.insert((x as i64, y as i64), n);
            }
        });
    });

    loop {
        // Find all with less than 4 neighbors
        let to_remove = rolls
            .iter()
            .filter_map(|(pair, n)| if n.len() < 4 { Some(*pair) } else { None })
            .collect::<Vec<_>>();
        if to_remove.is_empty() {
            break;
        }
        removed += to_remove.len() as i64;

        // Remove them from all neighbor lists
        rolls.values_mut().for_each(|v| {
            v.retain(|p| !to_remove.contains(p));
        });

        // Remove them from rolls
        rolls.retain(|k, _| !to_remove.contains(k));
    }
    removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(13, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(43, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(1367, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(9144, part2(&input));
    }
}
