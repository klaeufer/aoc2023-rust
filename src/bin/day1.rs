use std::{fs::File, io::{self, BufRead, BufReader}};

use once_cell::sync::Lazy;

fn main() -> io::Result<()> {
    // trailing question mark returns early if there's an error
    run_calibration("part 1", "day1input.txt", &SIMPLE_DIGITS)?;
    run_calibration("part 2", "day1input.txt", &ALL_DIGITS)
}

fn run_calibration(label: &str, file_name: &str, digits: &[String]) -> io::Result<()> {
    let file = File::open("data/".to_string() + file_name)?;
    let lines = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    println!("Day 1 {} solution: {}", label, calibrate_lines(&lines, digits));
    Ok(())
}

fn digit_to_int(digit: &str) -> Option<usize> {
    // ok() converts potential parse errors into Options
    if SIMPLE_DIGITS.contains(&digit.to_string()) {
        digit.parse::<usize>().ok()
    } else {
        WORD_DIGITS.iter().position(|x| x == digit).map(|p| p + 1)
    }
}

fn calibrate_lines(lines: &[String], digits: &[String]) -> usize {
    lines
        .iter()
        .filter_map(|l| calibrate_line(l, digits))
        .sum::<usize>()
}

fn calibrate_line(line: &str, digits: &[String]) -> Option<usize> {
    // filter_map retains only the successful results of finding a digit on a line
    // the Option monad's and_then (flatmap) handles any early returns
    let first = digits
        .iter()
        .filter_map(|d| line.find(d).map(|i| (d, i)))
        .min_by(|(_, i), (_, i2)| i.cmp(i2))
        .and_then(|(d, _)| digit_to_int(d));
    let last = digits
        .iter()
        .filter_map(|d| line.rfind(d).map(|i| (d, i)))
        .max_by(|(_, i), (_, i2)| i.cmp(i2))
        .and_then(|(d, _i)| digit_to_int(d));
    first.and_then(|f| last.map(|l| f * 10 + l))
}

static SIMPLE_DIGITS: Lazy<Vec<String>> = Lazy::new(|| {
    (0 .. 10).map(|d| d.to_string()).collect::<Vec<_>>() 
});

static WORD_DIGITS: Lazy<Vec<String>> = Lazy::new(|| { 
    ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
    .map(str::to_string)
    .to_vec() 
});

static ALL_DIGITS: Lazy<Vec<String>> = Lazy::new(|| {
    SIMPLE_DIGITS.iter().chain(WORD_DIGITS.iter()).map(|x| x.to_string()).collect()
});


#[cfg(test)]

#[test]
fn test_dti_3() { 
    assert_eq!(digit_to_int("3"), Some(3));
}

#[test]
fn test_dti_three() { 
    assert_eq!(digit_to_int("three"), Some(3));
}

#[test]
fn test_calibrate_line_simple() { 
    assert_eq!(calibrate_line("123456789", &SIMPLE_DIGITS), Some(19));
}

#[test]
fn test_calibrate_line_all() { 
    assert_eq!(calibrate_line("one two three four five six seven eight nine", &ALL_DIGITS), Some(19));
}

#[test]
fn test_part_1() { 
    let example = [
        "1abc2",
        "pqr3stu8vwx",
        "a1b2c3d4e5f",
        "treb7uchet",
    ].map(str::to_string).to_vec();
    assert_eq!(calibrate_lines(&example, &SIMPLE_DIGITS), 142);
}

#[test]
fn test_part_2() { 
    let example = [
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ].map(str::to_string).to_vec();
    assert_eq!(calibrate_lines(&example, &ALL_DIGITS), 281);
}
