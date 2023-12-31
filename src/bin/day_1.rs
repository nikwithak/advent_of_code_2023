use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Peekable,
    str::Chars,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_sample_input() {
        let result = part_1("inputs/day_1_sample__part_1.txt");
        assert_eq!(result, 142);
    }
    #[test]
    fn part_1_final_input() {
        let result = part_1("inputs/day_1.txt");
        assert_eq!(result, 55108);
    }
    #[test]
    fn part_2_final_input() {
        let result = part_2("inputs/day_1.txt");
        assert_eq!(result, 56324);
    }

    #[test]
    fn part_2_sample_input() {
        let result = part_2("inputs/day_1_sample__part_2.txt");
        assert_eq!(result, 281);
    }
}

fn main() {
    part_1("inputs/day_1_sample__part_1.txt");
    part_1("inputs/day_1.txt");
    part_2("inputs/day_1_sample__part_2.txt");
    part_2("inputs/day_1.txt");
}

fn part_1(filename: &str) -> u64 {
    fn get_calibration_value(value: &str) -> u64 {
        let mut chars = value.chars();
        let first = chars.find(|c| c.is_digit(10)).expect("No first digit found");
        let second = chars.rev().find(|c| c.is_digit(10)).unwrap_or(first);

        let mut value_str = String::new();
        value_str.push(first);
        value_str.push(second);

        return value_str.parse().unwrap();
    }

    let mut calibration_values_sum: u64 = 0;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let val = get_calibration_value(&line.unwrap());
        calibration_values_sum += val;
    }
    println!("Sum of calibration values: {}", calibration_values_sum);
    calibration_values_sum
}

fn part_2(filename: &str) -> u64 {
    fn get_calibration_value(value: &str) -> u64 {
        // First get an iter of the chars, to go through. We can do this with one pass.
        let chars = value.chars();

        let digits = vec![
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        let mut partials: Vec<(Peekable<Chars>, u8)> = Vec::new();
        let mut matched_digits = Vec::new();

        for char in chars {
            partials.append(
                &mut digits
                    .iter()
                    .map(|(needle, digit)| (needle.chars().peekable(), *digit))
                    .collect(),
            );

            let mut to_remove = Vec::new();
            for (i, partial) in partials.iter_mut().enumerate() {
                if partial.0.next().map_or(false, |c| c.eq(&char)) {
                    if partial.0.peek().is_none() {
                        matched_digits.push(partial.1);
                        to_remove.push(i);
                    }
                } else {
                    to_remove.push(i);
                }
            }
            for i in to_remove.iter().rev() {
                let _ = partials.swap_remove(*i);
            }
        }

        let value_str =
            *matched_digits.first().unwrap() as u64 * 10 + *matched_digits.last().unwrap() as u64;

        value_str
    }

    let mut calibration_values_sum: u64 = 0;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let val = get_calibration_value(&line.unwrap());
        calibration_values_sum += val;
    }
    println!("Sum of calibration values: {}", calibration_values_sum);
    calibration_values_sum
}
