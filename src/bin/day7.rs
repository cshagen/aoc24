use advent24::get_lines;
use std::u64;
use rayon::prelude::*;

const FILENAME: &'static str = "./data/d7-input.txt";
pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}

fn part1(filename: &str) -> u64 {
  let lines = get_lines(filename);
  lines
    .iter()
    .map(|s| &s[..])
    .collect::<Vec<_>>()
    .par_chunks(lines.len() / 32)
    .map(|chunk| {
      chunk
        .iter()
        .map(|line| {
          let (result, pars) = parse_line(&line);
          if calc1(&pars).contains(&result) {
            result
          } else {
            0
          }
        })
        .sum::<u64>()
    })
    .sum()
}
fn part2(filename: &str) -> u64 {
  let lines = get_lines(filename);
  lines
    .iter()
    .map(|s| &s[..])
    .collect::<Vec<_>>()
    .par_chunks(lines.len() / 32)
    .map(|chunk| {
      chunk
        .iter()
        .map(|line| {
          let (result, pars) = parse_line(&line);
          if calc2(&pars, result).contains(&result) {
            result
          } else {
            0
          }
        })
        .sum::<u64>()
    })
    .sum()
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
  line
    .split_once(":")
    .map(|(res_str, par_str)| (res_str.parse::<u64>().unwrap(), par_str.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>()))
    .unwrap()
}

fn calc1(els: &[u64]) -> Vec<u64> {
  let mut res: Vec<u64> = vec![els[0]];
  for i in 1..els.len() {
    res = res.iter().map(|a| vec![a * els[i], a + els[i]]).collect::<Vec<Vec<u64>>>().concat();
  }
  res
}

fn calc2(els: &[u64], upper_bound: u64) -> Vec<u64> {
  let mut res: Vec<u64> = vec![els[0]];
  for i in 1..els.len() {
    res = res.iter().map(|a| vec![a * els[i], a + els[i], glue(a, &els[i], upper_bound)]).collect::<Vec<Vec<u64>>>().concat();
  }
  res
}

fn glue(a: &u64, b: &u64, upper_bound: u64) -> u64 {
  if *a > upper_bound || *b > upper_bound {
    0
  } else {
    let mut log = 10;
    let mut n = *b;
    while n / 10 > 0 {
      log = log * 10;
      n = n / 10;
    }
    //println!("-- {} -- {} -- {} -- {}", *a,*b,log, *a * log + *b);
    a * log + b
  }
}
