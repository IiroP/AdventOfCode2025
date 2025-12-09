use aoc2025::common::read_input;
use geo::{Covers, LineString, Polygon, Rect, coord};

const DAY: u32 = 9;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<(i64, i64)> {
    input
        .iter()
        .map(|row| {
            let splitted: Vec<&str> = row.split(",").collect();
            let x = splitted[0].parse().unwrap();
            let y = splitted[1].parse().unwrap();
            (x, y)
        })
        .collect()
}

// Parse sample input
fn _sample_input() -> Vec<(i64, i64)> {
    let data = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
    .split('\n')
    .map(|s| s.to_string())
    .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<(i64, i64)> {
    process_input(read_input(DAY))
}

fn part1(input: &Vec<(i64, i64)>) -> i64 {
    (0..input.len())
        .map(|i| {
            let (x1, y1) = input[i];
            (i + 1..input.len())
                .map(|j| {
                    let (x2, y2) = input[j];
                    let area = ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1);
                    area
                })
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0)
}

fn validate_area(polygon: &Polygon<f64>, corner1: (i64, i64), corner2: (i64, i64)) -> bool {
    let (x0, y0) = corner1;
    let (x1, y1) = corner2;
    //println!("Validating area between {:?} and {:?}", corner1, corner2);
    let rect: Rect<f64> = Rect::new(
        coord! { x: x0 as f64, y: y0 as f64 },
        coord! { x: x1 as f64, y: y1 as f64 },
    );

    // Check all corners first for a quick reject
    if !polygon.covers(&coord! {x: x0 as f64, y: y0 as f64})
        || !polygon.covers(&coord! {x: x0 as f64, y: y1 as f64})
        || !polygon.covers(&coord! {x: x1 as f64, y: y0 as f64})
        || !polygon.covers(&coord! {x: x1 as f64, y: y1 as f64})
    {
        return false;
    }

    // Then check the full rectangle
    polygon.covers(&rect)
}

// A bit slow (~6s on debug, ~instant on release)
fn part2(input: &Vec<(i64, i64)>) -> i64 {
    let mut points = input
        .iter()
        .map(|(a, b)| coord! { x: *a as f64, y: *b as f64 })
        .collect::<Vec<_>>();
    points.insert(
        input.len(),
        coord! { x: input[0].0 as f64, y: input[0].1 as f64 },
    );
    let polygon = Polygon::new(LineString::from(points), vec![]);
    let mut current_max = 0;
    (0..input.len())
        .map(|i| {
            let (x1, y1) = input[i];
            let result = (i + 1..input.len())
                .filter_map(|j| {
                    let (x2, y2) = input[j];
                    let area = ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1);
                    if area > current_max && validate_area(&polygon, (x1, y1), (x2, y2)) {
                        Some(area)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(0);
            if result > current_max {
                current_max = result;
            }
            result
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(50, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(24, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(4748826374, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(1554370486, part2(&input));
    }
}
