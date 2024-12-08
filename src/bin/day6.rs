use advent24::get_lines;
use regex::Regex;
const FILENAME: &'static str = "./data/d6-input.txt";
const RE: &str = r"(?<dir>[>v<^]+)";
pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
  Right,
  Down,
  Left,
  Up,
}
fn part1(filename: &str) -> i32 {
  let mut mat: Vec<Vec<char>> = vec![];
  let mut dir: Direction = Direction::Right;
  let mut pos: (isize, isize) = (0, 0);

  let mut done = false;
  let mut visited: Vec<(isize, isize)> = vec![];
  let re = Regex::new(&RE).unwrap();
  // make matrix and find start position and direction
  for (y, line) in get_lines(filename).iter().enumerate() {
    mat.push(line.chars().collect());
    let caps = re.captures(&line);
    let d = match &caps {
      Some(caps) => caps.name("dir").unwrap().as_str(),
      None => "",
    };
    if d != "" {
      dir = match d {
        ">" => Direction::Right,
        "v" => Direction::Down,
        "<" => Direction::Left,
        "^" => Direction::Up,
        _ => panic!("Unknown directive found"),
      };
      pos = (caps.unwrap().get(0).unwrap().start() as isize, y as isize);
      visited.push(pos);

      continue;
    }
  }
  while !done {
    let mut valid_position = false;
    let mut newpos: (isize, isize) = (0, 0);
    while !valid_position {
      newpos = match dir {
        Direction::Right => (pos.0 + 1, pos.1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Up => (pos.0, pos.1 - 1),
      };
      if !((0..mat[0].len() as isize).contains(&newpos.0) && (0..mat.len() as isize).contains(&newpos.1)) {
        done = true;
        valid_position = true;
        continue;
      }
      if has_obstacle(&mat, newpos) {
        // turn right
        dir = match dir {
          Direction::Right => Direction::Down,
          Direction::Down => Direction::Left,
          Direction::Left => Direction::Up,
          Direction::Up => Direction::Right,
        };
      } else {
        valid_position = true
      }
    }
    pos = newpos;
    if !done && !visited.contains(&pos) {
      visited.push(pos);
    }
  }

  visited.len() as i32
}

fn part2(filename: &str) -> i32 {
  let mut mat: Vec<Vec<char>> = vec![];
  let mut start_dir: Direction = Direction::Right;
  let mut start_pos: (isize, isize) = (0, 0);
  let mut done = false;
  let mut barriers: Vec<(isize, isize)> = vec![];
  let mut result = 0;
  let re = Regex::new(&RE).unwrap();
  // make matrix and find start position and direction
  for (y, line) in get_lines(filename).iter().enumerate() {
    mat.push(line.chars().collect());
    let caps = re.captures(&line);
    let d = match &caps {
      Some(caps) => caps.name("dir").unwrap().as_str(),
      None => "",
    };
    if d != "" {
      start_dir = match d {
        ">" => Direction::Right,
        "v" => Direction::Down,
        "<" => Direction::Left,
        "^" => Direction::Up,
        _ => panic!("Unknown directive found"),
      };
      start_pos = (caps.unwrap().get(0).unwrap().start() as isize, y as isize);
      continue;
    }
  }
  let mut pos = start_pos;
  let mut dir = start_dir;
  // ---------------------------------------------------- walk
  while !done {
    //println!("--- Position: {}/{}",pos.0,pos.1);
    let mut valid_position = false;
    let mut newpos: (isize, isize) = (0, 0);
    // find the new position
    while !valid_position {
      // try in the current direction
      newpos = step(pos, dir);
      if out_of_bounds(newpos, &mat) {
        // we have left the area
        done = true;
        valid_position = true;
        continue;
      }
      if has_obstacle(&mat, newpos) {
        // turn right
        dir = turn_right(dir);
      } else {
        valid_position = true
      }
    }
    if !done {
      pos = newpos;
      // -------------------------------------------------test if a barrier would create a cycle
      //println!("Testing obstacle at {}/{}", pos.0, pos.1);
      if !barriers.contains(&pos) {
        mat[pos.1 as usize][pos.0 as usize] = '#';
        if has_cycle(&mat, start_pos, start_dir) {
          println!("Obstacle at {:?}", pos);
          result += 1;
          barriers.push(pos);
        }
        mat[pos.1 as usize][pos.0 as usize] = '.';
      }
    }
  }
  result
}

fn has_cycle(mat: &Vec<Vec<char>>, initial_pos: (isize, isize), initial_dir: Direction) -> bool {
  let mut pos = initial_pos;
  let mut dir = initial_dir;
  let mut loop_tracker: Vec<(isize, isize, Direction)> = vec![(pos.0, pos.1, dir)];
  // Go on until cycle found or area left
  //println!("Cycle checker ({:?})",pos);
  loop {
    let mut valid_position = false;
    let mut newpos: (isize, isize) = (0, 0);
    while !valid_position {
      newpos = step(pos, dir);
      if out_of_bounds(newpos, mat) {
        return false;
      }
      if has_obstacle(&mat, newpos) {
        dir = turn_right(dir);
      } else {
        valid_position = true
      }
    }
    if loop_tracker.contains(&(newpos.0, newpos.1, dir)) {
      //println!("CYCLE AT {}/{}",newpos.0,newpos.1);
      return true;
    } else {
      //print!("-");
      pos = newpos;
      loop_tracker.push((pos.0, pos.1, dir));
      //println!("{:?}",loop_tracker);
    }
  }
}

fn has_obstacle(mat: &Vec<Vec<char>>, pos: (isize, isize)) -> bool {
  mat[pos.1 as usize][pos.0 as usize] == '#'
}
fn step(pos: (isize, isize), dir: Direction) -> (isize, isize) {
  match dir {
    Direction::Right => (pos.0 + 1, pos.1),
    Direction::Down => (pos.0, pos.1 + 1),
    Direction::Left => (pos.0 - 1, pos.1),
    Direction::Up => (pos.0, pos.1 - 1),
  }
}
fn turn_right(d: Direction) -> Direction {
  match d {
    Direction::Right => Direction::Down,
    Direction::Down => Direction::Left,
    Direction::Left => Direction::Up,
    Direction::Up => Direction::Right,
  }
}
fn out_of_bounds(pos: (isize, isize), mat: &Vec<Vec<char>>) -> bool {
  return !((0..mat[0].len() as isize).contains(&pos.0) && (0..mat.len() as isize).contains(&pos.1));
}
