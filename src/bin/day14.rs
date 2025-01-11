use advent24::get_lines;
use regex::Regex;

const FILENAME: &'static str = "./data/d14-input.txt";
const RE1: &str = r"(-?\d+)";
const STEPS: usize = 100;
const MAX_X: i32 = 100;
const MAX_Y: i32 = 102;

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}
fn part1(filename: &str) -> i64 {
  let mut result = [0, 0, 0, 0];
  let re = Regex::new(&RE1).unwrap();

  for line in get_lines(filename) {
    //println!("{:?}", line);
    let parsed: Vec<i32> = re.find_iter(&line).map(|m| m.as_str().parse::<i32>().unwrap()).collect();
    let pos = (parsed[0], parsed[1]);
    let vec = (parsed[2], parsed[3]);

    let target = get_target(pos, vec, STEPS);
    //println!("{:?}, {:?} -> {:?}", pos, vec, target);

    if target.0 < MAX_X / 2 {
      if target.1 < MAX_Y / 2 {
        result[0] += 1;
      } else if target.1 > MAX_Y / 2 {
        result[1] += 1;
      }
    } else if target.0 > MAX_X / 2 {
      if target.1 < MAX_Y / 2 {
        result[2] += 1;
      } else if target.1 > MAX_Y / 2 {
        result[3] += 1;
      }
    }
  }
  println!("{:?}", result);
  result.iter().product::<i32>() as i64
}

#[derive(Debug,Clone)]
struct Robot {
	x: i32,
	y: i32,
	v_x: i32,
	v_y: i32,
	len_x: i32,
	len_y: i32
}

impl Robot {
	fn step(mut self) -> Self{
		self.x += self.v_x;
		if self.x < 0 {
			self.x = self.len_x + 1 + self.x;
		}
		if self.x > self.len_x {
			self.x =self.x % (self.len_x + 1);
		}
		self.y += self.v_y;
		if self.y < 0 {
			self.y = self.len_y + 1 + self.y;
		}
		if self.y > self.len_y {
			self.y =self.y % (self.len_y + 1);
		}
		self
	}
}

fn part2(filename: &str) -> i64 {
  let mut mat: Vec<Vec<char>>; //= vec![vec![' '; (MAX_X as usize) + 2]; MAX_Y as usize + 2];
  let mut result = [0, 0, 0, 0];
	let mut robots : Vec<Robot> = vec![];
	let re = Regex::new(&RE1).unwrap();
	for line in get_lines(filename) {
		let parsed: Vec<i32> = re.find_iter(&line).map(|m| m.as_str().parse::<i32>().unwrap()).collect();
		robots.push(Robot {x: parsed[0],y: parsed[1],v_x: parsed[2],v_y: parsed[3],len_x: MAX_X,len_y:MAX_Y});
	}
	for count in 1..8000 {
		mat = vec![vec![' '; (MAX_X as usize) + 2]; MAX_Y as usize + 2];
		//println!("{}",count);
		for r in robots.iter_mut() {
			let x = r.clone().step();
			*r = x;
			if r.x < MAX_X / 2 {
        if r.y < MAX_Y / 2 {
          result[0] += 1;
        } else if r.y > MAX_Y / 2 {
          result[2] += 1;
        }
      } else if r.x > MAX_X / 2 {
        if r.y < MAX_Y / 2 {
          result[1] += 1;
        } else if r.y > MAX_Y / 2 {
          result[3] += 1;
        }
      }
			mat[r.y as usize][r.x as usize] = '🪀';
		
		}
		//if (result[0] - result[1] as i32).abs() <2  &&  (result[2]-result[3] as i32).abs() < 2 {
    //if (result[3] + result[1]) < 220 && (result[0] + result[1] - result[2] - result[3] as i32) > 100{
    //if count > 7050 && count < 7060 {  
		if mat[25][40..46] == "🪀🪀🪀🪀🪀🪀".chars().collect::<Vec<char>>() {
			println!("{} - {:?}", count, result);
      for y in 0..MAX_Y {
        for x in 0..MAX_X {
          print!("{}", mat[y as usize][x as usize]);
        }
        println!("");
      }
    }
    result = [0, 0, 0, 0];
   // mat = vec![vec![' '; MAX_X as usize + 2]; MAX_Y as usize + 2];
  }
  0
}

fn get_target(pos: (i32, i32), vec: (i32, i32), steps: usize) -> (i32, i32) {
  let mut p = pos;
  for _i in 0..steps {
    //println!("{}: {:?}",i,p);
    p = ((add(p.0, vec.0, MAX_X)), (add(p.1, vec.1, MAX_Y)));
    //println!("to {:?}",p)
  }

  p
}
fn add(p: i32, d: i32, max: i32) -> i32 {
  let mut n = p + d;
  if n < 0 {
    n = max + 1 + n;
  }
  if n > max {
    n = n % (max + 1);
  }
  n
}
