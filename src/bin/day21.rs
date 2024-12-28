use std::{cmp::min, collections::HashMap};

use advent24::get_lines;
const FILENAME: &'static str = "./data/d21-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}
// ------------------------------------------ part1()
fn part1(filename: &str) -> i64 {
  let codes: Vec<Vec<char>> = get_lines(filename).into_iter().map(|line| line.chars().collect()).collect();
  let mut prev_key_1 = 'A';
  let mut prev_key_2 = Direction::Push;
  let mut prev_key_3 = Direction::Push;
  let mut result = 0;

  for code in codes.iter() {
    let mut coderesult = 0;
    // Numberpad
    for c in code {
			let path_options = get_moves_recursive(number_pos(prev_key_1), number_pos(*c)); // all the possible paths between the two buttons
      let mut min_len = 2147483647;
      for path in path_options {
			  // Dir Pad 1
        let mut move_groups_dir1: Vec<Vec<Direction>> = vec![];
        for m in path.into_iter() {
          move_groups_dir1.push(dirpad_moves(prev_key_2, m.clone()));
          prev_key_2 = m;
        }
			  // Dir Pad 2
        let mut move_groups_dir2: Vec<Vec<Direction>> = vec![];
        for m in move_groups_dir1.concat().into_iter() {
          move_groups_dir2.push(dirpad_moves(prev_key_3, m.clone()));
          prev_key_3 = m;
        }
        let num = move_groups_dir2.concat().len() as u64;
        if num < min_len {
          min_len = num;
        }
        prev_key_2 = Direction::Push;
        prev_key_3 = Direction::Push;
			}
      coderesult += min_len;
      prev_key_1 = *c;
      prev_key_2 = Direction::Push;
      prev_key_3 = Direction::Push;
			
    }

    let num = code[0..3].into_iter().collect::<String>().parse::<u64>().unwrap();
    result += num * coderesult;
   }
  result as i64
}
// ----------------------------------------- part2()
fn part2(filename: &str) -> i64 {
  let codes: Vec<Vec<char>> = get_lines(filename).into_iter().map(|line| line.chars().collect()).collect();
  let mut prev_key_1 = 'A';
  let mut prev_key_2 = Direction::Push;
  let mut result = 0;
  let mut game2 = Game2 { cache: HashMap::new() };

	for code in codes.iter() {
		let mut coderesult = 0;
    // Numberpad
    for key in code {
	    let path_options = get_moves_recursive(number_pos(prev_key_1), number_pos(*key)); // all the possible paths between the two buttons
      let mut min_len = u64::MAX;
      for path in path_options {
	      let mut move_groups: Vec<u64> = vec![];
        for m in path.into_iter() {
          let steps: u64;
          (game2, steps) = game2.dir_pad_clicks(prev_key_2, m.clone(), 1);
          move_groups.push(steps);
          prev_key_2 = m;
        }
        let moves: u64 = move_groups.iter().sum();
        let num = moves as u64;
        if num < min_len {
          min_len = num;
        }
        prev_key_2 = Direction::Push;
	    }
      coderesult += min_len;
      prev_key_1 = *key;
      prev_key_2 = Direction::Push;
    }
    let num = code[0..3].into_iter().collect::<String>().parse::<u64>().unwrap();
    result += num * coderesult;
  }
  result as i64
}
#[derive(Clone, Debug, PartialEq, Hash, Eq, Copy)]
enum Direction {
  Right,
  Down,
  Left,
  Up,
  Push,
}
const LEVELS: u64 = 25;
struct Game2 {
  cache: HashMap<(u64, Direction, Direction), u64>,
}
impl Game2 {
	// -----------------------------------------dir_pad_clicks()
  fn dir_pad_clicks(mut self, prev: Direction, key: Direction, level: u64) -> (Game2, u64) {
    let cached = self.cache.get(&(level, prev, key)).copied();
    let result: u64;
    if cached.is_some() && cached.unwrap() > 0 {
	    return (self, cached.unwrap().clone());
    } else {
      if level == LEVELS {
        let steps_bottom = dirpad_moves(prev, key);
        result = steps_bottom.len() as u64 ;
      } else {
        let mut steps2: u64;
        let path_options = dirpad_move_options(dir_pos(prev), dir_pos(key)); // all the possible paths between the two buttons
        let mut path_results: Vec<u64> = vec![];
        if path_options.len() == 0 {
          return (self, 0);
        }
        for steps in path_options {
          let mut last = Direction::Push;
          let mut subresult: u64 = 0;
          for next in steps {
            (self, steps2) = self.dir_pad_clicks(last, next, level + 1);
            subresult += steps2.clone();
            last = next;
          }
          path_results.push(subresult);
        }
        result = path_results.iter().fold(u64::MAX, |acc, l| min(acc, *l));
      }
    }
    self = self.cache_add(level, prev, key, result);
    (self, result)
  }

  fn cache_add(mut self, level: u64, from: Direction, to: Direction, steps: u64) -> Game2 {
	  self.cache.insert((level, from, to), steps.clone());
    self
  }
}

fn number_pos(c: char) -> (i64, i64) {
  match c {
    '0' => (1, 3),
    '1' => (0, 2),
    '2' => (1, 2),
    '3' => (2, 2),
    '4' => (0, 1),
    '5' => (1, 1),
    '6' => (2, 1),
    '7' => (0, 0),
    '8' => (1, 0),
    '9' => (2, 0),
    'A' => (2, 3),
    _ => panic!("invalid key"),
  }
}
fn dir_pos(d: Direction) -> (i64, i64) {
  match d {
    Direction::Right => (2, 1),
    Direction::Down => (1, 1),
    Direction::Left => (0, 1),
    Direction::Up => (1, 0),
    Direction::Push => (2, 0),
  }
}

fn get_moves_recursive(from: (i64, i64), to: (i64, i64)) -> Vec<Vec<Direction>> {
  let mut result: Vec<Vec<Direction>> = vec![];
  let vdist = to.1 - from.1;
  let hdist = to.0 - from.0;
  if hdist > 0 {
    if hdist == 1 && vdist == 0 {
      result.push(vec![Direction::Right, Direction::Push]);
    } else {
      let mut subresults = get_moves_recursive((from.0 + 1, from.1), to);
      for list in subresults.iter_mut() {
        list.splice(0..0, [Direction::Right]);
      }
      result.append(&mut subresults);
    }
  }
  if hdist < 0 && from != (1, 3) {
    if hdist == -1 && vdist == 0 {
      result.push(vec![Direction::Left, Direction::Push]);
    } else {
      let mut subresults = get_moves_recursive((from.0 - 1, from.1), to);
      for list in subresults.iter_mut() {
        list.splice(0..0, [Direction::Left]);
      }
      result.append(&mut subresults);
    }
  }
  if vdist > 0 && from != (0, 2) {
    if vdist == 1 && hdist == 0 {
      result.push(vec![Direction::Down, Direction::Push]);
    } else {
      let mut subresults = get_moves_recursive((from.0, from.1 + 1), to);
      for list in subresults.iter_mut() {
        list.splice(0..0, [Direction::Down]);
      }
      result.append(&mut subresults);
    }
  }
  if vdist < 0 {
    if vdist == -1 && hdist == 0 {
      result.push(vec![Direction::Up, Direction::Push]);
    } else {
      let mut subresults = get_moves_recursive((from.0, from.1 - 1), to);
      for list in subresults.iter_mut() {
        list.splice(0..0, [Direction::Up]);
      }
      result.append(&mut subresults);
    }
  }
  result
}

fn dirpad_move_options(from: (i64, i64), to: (i64, i64)) -> Vec<Vec<Direction>> {
  let mut result: Vec<Vec<Direction>> = vec![];
  let vdist = to.1 - from.1;
  let hdist = to.0 - from.0;
	if hdist == 0 && vdist == 0 {
		return vec![vec![Direction::Push]];
	}
  if hdist > 0 {
    if hdist == 1 && vdist == 0 {
      result.push(vec![Direction::Right, Direction::Push]);
    } else {
      let mut subresults = dirpad_move_options((from.0 + 1, from.1), to);
      for list in subresults.iter_mut() {
        list.splice(0..0, [Direction::Right]);
      }
      result.append(&mut subresults);
    }
  }
  if hdist < 0 && from != (1, 0) {
    if hdist == -1 && vdist == 0 {
      result.push(vec![Direction::Left, Direction::Push]);
    } else {
      let mut subresults = dirpad_move_options((from.0 - 1, from.1), to);
      for list in subresults.iter_mut() {
        list.splice(0..0, [Direction::Left]);
      }
      result.append(&mut subresults);
    }
  }
  if vdist > 0 {
    if vdist == 1 && hdist == 0 {
      result.push(vec![Direction::Down, Direction::Push]);
    } else {
      let mut subresults = dirpad_move_options((from.0, from.1 + 1), to);
      for list in subresults.iter_mut() {
        list.splice(0..0, [Direction::Down]);
      }
      result.append(&mut subresults);
    }
  }
  if vdist < 0 && from != (0, 1) {
    if vdist == -1 && hdist == 0 {
      result.push(vec![Direction::Up, Direction::Push]);
    } else {
      let mut subresults = dirpad_move_options((from.0, from.1 - 1), to);
      for list in subresults.iter_mut() {
        list.splice(0..0, [Direction::Up]);
      }
      result.append(&mut subresults);
    }
  }
  result
}

fn dirpad_moves(start_key: Direction, end_key: Direction) -> Vec<Direction> {
  let start = dir_pos(start_key);
  let end = dir_pos(end_key);
  let mut result: Vec<Direction> = vec![];
  let vdist = end.1 - start.1;
  let hdist = end.0 - start.0;

  if start.1 == 0 {
    if vdist > 0 {
      for _i in 0..vdist {
        result.push(Direction::Down);
      }
    } else if vdist < 0 {
      for _i in 0..vdist.abs() {
        result.push(Direction::Up);
      }
    }
  }
  if hdist > 0 {
    for _i in 0..hdist {
      result.push(Direction::Right);
    }
  } else if hdist < 0 {
    for _i in 0..hdist.abs() {
      result.push(Direction::Left);
    }
  }
  if start.1 == 1 {
    if vdist > 0 {
      for _i in 0..vdist {
        result.push(Direction::Down);
      }
    } else if vdist < 0 {
      for _i in 0..vdist.abs() {
        result.push(Direction::Up);
      }
    }
  }
  result.push(Direction::Push);
  result
}
