use std::collections::HashMap;

use advent24::get_lines;

const FILENAME: &'static str = "./data/d19-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}
fn part1(filename: &str) -> i64 {
  let mut result = 0;
  let lines = get_lines(filename);
  let towels = lines[0].split(", ").collect();
  let designs = lines[2..].iter().map(|t| &(t[..]));
  for design in designs {
    if check(design, &towels) {
			result += 1;
		}
		
  }
  result as i64
}
fn check(design: &str, towels: &Vec<&str>) -> bool {
  for towel in towels {
    if design.eq(*towel) {
      return true;
    } else {
      if towel.len() <= design.len() && design[0..towel.len()].eq(*towel) {
        if check(&design[towel.len()..], towels) {
					return true;
				}
      }
    }
  }
  return false;
}

struct Game2 {
  designs: Vec<String>,
  towels: Vec<String>,
  cache: HashMap<String, u64>,
}
impl Game2 {
  fn parse(mut self, filename: &str) -> Game2 {
    let lines = get_lines(filename);
    self.towels = lines[0].split(", ").map(|s| s.to_string()).collect();
    self.designs = lines[2..].iter().map(|t| (t[..].to_string())).collect();
    self
  }
  fn cache_find(self, design: &str) -> (Game2, Option<u64>) {
    let result = self.cache.get(design).copied();
    if result.is_some() {
      return (self, result);
    } else {
      return (self, None);
    }
  }
  fn cache_add(mut self, design: &str, count: u64) -> Game2 {
    self.cache.insert(design.to_string(), count);
    self
  }
  fn check(mut self, design: &str) -> (Game2, u64) {
    let cached: Option<u64>;
    (self, cached) = self.cache_find(design);
    if cached.is_some() {
      return (self, cached.unwrap());
    }
    let mut result = 0;
    let ts = self.towels.to_vec();
    for towel in ts {
      if design.eq(&towel) {
        result += 1;
      } else {
        if towel.len() <= design.len() && design[0..towel.len()].eq(&towel) {
          let count;
          (self, count) = self.check(&design[towel.len()..]);
          result += count;
        }
      }
    }
    self = self.cache_add(design, result);
    (self, result)
  }

  fn run(mut self) -> u64 {
    let mut result = 0;
    let ds = self.designs.to_vec();
    for design in ds {
      let count: u64;
      (self, count) = self.check(&design);
      result += count;
    }
    result
  }
}

fn part2(filename: &str) -> i64 {
  let mut game = Game2 {
    designs: vec![],
    towels: vec![],
    cache: HashMap::new(),
  };

  game = game.parse(filename);
  game.run() as i64
}
