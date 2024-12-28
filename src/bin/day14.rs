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

fn part2(filename: &str) -> i64 {
  let mut mat: Vec<Vec<char>> = vec![vec![' '; (MAX_X as usize) + 2]; MAX_Y as usize + 2];
  let mut result = [0, 0, 0, 0];

  let re = Regex::new(&RE1).unwrap();
  for steps in 4890..4899 {
    for line in get_lines(filename) {
      //println!("{:?}", line);
      let parsed: Vec<i32> = re.find_iter(&line).map(|m| m.as_str().parse::<i32>().unwrap()).collect();
      let pos = (parsed[0], parsed[1]);
      let vec = (parsed[2], parsed[3]);
      let target = get_target(pos, vec, steps);
      //println!("{:?}, {:?} -> {:?}", pos, vec, target);
      mat[target.1 as usize][target.0 as usize] = '🪀';

      if target.0 < MAX_X / 2 {
        if target.1 < MAX_Y / 2 {
          result[0] += 1;
        } else if target.1 > MAX_Y / 2 {
          result[2] += 1;
        }
      } else if target.0 > MAX_X / 2 {
        if target.1 < MAX_Y / 2 {
          result[1] += 1;
        } else if target.1 > MAX_Y / 2 {
          result[3] += 1;
        }
      }
    }
    //if result[0] == result[1] && result[2]==result[3] {
      println!("{} - {:?}", steps, result);
      for y in 0..MAX_Y {
        for x in 0..MAX_X {
          print!("{}", mat[x as usize][y as usize]);
        }
        println!("");
      }
		//}
      result = [0, 0, 0, 0];
			mat = vec![vec![' '; MAX_X as usize + 2]; MAX_Y as usize + 2];
    
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
