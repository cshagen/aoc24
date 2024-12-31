use std::collections::HashMap;

use advent24::get_lines;

const FILENAME: &'static str = "./data/d16-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  //println!("Part 2: {}", part2(FILENAME));
}
fn part1(filename: &str) -> i64 {
  let mut maze = Maze::new();
  
  maze = maze.parse(filename);
  let result: Option<u64>;
  (_, result, _) = maze.walk(&Position { x: 999999, y: 999999 }, &Direction::Right, vec![]);
  result.unwrap() as i64 - 1
}

struct Maze {
  board: Vec<Vec<i64>>,
  start: Position,
  end: Position,
  vlen: usize,
  hlen: usize,
  cache: HashMap<(Position, Direction), u64>,
	unvisited: Vec<(Position,Direction,u64)>
}
impl Maze {

	fn new () -> Self {
		Maze {
			board: vec![],
    start: Position { x: 0, y: 0 },
    end: Position { x: 0, y: 0 },
    vlen: 0,
    hlen: 0,
    cache: HashMap::new(),
		unvisited: vec![]
		}
	}

  fn parse(mut self, filename: &str) -> Maze {
    for (y, line) in get_lines(filename).iter().enumerate() {
			for (x, c) in line.chars().enumerate() {
				if c != '#' {
        	if c == 'S' {
          	self.start = Position::new(x,y) };
        	}
        	if c == 'E' {
          	self.end = Position::new(x,y) };
        	}
				self.unvisited.push((Position::new(x,y),Direction::Right,i64::MAX));	
			}
      self.board.push(line.chars().map(|c| if c == '#' { -1 } else { 0 }).collect());
    }
    self.hlen = self.board[0].len();
    self.vlen = self.board.len();
    self
  }

  // get the next legal steps from the current position
  fn next_steps(&self, p: &Position, current_dir: Direction) -> (Vec<(Position, Direction)>) {
    let directions = [Direction::Right, Direction::Down, Direction::Left, Direction::Up];
    let mut result: Vec<(Position, Direction)> = vec![];
    let mut loop_detected = false;
    for dir in directions {
      match dir {
        Direction::Right => {
          let new_pos = Position { x: p.x + 1, y: p.y };
          if current_dir != Direction::Left && p.x < self.hlen - 1 && self.is_free(&new_pos) {
            result.push((new_pos, dir));
          }
        }
        Direction::Down => {
          let new_pos = Position { x: p.x, y: p.y + 1 };
          if current_dir != Direction::Up && p.y < self.vlen - 1 && self.is_free(&new_pos){
            result.push((new_pos, dir))
          }
        }
        Direction::Left => {
          let new_pos = Position { x: p.x - 1, y: p.y };
          if current_dir != Direction::Right && p.x > 0 && self.is_free(&new_pos) {
            result.push((new_pos, dir))
          }
        }
        Direction::Up => {
          let new_pos = Position { x: p.x, y: p.y - 1 };
          if current_dir != Direction::Down && p.y > 0 && self.is_free(&new_pos) {
            result.push((new_pos, dir))
          }
        }
      };
    }
    (result)
  }

  // return the value at the position provided
  fn get(&self, pos: Position) -> i64 {
    self.board[pos.y][pos.x]
  }

  // recursive: walk through the maze from position p, trying to find the shortest path
  fn walk(mut self, p: &Position, dir: &Direction, visited: Vec<(Position, Direction)>) -> (Maze, Option<u64>, bool) {
    let pos: Position;
    let mut circle: bool = false;

    if p.x == 999999 && p.y == 999999 {
      pos = self.start.clone();
    } else {
      pos = p.clone();
    }

    let cached = self.cache.get(&(pos.clone(), dir.clone()));
    if cached.is_some() {
      let count = *cached.unwrap();
      return (self, Some(count),circle);
    }

    if pos == self.end {
      println!("REACHED END");
      self.cache.insert((pos, *dir), 1);
      return (self, Some(1),false);
    }
		
    let mut new_visited = visited.to_vec();
    new_visited.push((pos.clone(), *dir));
    let next_steps: Vec<(Position, Direction)>;
    (next_steps, circle) = self.next_steps(&pos, *dir, &new_visited);
    if next_steps.len() == 0 {
     /*  if !circle || !previous_circle {
        self.cache.insert((pos, *dir), u64::MAX);
      } */
      return (self, None, circle);
    }
    let mut min: u64 = u64::MAX;
		let mut mincircle = false;
    for (next_pos, new_dir) in next_steps.iter() {
      let count: Option<u64>;
			let subcircle : bool;
      (self, count, subcircle) = self.walk(next_pos, new_dir, new_visited.to_vec());
      if count.is_some() && count.unwrap() < u64::MAX{
				let mut val = count.unwrap();
				//println!("{}",val);
				if dir != new_dir {
					val += 1000;
				} 
        if val < min {
          min = val;
					mincircle = subcircle;
				} 
    	}
		}
		circle = circle || mincircle;
    if min < u64::MAX { // we found at least 1 path to the end
      let points = min + 1;
      
      if !circle  {
        self.cache.insert((pos, *dir), points);
      }
      return (self, Some(points),circle);
     } else { // there was no path to the end
      if !circle  {
        self.cache.insert((pos, *dir), u64::MAX);
      }
      return (self, None, circle);
    } 
  }
	// Dijkstra shortest path search
	fn scan (mut self, start_pos: Position, start_dir: Direction) -> i64 {
		let mut pos = start_pos.clone();
		let mut dir = start_dir;
		let mut done = false;
		self.board[start_pos.y][start_pos.x] = 0;
		while !done {
			remove_from_unvisited (pos);
			let next_steps = self.next_steps(&pos, dir);
			let mut min = u64::MAX;
			let mut min_pos 
			for (next_pos, next_dir) in next_steps {
				let mut pts = self.get(pos);
				if next_dir == dir {
					pts += 1000;
				} else {
					pts += 1001;
				}
				if pts < self.board[next_pos.y][next_pos.x] {
					self.board[next_pos.y][next_pos.x] = pts;
				}
			}
		}
		0
	}
  fn is_free(&self, p: &Position) -> bool {
    self.get(p.clone()) != -1
  }
	fn remove_from_unvisited(mut self, p: position) {
		self.unvisited.iter().find()
		for (i,pos) in self.unvisited.iter().enumerate() {
			if pos == p {
				self.unvisited.remove(i);
			}
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
  Right,
  Down,
  Left,
  Up,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Position {
  x: usize,
  y: usize,
}
impl Position {
	fn new (x: usize, y: usize) -> Self {
		Position{x,y}
	}
}
