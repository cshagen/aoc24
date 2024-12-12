use std::collections::HashMap;
use advent24::get_lines;
const FILENAME: &'static str = "./data/d11-input.txt";
pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}
fn part1(filename: &str) -> i64 {
  let mut stones: Vec<u64> = get_lines(filename)[0].split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
  for _run in 0..25 {
		let mut i = 0;
		let mut progress: usize;
    while i < stones.len() {
      (stones, progress) = blink(stones, i);
			i += progress;
    }
  }
  stones.len() as i64
}

fn blink(mut stones: Vec<u64>, pos: usize) -> (Vec<u64>,usize) {
	let mut progress: usize = 1;
  if stones[pos] == 0 {
    stones[pos] = 1;
  } else if stones[pos].to_string().len() % 2 == 0 {
		let v = stones[pos].to_string();
		let m = v.to_string().len()/2;
		let v1 : u64 = v[..m].parse().unwrap();
		let v2 : u64= v[m..].parse().unwrap();
		stones[pos] = v1;
			stones.insert(pos + 1, v2);
		progress = 2;
  } else {
    stones[pos] = stones[pos] * 2024
  }
  (stones,progress)
}

struct Cache {
	dict: HashMap<(String,usize),usize> ,
}
impl Cache {
	fn new() -> Cache {
		Cache {dict: HashMap::new()}
	}
	fn add (mut self, val: String, blinks: usize, count: usize) -> Cache {
		self.dict.insert((val,blinks), count);
		self
	}
	fn find (&self, val: String,blinks:usize) -> Option<&usize> {
		self.dict.get(&(val,blinks))
	}
}

fn part2(filename: &str) -> i64 {
	let stones: Vec<String> = get_lines(filename)[0].split_whitespace().map(|s|s.to_string()).collect();
	let cache = Cache::new();
	stones.iter().fold((0,cache),|(sum,cache),stone| {
			let (count,cache) = blink2(&stone,75,cache); 
			(sum + count, cache)
	}
	).0 as i64
}

fn blink2(stone: &str, blinks: usize, mut cache: Cache ) -> (usize,Cache) {
	let cached =  cache.find(stone.to_string(), blinks);
	match cached {
		Some(n) => return (*n,cache),
		None => (),
	}
	let mut new_stones : Vec<String>= vec![];
	if stone == "0" {
    new_stones.push("1".to_string());
  } else if stone.len() % 2 == 0 {
		let m = stone.len()/2;
		let v1 = &stone[..m];
		let v2 = &stone[m..].to_string().parse::<usize>().unwrap().to_string();
		new_stones.push(v1.to_string());
		new_stones.push(v2.to_string());
	} else {
    new_stones.push((stone.parse::<i64>().unwrap() * 2024).to_string());
  }
	if blinks == 1 {
		cache = cache.add(stone.to_string(), 1, new_stones.len());
		(new_stones.len(), cache) 
	} else {
		let (count,mut cache) = new_stones.iter().fold ((0,cache),|(sum,cache),stone| {
			let (t,cache) = blink2(stone, blinks -1, cache);
			(sum + t,cache)
		});
		cache = cache.add(stone.to_string(),blinks,count);
		(count,cache)
	}
}