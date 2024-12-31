use advent24::get_lines;

const FILENAME: &'static str = "./data/d16-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  //println!("Part 2: {}", part2(FILENAME));
}
fn part1(filename: &str) -> i32 {
  let mut maze = Maze::new();
	maze = maze.parse(filename);
  maze.scan()
}

struct Maze {
  board: Vec<Vec<i32>>,
  start: Position,
  end: Position,
  vlen: usize,
  hlen: usize,
  unvisited: Vec<(i32, Position, Direction)>,
}
impl Maze {
  fn new() -> Self {
    Maze {
      board: vec![],
      start: Position { x: 0, y: 0 },
      end: Position { x: 0, y: 0 },
      vlen: 0,
      hlen: 0,
      unvisited: vec![],
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

  // return the value at the position provided
  fn get(&self, pos: Position) -> i32 {
    self.board[pos.y][pos.x]
  }

  fn set(mut self, pos: Position, val: i32) -> Self {
    self.board[pos.y][pos.x] = val;
    self
  }

  // Dijkstra shortest path search
  fn scan(mut self) -> i32 {
    let mut pos: Position;
    let mut dir: Direction;
    let mut shortest_len: i32;
    self.unvisited.sort();

    while self.unvisited.len() > 0 {
      (self, (shortest_len, pos, dir)) = self.pop_smallest();
      if pos == self.end {
        return shortest_len;
      }
      let next_steps = self.next_steps(&pos, dir);
      for (next_pos, next_dir) in next_steps.iter() {
        let el_idx = self.get_unvisited((*next_pos).clone()).unwrap_or(usize::MAX);
        if el_idx < usize::MAX {
          let next_step = self.unvisited[el_idx].clone();
          let mut cost = shortest_len;
          if *next_dir == dir {
            cost += 1;
          } else {
            cost += 1001;
          }
          if cost < next_step.0 {
            self.unvisited[el_idx].0 = cost;
            self.unvisited[el_idx].2 = next_dir.clone();
            self = self.set(next_pos.clone(), cost);
          }
        }
      }
      self.unvisited.sort();
    }
    0
  }
  fn is_free(&self, p: &Position) -> bool {
    self.get(p.clone()) != -1
  }
  fn get_unvisited(&self, p: Position) -> Option<usize> {
    self.unvisited.iter().position(|(_, pos, _)| *pos == p)
  }

  fn pop_smallest(mut self) -> (Self, (i32, Position, Direction)) {
    self.unvisited.sort();

    let result = self.unvisited.splice(0..1, []).collect::<Vec<(i32, Position, Direction)>>().first().unwrap().clone();
    (self, result)
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Direction {
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
