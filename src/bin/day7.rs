use advent24::get_lines;
use std::i64;
const FILENAME: &'static str = "./data/d7-input.txt";
pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}

fn part1(filename: &str) -> i64 {
  let mut total = 0;
  for line in get_lines(filename) {
    let (result, pars) = parse_line(&line);
    if calc1(&pars).contains(&result) {
      total += result
    };
  }
  total
}
fn part2(filename: &str) -> i64 {
  let mut total = 0;
  for line in get_lines(filename) {
    let (result, pars) = parse_line(&line);
    if calc2(&pars, result).contains(&result) {
      total += result
    };
  }
  total
}
fn parse_line(line: &str) -> (i64, Vec<i64>) {
  line
    .split_once(":")
    .map(|(res_str, par_str)| (res_str.parse::<i64>().unwrap(), par_str.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>()))
    .unwrap()
}

fn calc1(els: &[i64]) -> Vec<i64> {
  if els.len() == 1 {
    vec![els[0]]
  } else {
    vec![
      calc1(&els[..els.len() - 1]).iter().map(|prefix| prefix + els.last().unwrap()).collect::<Vec<i64>>(),
      calc1(&els[..els.len() - 1]).iter().map(|prefix| prefix * els.last().unwrap()).collect(),
    ]
    .concat()
  }
}

fn calc2(els: &[i64], upper_bound: i64) -> Vec<i64> {
  if els.len() == 1 {
    vec![els[0]]
  } else {
    let sums: Vec<i64> = calc2(&els[..els.len() - 1], upper_bound)
      .iter()
      .filter(|n| **n <= upper_bound)
      .map(|prefix| prefix + els.last().unwrap())
      .collect();
    let prods: Vec<i64> = calc2(&els[..els.len() - 1], upper_bound)
      .iter()
      .filter(|n| **n <= upper_bound)
      .map(|prefix| prefix * els.last().unwrap())
      .collect();
    let concats: Vec<i64> = calc2(&els[..els.len() - 1], upper_bound)
      .iter()
      .filter(|n| **n <= upper_bound)
      .map(|prefix| glue2(prefix, els.last().unwrap(), upper_bound))
      .collect();

    vec![sums, prods, concats].concat()
  }
}
/* fn glue(a: &i64, b: &i64, upper_bound: i64) -> i64 {
    if *a > upper_bound || *b > upper_bound {
        0
    } else {
        let mut result = a.to_string().to_owned();
        result.push_str(&(b.to_string()));
        result.parse::<i64>().unwrap()
    }
} */
fn glue2(a: &i64, b: &i64, upper_bound: i64) -> i64 {
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
/* fn glue3(a: &i64, b: &i64, upper_bound: i64) -> i64 {
  let s = a.to_string() + &b.to_string();
  s[..].parse::<i64>().unwrap()
} */
