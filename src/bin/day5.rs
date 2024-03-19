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

    let x1 = process_part_1(lines_from_file("data/day5example.txt").unwrap().by_ref());
    println!("Day 5 part 1 example: {:?}", x1);

    let s1 = process_part_1(lines_from_file("data/day5input.txt").unwrap().by_ref());
    println!("Day 5 part 1 solution: {:?}", s1);

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
    let header = lines.next()?; // Skip section header, assuming it's always there.
    info!("section header: {:?}", header);

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

    Some(Box::new(move |i| {
        ranges
            .iter()
            .find(|&&(_, s, l)| (s..s + l).contains(&i))
            .map_or(i, |&(r, s, _)| r + i - s)
    }))
}

fn process_part_1(input: &mut impl Iterator<Item = String>) -> usize {
    info!("process_part_1: start");
    let seeds = make_vec(input);
    info!("seeds: {:?}", seeds);
    let all_maps = std::iter::successors(make_map(input), |_| make_map(input))
        .collect::<Vec<_>>();
    info!("all_maps: {:?}", all_maps.len());
    let seed_to_location = all_maps
        .into_iter()
        .rev()
        .reduce(|f, g| Box::new(move |x| f(g(x)))).unwrap();
    seeds.iter().map(|&x| seed_to_location(x)).min().unwrap()
}

/*
  def processPart1(input: Iterator[String]) =
    val seeds = makeSeq(input)
    val allMaps = Iterator.continually(makeMap(input)).takeWhile(_.nonEmpty)
    val seedToLocation = allMaps.map(_.get).toSeq.reverse.reduce(_.compose(_))
    seeds.map(seedToLocation).min

  def processPart2(input: Iterator[String]) =
    val seeds = makeSeq(input)
    val allMaps = Iterator.continually(makeMap(input)).takeWhile(_.nonEmpty)
    val seedToLocation = allMaps.map(_.get).toSeq.reverse.reduce(_.compose(_))
    seeds.sliding(2, 2).map: p =>
      (p.head until p.head + p.last)
//        .tapEach(println)
        .map(seedToLocation).min
    .tapEach(println)
    .min
*/