use nalgebra::{Matrix2, Vector2};
use advent24::get_lines;
use regex::Regex;
const FILENAME: &'static str = "./data/d13-input.txt";
//const RE1: &str = r"X+(?<x>\d+), Y+(?<y>\d+)";

const RE1: &str = r"(?<x>\d+)";
pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}
struct Game {
  x_a: i64,
  x_b: i64,
  x_r: i64,
  y_a: i64,
  y_b: i64,
  y_r: i64,
}
fn part1(filename: &str) -> i64 {
  let lines = get_lines(filename);
  let mut result = 0;
  let re = Regex::new(&RE1).unwrap();
  let mut games: Vec<Game> = vec![];
  let mut i = 0;
  while i < lines.len() {
    let mut matches: Vec<i64> = re.find_iter(&lines[i]).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
    let x_a = matches[0];
    let y_a = matches[1];
    matches = re.find_iter(&lines[i + 1]).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
    let x_b = matches[0];
    let y_b = matches[1];
    matches = re.find_iter(&lines[i + 2]).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
    let x_r = matches[0];
    let y_r = matches[1];
    let g = Game { x_a, x_b, x_r, y_a, y_b, y_r };
    games.push(g);
    i += 4;
  }

  for game in games {
    for a in 0..100 {
      for b in 0..100 {
        if a * game.x_a + b * game.x_b == game.x_r && a * game.y_a + b * game.y_b == game.y_r {
          println!("{} - {}",a,b);
          result += a * 3 + b;
        }
      }
    }
  }
  result as i64
}

fn part2(filename: &str) -> i64 {
  let lines = get_lines(filename);
  let mut result = 0.0;
  let re = Regex::new(&RE1).unwrap();
  let mut games: Vec<Game> = vec![];
  let mut i = 0;
  while i < lines.len() {
    let mut matches: Vec<i64> = re.find_iter(&lines[i]).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
    let x_a = matches[0];
    let y_a = matches[1];
    matches = re.find_iter(&lines[i + 1]).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
    let x_b = matches[0];
    let y_b = matches[1];
    matches = re.find_iter(&lines[i + 2]).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
    let x_r = matches[0] + 10000000000000;
    let y_r = matches[1] + 10000000000000;
    let g = Game { x_a, x_b, x_r, y_a, y_b, y_r };
    games.push(g);
    i += 4;
  }

  for game in games {
    //println!("------");
   	//println!("{:?}",game.x_a);

		//let m = arr2(&[[game.x_a, game.x_b],
		//	[game.y_a,game.y_b]]);
		let m = Matrix2::new(game.x_a as f64, game.x_b as f64,game.y_a as f64,game.y_b as f64);
		let v = Vector2::new(game.x_r as f64, game.y_r as f64);
		
		match m.try_inverse() {
			Some(inv) => {
					//println!("The inverse of m1 is: {}", inv);
					let v2 = inv*v;
					let a = (v2[0]*100.0).round()/100.0;
					let b = (v2[1]*100.0).round()/100.0;
					//println!("{:?},{:?}",a,b);
					if a - a.round() == 0.0 && b - b.round() == 0.0 {
				//	println!("!");
					result += a*3.0 +b;
					}
			}
			None => {
					println!("m1 is not invertible!");
			}
	}
		

		
  }
   
  
  result as i64
}
