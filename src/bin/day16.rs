use advent24::get_lines;

const FILENAME: &'static str = "./data/d16-input.txt";

pub fn main() {
  let max = part1(FILENAME);
  println!("Part 1: {}", max);
  println!("Part 2: {}", part2(FILENAME, max));
}
fn part1(filename: &str) -> i32 {
  let mut maze = Maze::new();
  maze = maze.parse(filename);
  maze.scan()
}
fn part2(filename: &str, max: i32) -> i32 {
  let mut maze = Maze::new();
  maze = maze.parse2(filename);
  maze.scan2(max)
}
struct Maze {
  board: Vec<Vec<i32>>,
  board2: Vec<Vec<Vec<(Direction, i32)>>>,
  start: Position,
  end: Position,
  vlen: usize,
  hlen: usize,
  unvisited: Vec<(i32, Position, Direction)>,
  predecessors: Vec<(Position, Direction, Position, Direction)>,
}
impl Maze {
  fn new() -> Self {
    Maze {
      board: vec![],
      board2: vec![],
      start: Position { x: 0, y: 0 },
      end: Position { x: 0, y: 0 },
      vlen: 0,
      hlen: 0,
      unvisited: vec![],
      predecessors: vec![],
    }
  }

  fn parse(mut self, filename: &str) -> Maze {
    for (y, line) in get_lines(filename).iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        if c != '#' {
          if c == 'S' {
            self.start = Position::new(x, y);
            self.unvisited.push((0, Position::new(x, y), Direction::Right))
          } else {
            if c == 'E' {
              self.end = Position::new(x, y)
            };
            self.unvisited.push((i32::MAX, Position::new(x, y), Direction::Right));
          }
        }
      }
      self.board.push(line.chars().map(|c| if c == '#' { -1 } else { 0 }).collect());
    }
    self.unvisited.sort();
    self.hlen = self.board[0].len();
    self.vlen = self.board.len();
    self
  }
  fn parse2(mut self, filename: &str) -> Maze {
    let directions = [Direction::Right, Direction::Down, Direction::Left, Direction::Up];
    for (y, line) in get_lines(filename).iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        if c != '#' {
          if c == 'S' {
            self.start = Position::new(x, y);
            self.unvisited.push((0, Position::new(x, y), Direction::Right));
            self.unvisited.push((i32::MAX, Position::new(x, y), Direction::Down));
            self.unvisited.push((i32::MAX, Position::new(x, y), Direction::Left));
            self.unvisited.push((i32::MAX, Position::new(x, y), Direction::Up));
          } else {
            if c == 'E' {
              self.end = Position::new(x, y)
            };
            for d in directions {
              self.unvisited.push((i32::MAX, Position::new(x, y), d));
            }
          }
        }
      }
      self.board2.push(
        line
          .chars()
          .map(|c| {
            if c == '#' {
              vec![]
            } else {
              vec![(Direction::Right, i32::MAX), (Direction::Down, i32::MAX), (Direction::Left, i32::MAX), (Direction::Up, i32::MAX)]
            }
          })
          .collect(),
      );
    }
    self.unvisited.sort();
    self.hlen = self.board2[0].len();
    self.vlen = self.board2.len();
    self
  }

  // get the next legal steps from the current position
  fn next_steps(&self, p: &Position, current_dir: Direction) -> Vec<(Position, Direction)> {
    let directions = [Direction::Right, Direction::Down, Direction::Left, Direction::Up];
    let mut result: Vec<(Position, Direction)> = vec![];
    for dir in directions {
      match dir {
        Direction::Right => {
          let new_pos = Position::new(p.x + 1, p.y);
          if current_dir != Direction::Left && p.x < self.hlen - 1 && self.is_free(&new_pos) {
            result.push((new_pos, dir));
          }
        }
        Direction::Down => {
          let new_pos = Position::new(p.x, p.y + 1);
          if current_dir != Direction::Up && p.y < self.vlen - 1 && self.is_free(&new_pos) {
            result.push((new_pos, dir))
          }
        }
        Direction::Left => {
          let new_pos = Position::new(p.x - 1, p.y);
          if current_dir != Direction::Right && p.x > 0 && self.is_free(&new_pos) {
            result.push((new_pos, dir))
          }
        }
        Direction::Up => {
          let new_pos = Position::new(p.x, p.y - 1);
          if current_dir != Direction::Down && p.y > 0 && self.is_free(&new_pos) {
            result.push((new_pos, dir))
          }
        }
      };
    }
    result
  }
  fn next_steps_2(&self, p: &Position, current_dir: Direction) -> Vec<(Position, Direction, i32)> {
    let directions = [Direction::Right, Direction::Down, Direction::Left, Direction::Up];
    let mut result: Vec<(Position, Direction, i32)> = vec![];
    for dir in directions {
      match dir {
        Direction::Right => {
          let new_pos = Position::new(p.x + 1, p.y);
          if current_dir != Direction::Left && p.x < self.hlen - 1 && self.is_free_2(&new_pos, &dir) {
            result.push((new_pos.clone(), dir, self.get2(&new_pos, &dir)));
          }
        }
        Direction::Down => {
          let new_pos = Position::new(p.x, p.y + 1);
          if current_dir != Direction::Up && p.y < self.vlen - 1 && self.is_free_2(&new_pos, &dir) {
            result.push((new_pos.clone(), dir, self.get2(&new_pos, &dir)));
          }
        }
        Direction::Left => {
          let new_pos = Position::new(p.x - 1, p.y);
          if current_dir != Direction::Right && p.x > 0 && self.is_free_2(&new_pos, &dir) {
            result.push((new_pos.clone(), dir, self.get2(&new_pos, &dir)));
          }
        }
        Direction::Up => {
          let new_pos = Position::new(p.x, p.y - 1);
          if current_dir != Direction::Down && p.y > 0 && self.is_free_2(&new_pos, &dir) {
            result.push((new_pos.clone(), dir, self.get2(&new_pos, &dir)));
          }
        }
      };
    }
    result
  }

  fn get_previous(&self, pos: Position, dir: Direction) -> Vec<(Position, Direction)> {
    self
      .predecessors
      .iter()
      .filter(|(to_pos, to_dir, _, _)| *to_pos == pos && *to_dir == dir)
      .map(|(_, _, pos, dir)| (pos.clone(), dir.clone()))
      .collect()
  }
  // return the value at the position provided
  fn get(&self, pos: Position) -> i32 {
    self.board[pos.y][pos.x]
  }
  fn get2(&self, pos: &Position, d: &Direction) -> i32 {
    if self.board2[pos.y][pos.x] == [] {
      return -1;
    }

    self.board2[pos.y][pos.x]
      .iter()
      .filter(|(dir, _)| dir == d)
      .map(|e| e.1.clone())
      .collect::<Vec<i32>>()
      .first()
      .unwrap()
      .clone()
  }
  fn set(mut self, pos: Position, val: i32) -> Self {
    self.board[pos.y][pos.x] = val;
    self
  }
  fn set2(mut self, pos: Position, dir: Direction, val: i32) -> Self {
    self.board2[pos.y][pos.x].iter_mut().for_each(|(d, count)| {
      if *d == dir {
        *count = val.clone()
      } else {
      };
    });
    self
  }
  // Dijkstra shortest path search
  fn scan(mut self) -> i32 {
    let mut pos: Position;
    let mut dir: Direction;
    let mut distance: i32;
    let mut result: i32 = 0;

    while self.unvisited.len() > 0 {
      (self, (distance, pos, dir)) = self.pop_smallest();
      if pos == self.end {
        result = distance;
      }
      let next_steps = self.next_steps(&pos, dir);
      for (next_pos, next_dir) in next_steps.iter() {
        let next_index = self.get_unvisited((*next_pos).clone()).unwrap_or(usize::MAX);
        if next_index < usize::MAX {
          let next_step = self.unvisited[next_index].clone();
          let mut cost = distance;
          if *next_dir == dir {
            cost += 1;
          } else {
            cost += 1001;
          }
          if cost < next_step.0 {
            self.unvisited[next_index].0 = cost;
            self.unvisited[next_index].2 = next_dir.clone();
            self = self.set(next_pos.clone(), cost);
          }
        }
      }
    }
    result
  }

  fn scan2(mut self, max: i32) -> i32 {
    let mut pos: Position;
    let mut dir: Direction;
    let mut distance: i32;

    while self.unvisited.len() > 0 {
      (self, (distance, pos, dir)) = self.pop_smallest(); // take a node with the shortest distance from the start out of the unvisited list...
      let next_steps = self.next_steps_2(&pos, dir); //... and calculate the possibe steps from here
      for (next_pos, next_dir, next_distance) in next_steps.iter() {
        // For every possible step...
        let next_index = self.get_unvisited2((*next_pos).clone(), (*next_dir).clone()).unwrap_or(usize::MAX);
        if next_index < usize::MAX {
          let mut cost = distance;
          if *next_dir == dir {
            cost += 1;
          } else {
            cost += 1001;
          }
          if cost < *next_distance {
            self.unvisited[next_index].0 = cost;
            self = self.set2(next_pos.clone(), next_dir.clone(), cost);
            self.predecessors.push((next_pos.clone(), next_dir.clone(), pos.clone(), dir.clone()));
          } else if cost == *next_distance {
            self.predecessors.push((next_pos.clone(), next_dir.clone(), pos.clone(), dir.clone()));
          }
        }
      }
    }
    let mut targets = vec![];
    for d in vec![Direction::Right, Direction::Down, Direction::Left, Direction::Up] {
      if self.get2(&self.end, &d) == max {
        targets.push((self.end.clone(), d))
      }
    }

    let mut result: Vec<Position> = vec![self.end.clone()];
    while targets.len() > 0 {
      let target: (Position, Direction) = targets.splice(0..1, []).collect::<Vec<(Position, Direction)>>()[0].clone();
      let prevs = self.get_previous(target.0.clone(), target.1.clone());
      if target.0.x == 5 && target.0.y == 7 {
        println!("previous of {}/{}: {:?}", target.0.x, target.0.y, prevs);
      }
      for prev in prevs {
        if !result.contains(&prev.0) {
          result.push(prev.0.clone());
        }
        targets.push(prev.clone())
      }
    }
    result.len() as i32
  }

  fn is_free(&self, p: &Position) -> bool {
    self.get(p.clone()) != -1
  }
  fn is_free_2(&self, p: &Position, d: &Direction) -> bool {
    self.get2(p, d) != -1
  }
  fn get_unvisited(&self, p: Position) -> Option<usize> {
    self.unvisited.iter().position(|(_, pos, _)| *pos == p)
  }
  fn get_unvisited2(&self, p: Position, d: Direction) -> Option<usize> {
    self.unvisited.iter().position(|(_, pos, dir)| *pos == p && *dir == d)
  }

  fn pop_smallest(mut self) -> (Self, (i32, Position, Direction)) {
    self.unvisited.sort();
		let result = self.unvisited.splice(0..1, []).collect::<Vec<(i32, Position, Direction)>>().first().unwrap().clone();
    (self, result)
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum Direction {
  Right,
  Down,
  Left,
  Up,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
struct Position {
  x: usize,
  y: usize,
}
impl Position {
  fn new(x: usize, y: usize) -> Self {
    Position { x, y }
  }
}
