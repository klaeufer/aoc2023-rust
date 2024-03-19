use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use log::info;
use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;

static NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());

fn main() -> Result<()> {
    env_logger::init();

    process("Day 5 example", lines_from_file("data/day5example.txt").unwrap().by_ref());
    process("Day 5 solution", lines_from_file("data/day5input.txt").unwrap().by_ref());

    Ok(())
}

fn lines_from_file(filename: &str) -> Result<impl Iterator<Item = String>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines().map(|x| x.unwrap()))
}

fn make_vec(lines: &mut impl Iterator<Item = String>) -> Vec<usize> {
    let line = lines.next().unwrap();
    let result = NUMBER
        .captures_iter(&line.as_str())
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

fn process(label: &str, input: &mut impl Iterator<Item = String>) {
    info!("process: start");

    let seeds = make_vec(input);
    info!("seeds: {:?}", seeds);

    // create all maps and collect them into a vector
    let all_maps = 
        std::iter::successors(make_map(input), |_| make_map(input))
        .collect::<Vec<_>>();
    info!("all_maps: {:?}", all_maps.len());

    // compose all maps into a single function
    let seed_to_location = all_maps
        .into_iter()
        .rev()
        .reduce(|f, g| Box::new(move |x| f(g(x)))).unwrap();

    // part 1: find the minimum location for the given seeds
    let part1 = seeds
        .iter()
        .map(|&x| seed_to_location(x))
        .min().unwrap();
    println!("{} part 1: {}", label, part1);

    // part 2: find the minimum location for the given seeds interpreted as ranges
    let part2 = seeds
        .chunks(2)
        .map(|chunk| {
            (chunk[0]..chunk[0] + chunk[1])
                .map(|x| seed_to_location(x))
                .min()
                .unwrap()
        })
        .min().unwrap();
    println!("{} part 2: {}", label, part2);
}
