use std::collections::HashMap;

use aoc2025::common::read_input;

const DAY: u32 = 11;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> (Vec<String>, Vec<(String, String)>) {
    let mut nodes = vec![];
    let mut edges = vec![];
    input.iter().for_each(|row| {
        let parts = row.split(": ").collect::<Vec<&str>>();
        let name = parts[0].to_string();
        let new_edges = parts[1]
            .split(" ")
            .map(|s| (name.clone(), s.to_string()))
            .collect::<Vec<(String, String)>>();
        nodes.push(name);
        edges.extend(new_edges);
    });
    (nodes, edges)
}

// Parse sample input
fn _sample_input() -> (Vec<String>, Vec<(String, String)>) {
    let data = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

fn _sample_input2() -> (Vec<String>, Vec<(String, String)>) {
    let data = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> (Vec<String>, Vec<(String, String)>) {
    process_input(read_input(DAY))
}

fn part1(input: &(Vec<String>, Vec<(String, String)>)) -> i64 {
    let (_, edges) = input;
    let mut steps: HashMap<String, i64> = HashMap::new();

    fn recursive(
        current: String,
        edges: &Vec<(String, String)>,
        steps: &mut HashMap<String, i64>,
    ) -> i64 {
        if current == "out" {
            return 1;
        }
        if let Some(&s) = steps.get(&current) {
            return s;
        }
        let result = edges
            .iter()
            .filter_map(|(from, to)| {
                if from == &current {
                    Some(recursive(to.clone(), &edges, steps))
                } else {
                    None
                }
            })
            .sum();
        //println!("At {:?}, found {} paths", current, result);
        steps.insert(current, result);
        result
    }

    recursive("you".to_string(), edges, &mut steps)
}

fn part2(input: &(Vec<String>, Vec<(String, String)>)) -> i64 {
    let (_, edges) = input;
    let mut steps: HashMap<String, (i64, i64, i64, i64)> = HashMap::new();
    // (total paths, paths with dac, paths with fft, paths with both)

    fn recursive(
        current: String,
        edges: &Vec<(String, String)>,
        steps: &mut HashMap<String, (i64, i64, i64, i64)>,
    ) -> (i64, i64, i64, i64) {
        if current == "out" {
            return (1, 0, 0, 0);
        }
        if let Some(&s) = steps.get(&current) {
            return s;
        }
        let result = edges
            .iter()
            .filter_map(|(from, to)| {
                if from == &current {
                    Some(recursive(to.clone(), &edges, steps))
                } else {
                    None
                }
            })
            .reduce(|a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3))
            .map(|(total, dac, fft, both)| {
                if current == "dac" {
                    (total, total, fft, fft)
                } else if current == "fft" {
                    (total, dac, total, dac)
                } else {
                    (total, dac, fft, both)
                }
            })
            .unwrap_or((0, 0, 0, 0));

        //println!("At {:?}, found {:?}", current, result);
        steps.insert(current, result);
        result
    }

    recursive("svr".to_string(), edges, &mut steps).3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(5, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input2();
        assert_eq!(2, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(634, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(377452269415704, part2(&input));
    }
}
