use std::{fs::File, io::{self, BufRead, BufReader}};

use once_cell::sync::Lazy;

fn main() -> io::Result<()> {
    run_calibration("example 1", "day1example1.txt", SIMPLE_DIGITS.as_ref())?;
    run_calibration("example 2", "day1example2.txt", ALL_DIGITS.as_ref())?;
    run_calibration("part 1", "day1input.txt", SIMPLE_DIGITS.as_ref())?;
    run_calibration("part 2", "day1input.txt", ALL_DIGITS.as_ref())
}

fn run_calibration(label: &str, file_name: &str, digits: &[String]) -> io::Result<()> {
    let file = File::open("data/".to_string() + file_name)?;
    let lines = BufReader::new(file).lines();
    let result = lines
        .filter_map(|l| l.ok().and_then(|l| calibrate_line(l.as_str(), digits)))
        .sum::<usize>();
    println!("Day 1 {} solution: {}", label, result);
    Ok(())
}

fn digit_to_int(digit: &str) -> Option<usize> {
    if SIMPLE_DIGITS.contains(&digit.to_string()) {
        digit.parse::<usize>().ok()
    } else {
        WORD_DIGITS.iter().position(|x| x == digit).map(|p| p + 1)
    }
}

fn calibrate_line(line: &str, digits: &[String]) -> Option<usize> {
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
    (0 .. 10).map(|x| x.to_string()).collect() 
});


static WORD_DIGITS: Lazy<Vec<String>> = Lazy::new(|| { 
    ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
    .iter()
    .map(|x| x.to_string())
    .collect() 
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
fn test_calibrate_line() { 
    assert_eq!(calibrate_line("one two three four five six seven eight nine", &ALL_DIGITS), Some(19));
}
