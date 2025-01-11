use advent24::get_lines;
const FILENAME: &'static str = "./data/d18-input.txt";
const UPPER: i32 = 71;
const BYTECOUNT1: usize = 1024;
const BYTECOUNT2: usize = 3450;
pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}

fn part1(filename: &str) -> i32 {
  let mut comp = Computer::new();
  (comp,_) = comp.parse(filename, BYTECOUNT1);
  let result: i32;
  (_, result) = comp.scan();
  result
}
fn part2(filename: &str) -> i32 {
  let mut comp = Computer::new();
	let mut result: Position;
  for i in 1..BYTECOUNT2 {
    (comp,result) = comp.parse(filename, i);
    let shortest: i32;
    (comp, shortest) = comp.scan();
    if shortest < 0 {
      println!("DONE");
			println!("Result: {:?}",result);
      return i as i32;
    } else {
      println!("Round {}: {}", i, shortest);
    }
  }
	0
}


struct Computer {
  mat: Vec<Vec<i32>>,
  unvisited: Vec<(i32, Position)>,
  end: Position,
}
impl Computer {
  fn new() -> Self {
    Computer {
      mat: vec![vec![0; UPPER as usize]; UPPER as usize],
      unvisited: vec![],
      end: Position::new(UPPER - 1, UPPER - 1),
    }
  }

  fn parse(mut self, filename: &str, count: usize) -> (Self,Position) {
    self.unvisited = vec![];
    self.mat = vec![vec![0; UPPER as usize]; UPPER as usize];
		let mut result : Position = Position::new(0,0);
    for (i, line) in get_lines(filename).iter().enumerate() {
      if i < count as usize {
        let xs: &str;
        let ys: &str;
        (xs, ys) = line.split_once(',').unwrap();
        let x: i32 = xs.to_string().parse().unwrap();
        let y: i32 = ys.to_string().parse().unwrap();
				result = Position::new(x,y);
        self = self.set(Position::new(x, y), -1);
      }
    }
    for x in 0..UPPER as i32 {
      for y in 0..UPPER as i32 {
        if x == 0 && y == 0 {
          self.unvisited.push((0, Position::new(x, y)));
        } else if self.get(Position::new(x, y)) != -1 {
          self.unvisited.push((i32::MAX, Position::new(x, y)));
        }
      }
    }

    //println!("{:?}", self.mat);
    //println!("{:?}", self.unvisited);
    (self,result)
  }

  // Dijkstra shortest path search
  fn scan(mut self) -> (Self, i32) {
    let mut pos: Position;
    let mut shortest_len: i32;
    let mut result: i32 = i32::MAX;
    self.unvisited.sort();

    while self.unvisited.len() > 0 {
      (self, (shortest_len, pos)) = self.pop_smallest();
      if pos == self.end {
        result = shortest_len;
      }
      let next_steps = self.next_steps(&pos);
      for next_pos in next_steps.iter() {
        if *next_pos != pos {
          let el_idx = self.get_unvisited((*next_pos).clone()).unwrap_or(usize::MAX);
          if el_idx < usize::MAX {
            let next_step = self.unvisited[el_idx].clone();
            let cost = shortest_len + 1;

            if cost < next_step.0 {
              self.unvisited[el_idx].0 = cost;
              self = self.set(next_pos.clone(), cost);
            }
          }
        }
      }
      self.unvisited.sort();
    }
    (self, result)
  }
  fn get_unvisited(&self, p: Position) -> Option<usize> {
    self.unvisited.iter().position(|(_, pos)| *pos == p)
  }
  fn next_steps(&self, p: &Position) -> Vec<Position> {
    let directions = [Direction::Right, Direction::Down, Direction::Left, Direction::Up];
    let mut result: Vec<Position> = vec![];
    for dir in directions {
      match dir {
        Direction::Right => {
          let new_pos = Position::new(p.x + 1, p.y);
          if p.x < UPPER - 1 && self.is_free(&new_pos) {
            result.push(new_pos);
          }
        }
        Direction::Down => {
          let new_pos = Position::new(p.x, p.y + 1);
          if p.y < UPPER - 1 && self.is_free(&new_pos) {
            result.push(new_pos)
          }
        }
        Direction::Left => {
          let new_pos = Position::new(p.x - 1, p.y);
          if p.x > 0 && self.is_free(&new_pos) {
            result.push(new_pos)
          }
        }
        Direction::Up => {
          let new_pos = Position::new(p.x, p.y - 1);
          if p.y > 0 && self.is_free(&new_pos) {
            result.push(new_pos)
          }
        }
      };
    }
    result
  }

  // return the value at the position provided
  fn get(&self, pos: Position) -> i32 {
    self.mat[pos.y as usize][pos.x as usize]
  }

  fn set(mut self, pos: Position, val: i32) -> Self {
    self.mat[pos.y as usize][pos.x as usize] = val;
    self
  }

  fn pop_smallest(mut self) -> (Self, (i32, Position)) {
    self.unvisited.sort();

    let result = self.unvisited.splice(0..1, []).collect::<Vec<(i32, Position)>>().first().unwrap().clone();
    (self, result)
  }

  fn is_free(&self, p: &Position) -> bool {
    self.get(p.clone()) != -1
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
struct Position {
  x: i32,
  y: i32,
}
impl Position {
  fn new(x: i32, y: i32) -> Self {
    Position { x, y }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Direction {
  Right,
  Down,
  Left,
  Up,
}
