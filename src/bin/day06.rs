use aoc2025::common::read_input;

const DAY: u32 = 6;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> (Vec<String>, Vec<Vec<i64>>, Vec<Vec<char>>) {
    let symbols: Vec<String> = input
        .last()
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect();
    let data = input
        .iter()
        .take(input.len() - 1)
        .map(|row| {
            row.split(" ")
                .filter(|s| !s.is_empty())
                .map(|v| v.parse::<i64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i64>>>();
    let data2 = input
        .iter()
        .take(input.len() - 1)
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    (symbols, data, data2)
}

// Parse sample input
fn _sample_input() -> (Vec<String>, Vec<Vec<i64>>, Vec<Vec<char>>) {
    let data = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> (Vec<String>, Vec<Vec<i64>>, Vec<Vec<char>>) {
    process_input(read_input(DAY))
}

fn part1(input: &(Vec<String>, Vec<Vec<i64>>, Vec<Vec<char>>)) -> i64 {
    let (symbols, data, _) = input;
    let mut results: Vec<i64> = data.first().unwrap().clone();
    for row in data.iter().skip(1) {
        for (i, symbol) in symbols.iter().enumerate() {
            match symbol.as_str() {
                "*" => results[i] *= row[i],
                "+" => results[i] += row[i],
                _ => panic!("Unknown symbol {}", symbol),
            }
        }
    }
    results.iter().sum()
}

fn part2(input: &(Vec<String>, Vec<Vec<i64>>, Vec<Vec<char>>)) -> i64 {
    let (symbols, _, data) = input;
    let y_len = data.len();
    let x_len = data.first().unwrap().len();
    let mut numbers: Vec<Vec<i64>> = vec![Vec::new(); symbols.len()];
    let mut i = 0;
    for x in 0..x_len {
        let mut current = Vec::new();
        for y in 0..y_len {
            current.push(data[y][x]);
        }
        let joined: String = current.iter().collect();
        let number = joined.trim().parse::<i64>().ok();
        if let Some(n) = number {
            numbers[i].push(n);
        } else {
            i += 1;
        }
    }

    symbols
        .iter()
        .enumerate()
        .map(|(i, symbol)| match symbol.as_str() {
            "*" => numbers[i].iter().copied().product::<i64>(),
            "+" => numbers[i].iter().copied().sum::<i64>(),
            _ => panic!("Unknown symbol {}", symbol),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(4277556, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(3263827, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(5227286044585, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(10227753257799, part2(&input));
    }
}
