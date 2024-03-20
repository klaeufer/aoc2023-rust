use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use log::info;
use anyhow::Result; // simplified error handling
use once_cell::sync::Lazy; // global constants
use regex::Regex;

static NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());

fn main() -> Result<()> {
    env_logger::init();
    let file = File::open("data/day5input.txt")?;
    let mut lines = BufReader::new(file)
        .lines()
        .map(Result::unwrap);
    let (part1, part2) = process(&mut lines);
    println!("Day 5 part 1: {}", part1);
    println!("Day 5 part 2: {}", part2);
    Ok(())
}

fn make_vec(lines: &mut impl Iterator<Item = String>) -> Vec<usize> {
    let line = lines.next().unwrap();
    let result = NUMBER
        .captures_iter(line.as_str())
        .map(|x| x.get(1).unwrap().as_str().parse::<usize>().unwrap())
        .collect();
    lines.next();
    result
}

fn make_map(lines: &mut impl Iterator<Item = String>) -> Option<Box<dyn Fn(usize) -> usize>> {
    // skip section header, assuming it's always there
    let header = lines.next()?; 
    info!("section header: {:?}", header);

    // collect ranges (triplets) for this section
    let ranges = lines
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            info!("line: {:?}", line);
            let numbers: Vec<usize> = line
                .split_whitespace()
                .filter_map(|word| word.parse::<usize>().ok())
                .collect();
            (numbers[0], numbers[1], numbers[2])
        })
        .collect::<Vec<(usize, usize, usize)>>();
    info!("ranges: {:?}", ranges);

    // represent as a function
    Some(Box::new(move |i| {
        ranges
            .iter()
            .find(|&&(_, s, l)| (s..s + l).contains(&i))
            .map_or(i, |&(r, s, _)| r + i - s)
    }))
}

fn process(input: &mut impl Iterator<Item = String>) -> (usize, usize) {
    info!("process: start");

    let seeds = make_vec(input);
    info!("seeds: {:?}", &seeds);

    // create all maps and collect them into a vector
    let all_maps = 
        std::iter::successors(make_map(input), |_| make_map(input))
        .collect::<Vec<_>>();
    info!("all_maps: {:?}", all_maps.len());

    // compose all maps into a single function
    let seed_to_location = all_maps
        .into_iter()
        .rev()
        .reduce(|f, g| Box::new(move |x| f(g(x))))
        .unwrap();

    // part 1: find the minimum location for the given seeds
    let part1 = seeds
        .iter()
        .map(|&s| seed_to_location(s)) // TODO make point-free
        .min()
        .unwrap();
    info!("part 1: {}", part1);

    // part 2: find the minimum location for the given seeds interpreted as ranges
    let part2 = seeds
        .chunks(2)
        .map(|chunk| {
            (chunk[0]..chunk[0] + chunk[1])
                .map(&seed_to_location)
                .min()
                .unwrap()
        })
        .min()
        .unwrap();
    info!("part 2: {}", part2);

    (part1, part2) 
}


#[cfg(test)]

#[test]
fn test_process() { 
    let mut example = [
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ].into_iter().map(str::to_string);
    let (part1, part2) = process(&mut example);
    assert_eq!(part1, 35);
    assert_eq!(part2, 46);
}