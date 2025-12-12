use aoc2025::common::read_input;
use regex::Regex;

const DAY: u32 = 12;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> (Vec<Vec<Vec<char>>>, Vec<(usize, usize, Vec<usize>)>) {
    let area_pattern = Regex::new(r"^(\d+)x(\d+): ([\d ]+)$").unwrap();
    let mut shapes = vec![];
    let mut areas = vec![];
    let mut current_shape = vec![];
    input.iter().for_each(|row| {
        if let Some(caps) = area_pattern.captures(row) {
            let width: usize = caps[1].parse().unwrap();
            let height: usize = caps[2].parse().unwrap();
            let shape_indices: Vec<usize> =
                caps[3].split(' ').map(|s| s.parse().unwrap()).collect();
            areas.push((width, height, shape_indices));
        } else if row.is_empty() {
            shapes.push(current_shape.clone());
            current_shape.clear();
        } else if !row.contains(":") {
            // Ignore headers
            current_shape.push(row.chars().collect::<Vec<char>>());
        }
    });
    (shapes, areas)
}

// Parse sample input
fn _sample_input() -> (Vec<Vec<Vec<char>>>, Vec<(usize, usize, Vec<usize>)>) {
    let data = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> (Vec<Vec<Vec<char>>>, Vec<(usize, usize, Vec<usize>)>) {
    process_input(read_input(DAY))
}

fn part1(input: &(Vec<Vec<Vec<char>>>, Vec<(usize, usize, Vec<usize>)>)) -> i64 {
    let (_, areas) = input;
    areas
        .iter()
        .filter(|area| {
            let (width, height, shape_indices) = area;
            let remaining = shape_indices
                .iter()
                .enumerate()
                .flat_map(|(i, val)| vec![i; *val])
                .collect::<Vec<_>>();
            if remaining.len() * 7 > width * height {
                return false;
            }

            let rows = height / 3;
            let cols = width / 3;
            let grid_total = rows * cols;
            if remaining.len() <= grid_total {
                // Easy fit
                true
            } else {
                // More precise
                panic!("Too difficult input :)")
            }
        })
        .count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(479, part1(&input));
    }
}
