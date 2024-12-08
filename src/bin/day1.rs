use advent24::get_lines;
const FILENAME: &'static str = "./data/d1-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}

fn part1(filename: &str) -> i32 {
  let mut list1 = Vec::new();
	let mut list2 = Vec::new();

  for line in get_lines(filename) {
   	let nums : Vec::<i32>= line.split_whitespace()
	 		.map(|num| num.parse::<i32>().unwrap())
			.collect();
		list1.push(nums[0]);
		list2.push(nums[1]);
	}
	list1.sort();
	list2.sort();
	list1.iter()
		.zip(list2)
		.fold(0,| result,(&n1,n2) | result + (n1-n2).abs())
}

fn part2(filename: &str) -> i32 {
  let mut list1 : Vec::<i32>  = vec![];
	let mut list2 : Vec::<i32>  = vec![];

  for line in get_lines(filename) {
   let (s1,s2) = line.split_once(' ').unwrap();
		let n1 : i32 = s1.trim().parse().unwrap();
		let n2 : i32 = s2.trim().parse().unwrap();
		list1.push(n1);
		list2.push(n2);
	}
	list1.iter().fold(0,|result,n1| result + n1 * list2.iter().filter(|&n2| *n1 == *n2 ).count() as i32)
}