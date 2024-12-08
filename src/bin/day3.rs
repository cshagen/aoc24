use advent24::get_lines;
use regex::Regex;

const FILENAME: &'static str = "./data/d3-input.txt";
const RE1: &str = r"mul\((?<x>\d+),(?<y>\d+)\)";
const RE_DO: &str = r"do\(\)";
const RE_DONT: &str = r"don't\(\)";

pub fn main() {
    println!("Part 1: {}", part1(FILENAME));
    println!("Part 2: {}", part2(FILENAME));
}

fn part1(filename: &str) -> i32 {
    let mut result = 0;
    let re = Regex::new(&RE1).unwrap();
    for line in get_lines(filename) {
        result += re.captures_iter(&line).fold(0, |sum, caps| {
            let x = caps.name("x").unwrap().as_str().parse::<i32>().unwrap();
            let y = caps.name("y").unwrap().as_str().parse::<i32>().unwrap();
            sum + x * y
        });
    }
    result
}

fn part2(filename: &str) -> i32 {
    let re = Regex::new(&RE1).unwrap();
    let re_start = Regex::new(&RE_DO).unwrap();
    let re_stop = Regex::new(&RE_DONT).unwrap();

		
	let mut oneline :String = "".to_owned();
	for line in get_lines (filename) {
		oneline.push_str(&line);
	}
        let starts: Vec<usize> = re_start.find_iter(&oneline).map(|mtch| mtch.start()).collect();
        let stops: Vec<usize> = re_stop.find_iter(&oneline).map(|mtch| mtch.start()).collect();
        re.captures_iter(&oneline).fold(0, |sum, caps| {
            let x = caps.name("x").unwrap().start();
						let mut last_start = 0;
            let mut last_stop = 0;
            for i in &starts {
                if i <= &x {
                    last_start = *i;
                }
            }
            for j in &stops {
                if j <= &x {
                    last_stop = *j;
                }
            }
            if last_stop == 0 || last_start >= last_stop {
							//println!("({}) {} < {}", x, last_stop, last_start);
                let a = caps.name("x").unwrap().as_str().parse::<i32>().unwrap();
                let b = caps.name("y").unwrap().as_str().parse::<i32>().unwrap();
                sum + a * b
            } else {
                //println!("{} > {}", last_stop, last_start);
                sum
            }
					})
    
    
}
