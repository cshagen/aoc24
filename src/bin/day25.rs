use advent24::get_lines;
const FILENAME: &'static str = "./data/d25-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  //println!("Part 2: {}", part2(FILENAME));
}

fn part1(filename: &str) -> i64 {
  let lines =  get_lines(filename);
	let mut locks : Vec<Vec<u32>> = vec![];
	let mut keys : Vec<Vec<u32>> = vec![];
	let mut result = 0;
	 
	 // PARSE INPUT
	for i in 0..lines.len()/8+1{
		if lines[i*8] == "#####" { // lock
			locks.push(get_key_or_lock(&lines[i*8..i*8+7]))
		} else { // key
			keys.push(get_key_or_lock(&lines[i*8..i*8+7]))
		}
	}
	// COMPARE LOCKS AND KEYS
	for l in locks {
		for k in keys.iter() {
			if match_key(&l,&k) {
result += 1;			}
		}
	}
	result
}

fn get_key_or_lock (scheme: &[String]) -> Vec<u32> {
	let mut result = vec![0;5];
	for s in scheme {
		for (n,c) in s.chars().enumerate() {
			if c == '#' {
			result[n] += 1;
		}
	}
	}
	result.into_iter().map(|n|n-1).collect()
}

fn match_key (lock: &Vec<u32>,key:&Vec<u32>) -> bool{
	for (i,n) in lock.iter().enumerate() {
		if *n + key[i] > 5 {
			return false;
		}
	}
	true
}