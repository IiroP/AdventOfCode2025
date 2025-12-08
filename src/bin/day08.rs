use std::collections::HashMap;

use aoc2025::common::read_input;
use itertools::Itertools;
use regex::Regex;

const DAY: u32 = 8;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input, false);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<(i64, i64, i64)> {
    let pattern = Regex::new(r"^(\d+),(\d+),(\d+)$").unwrap();
    input
        .iter()
        .map(|row| {
            let caps = pattern.captures(row).unwrap();
            (
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
            )
        })
        .collect()
}

// Parse sample input
fn _sample_input() -> Vec<(i64, i64, i64)> {
    let data = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<(i64, i64, i64)> {
    process_input(read_input(DAY))
}

// Calculate all distances between nodes
fn distances(nodes: &Vec<(i64, i64, i64)>) -> Vec<(usize, usize, i64)> {
    let mut dists = Vec::new();
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            let dx = nodes[i].0 - nodes[j].0;
            let dy = nodes[i].1 - nodes[j].1;
            let dz = nodes[i].2 - nodes[j].2;
            let dist = (dx * dx + dy * dy + dz * dz).isqrt();
            dists.push((i, j, dist))
        }
    }
    dists.sort_by(|a, b| a.2.cmp(&b.2));
    dists
}

fn part1(input: &Vec<(i64, i64, i64)>, test: bool) -> i64 {
    let dists = distances(input);
    let mut circuits = HashMap::new();
    let target_connections = if test { 10 } else { 1000 };
    dists
        .iter()
        .take(target_connections)
        .for_each(|(from, to, _)| {
            // Check if both nodes are already in the same circuit
            if circuits.contains_key(from) && circuits.get(from) == circuits.get(to) {
                return;
            }

            // Merge circuits if needed
            let from_c = circuits.get(from);
            let to_c = circuits.get(to);
            if from_c.is_some() && to_c.is_none() {
                circuits.insert(*to, *from_c.unwrap());
            } else if from_c.is_none() && to_c.is_some() {
                circuits.insert(*from, *to_c.unwrap());
            } else if from_c.is_some() && to_c.is_some() {
                let old_c = *to_c.unwrap();
                let new_c = *from_c.unwrap();
                circuits.iter_mut().for_each(|(_, v)| {
                    if *v == old_c {
                        *v = new_c;
                    }
                });
            } else {
                // Create new circuit
                circuits.insert(*from, *from);
                circuits.insert(*to, *from);
            }
        });
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for &v in circuits.values() {
        *counts.entry(v).or_insert(0) += 1;
    }
    let values = counts
        .into_iter()
        .map(|(_, c)| c)
        .sorted_by(|a, b| b.cmp(a));
    values.take(3).product::<usize>() as i64
}

fn part2(input: &Vec<(i64, i64, i64)>) -> i64 {
    let dists = distances(input);
    let mut circuits = HashMap::new();
    // Keep track of the last necessary connection
    let mut last_connected = (0, 0);
    dists.iter().for_each(|(from, to, _)| {
        // Check if both nodes are already in the same circuit
        if circuits.contains_key(from) && circuits.get(from) == circuits.get(to) {
            return;
        }
        last_connected = (*from, *to);

        // Merge circuits if needed
        let from_c = circuits.get(from);
        let to_c = circuits.get(to);
        if from_c.is_some() && to_c.is_none() {
            circuits.insert(*to, *from_c.unwrap());
        } else if from_c.is_none() && to_c.is_some() {
            circuits.insert(*from, *to_c.unwrap());
        } else if from_c.is_some() && to_c.is_some() {
            let old_c = *to_c.unwrap();
            let new_c = *from_c.unwrap();
            circuits.iter_mut().for_each(|(_, v)| {
                if *v == old_c {
                    *v = new_c;
                }
            });
        } else {
            // Create new circuit
            circuits.insert(*from, *from);
            circuits.insert(*to, *from);
        }
    });
    input[last_connected.0].0 as i64 * input[last_connected.1].0 as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(40, part1(&input, true));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(25272, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(129564, part1(&input, false));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(42047840, part2(&input));
    }
}
