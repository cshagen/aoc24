use std::collections::HashMap;

use advent24::get_lines;

const FILENAME: &'static str = "./data/d16-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  //println!("Part 2: {}", part2(FILENAME));
}
fn part1(filename: &str) -> i64 {
  let mut maze = Maze {
    board: vec![],
    start: Position { x: 0, y: 0 },
    end: Position { x: 0, y: 0 },
    vlen: 0,
    hlen: 0,
		cache: HashMap::new()
  };
  let dir: Direction = Direction::Right;

  maze = maze.parse(filename);
	let result: Option<u32>;
  (maze,result) = maze.walk(&Position { x: 999999, y: 999999 }, &dir, vec![]);
	result.unwrap() as i64 -1

}

struct Maze {
  board: Vec<Vec<i32>>,
  start: Position,
  end: Position,
  vlen: usize,
  hlen: usize,
	cache: HashMap<(Position,Direction),u32>
}
impl Maze {
  fn parse(mut self, filename: &str) -> Maze {
    for (y, line) in get_lines(filename).iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        if c == 'S' {
          self.start = Position { x, y };
        }
        if c == 'E' {
          self.end = Position { x, y };
        }
      }
      self.board.push(line.chars().map(|c| if c == '#' { -1 } else { 0 }).collect());
    }
    self.hlen = self.board[0].len();
    self.vlen = self.board.len();
    self
  }

  // get the next legal steps from the current position
  fn next_steps(&self, p: &Position, current_dir: Direction) -> Vec<(Position, Direction)> {
    let directions = [Direction::Right, Direction::Down, Direction::Left, Direction::Up];
    let mut result: Vec<(Position, Direction)> = vec![];
    for dir in directions {
      match dir {
        Direction::Right => {
          if current_dir != Direction::Left && p.x < self.hlen - 1 && self.is_free(p.x + 1, p.y) {
            result.push((Position { x: p.x + 1, y: p.y }, dir))
          }
        }
        Direction::Down => {
          if current_dir != Direction::Up && p.y < self.vlen - 1 && self.is_free(p.x, p.y + 1) {
            result.push((Position { x: p.x, y: p.y +1 }, dir))
          }
        }
        Direction::Left => {
          if current_dir != Direction::Right && p.x > 0 && self.is_free(p.x - 1, p.y) {
            result.push((Position { x: p.x - 1, y: p.y }, dir))
          }
        }
        Direction::Up => {
          if current_dir != Direction::Down && p.y > 0 && self.is_free(p.x, p.y -1) {
            result.push((Position { x: p.x, y: p.y -1 }, dir))
          }
        }
      };
    }
    result
  }
  // return the value at the position provided
/*   fn get(&self, pos: Position) -> i32 {
    self.board[pos.y][pos.x]
  } */
  // recursive: walk through the maze from position p, trying to find the shortest path
  fn walk(mut self, p: &Position, dir: &Direction, visited: Vec<Position>) -> (Maze, Option<u32>) {
    let pos: Position;
		//println!("{:?}",p);
		//println!("Size of visited: {}",visited.len());
    if p.x == 999999 && p.y == 999999 {
      pos = self.start.clone();
    } else {
      pos = p.clone();
    }
		let	cached = self.cache.get(&(pos.clone(),dir.clone()));
		if cached.is_some() {
			println!("CACHE HIT");
			let count = *cached.unwrap();
			return (self,Some(count));
		}
    if pos == self.end {
      println!("REACHED END");
			self.cache.insert ((pos, *dir),1);
      return (self, Some(1));
    }
    if visited.contains(&pos) {
			//println!("loop: {:?}",pos);
			return (self, None);
    }
    let next_steps = self.next_steps(&pos, *dir);
    if next_steps.len() == 0 {
			//println!("stuck: {:?}",pos);
      return (self,None);
    }
    let mut min: u32 = 4294967292;
		let mut min_dir = Direction::Right;
		let mut new_visited = visited;
    new_visited.push(pos.clone());
    for (next_pos, new_dir) in next_steps.iter() {
			let count:Option<u32>;
      (self,count) = self.walk(next_pos, new_dir, new_visited.to_vec());
			if count.is_some() {
				println!("-- Count: {}",count.unwrap());
      	if count.unwrap() <= min {
        	min = count.unwrap();
					min_dir = *new_dir;
      	}
    	} else {println!("count is none")};
		}
		if min < 4294967292 {
			println! ("Min  with {}",  min);
			if min_dir != *dir {
				self.cache.insert((pos,*dir),min+1);
				println!("cache insert");
    	return (self,Some(min + 1001));
			}
			else {
				self.cache.insert((pos,*dir),min+1);
				println!("cache insert: {}",min+1);
				return (self,Some(min+1));
			}
		} else {
		return (self,None);
	}
}

  fn is_free(&self, x: usize, y: usize) -> bool {
    self.board[y][x] != -1
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy,Hash)]
enum Direction {
  Right,
  Down,
  Left,
  Up,
}
#[derive(Debug, PartialEq, Eq, Clone,Hash)]
struct Position {
  x: usize,
  y: usize,
}
