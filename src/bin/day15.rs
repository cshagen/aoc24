use advent24::get_lines;

const FILENAME: &'static str = "./data/d15-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}
fn part1(filename: &str) -> i64 {
  let mut result = 0;
  let mut board: Vec<Vec<char>> = vec![];
  let mut dirs: Vec<char> = vec![];
  let mut parse_board = true;
  let mut start: (usize, usize) = (0, 0);
  for (y, line) in get_lines(filename).iter().enumerate() {
    if line == "" {
      parse_board = false;
    } else if parse_board {
      board.push(line.chars().collect());
      match line.chars().position(|c| c == '@') {
        Some(x) => start = (x, y),
        None => (),
      }
    } else {
      for c in line.chars() {
        dirs.push(c);
      }
    }
  }
  let mut pos = start;
  for dir in dirs {
		
    (pos, board) = move_robot(dir, pos, board);
  }

  for y in 0..board.len() {
    for x in 0..board[0].len() {
      if board[y][x] == 'O' {
        result += x + 100 * y;
      }
    }
  }
  result as i64
}

fn move_robot(dir: char, pos: (usize, usize), mut mat: Vec<Vec<char>>) -> ((usize, usize), Vec<Vec<char>>) {
  let new_pos: (usize, usize) = match dir {
    '>' => (pos.0 + 1, pos.1),
    'v' => (pos.0, pos.1 + 1),
    '<' => (pos.0 - 1, pos.1),
    '^' => (pos.0, pos.1 - 1),
    _ => (pos.0, pos.1),
  };
  let ok: bool;
  match mat[new_pos.1][new_pos.0] {
    '#' => (pos, mat),
    '.' => (new_pos, mat),
    '@' => (new_pos, mat),
    'O' => {
      (ok, mat) = move_box(dir, new_pos, mat);
      if ok {
        ((new_pos.0, new_pos.1), mat)
      } else {
        ((pos.0, pos.1), mat)
      }
    }
    _ => (pos, mat),
  }
}
fn move_box(dir: char, pos: (usize, usize), mut mat: Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {
  let new_pos: (usize, usize) = match dir {
    '>' => (pos.0 + 1, pos.1),
    'v' => (pos.0, pos.1 + 1),
    '<' => (pos.0 - 1, pos.1),
    '^' => (pos.0, pos.1 - 1),
    _ => (pos.0, pos.1),
  };
  let ok: bool;
  match mat[new_pos.1][new_pos.0] {
    '#' => (false, mat),
    '.' => {
      mat[new_pos.1][new_pos.0] = 'O';
      mat[pos.1][pos.0] = '.';
      (true, mat)
    }
    '@' => {
      mat[new_pos.1][new_pos.0] = 'O';
      mat[pos.1][pos.0] = '.';
      (true, mat)
    }
    'O' => {
      (ok, mat) = move_box(dir, new_pos, mat);
      if ok {
        mat[new_pos.1][new_pos.0] = 'O';
        mat[pos.1][pos.0] = '.';
      }
      (ok, mat)
    }
    _ => (false, mat),
  }
}
#[derive(Debug, Clone)]
struct Position {
  x: i32,
  y: i32,
}
impl Position {
  fn new(x: i32, y: i32) -> Self {
    Position { x, y }
  }
}
fn part2(filename: &str) -> i64 {
  let mut result = 0;
  let mut board: Vec<Vec<char>> = vec![];
  let mut dirs: Vec<char> = vec![];
  let mut parse_board = true;
  let mut start = Position::new(0, 0);
  for (y, line) in get_lines(filename).iter().enumerate() {
    if line == "" {
      parse_board = false;
    } else if parse_board {
      let mut l: Vec<char> = vec![];
      for c in line.chars() {
        match c {
          '#' => {
            l.push('#');
            l.push('#');
          }
          'O' => {
            l.push('[');
            l.push(']');
          }
          '.' => {
            l.push('.');
            l.push('.');
          }
          '@' => {
            l.push('.');
            l.push('.');
          }
          _ => (),
        }
      }
      board.push(l);
      match line.chars().position(|c| c == '@') {
        Some(x) => start = Position::new(x as i32 *2, y as i32),
        None => (),
      }
    } else {
      for c in line.chars() {
        dirs.push(c);
      }
    }
  }
  let mut pos = start;
  // move the robot through the area
  for dir in dirs {
		//show_mat(&board,&pos);
    println!("--- {:?} {} ---", pos, dir);
		
    (pos, board) = move_robot2(dir, &pos, board);
  }
	show_mat(&board,&pos);
  // calculate the resulting score
  for y in 0..board.len() {
    for x in 0..board[0].len() {
      if board[y][x] == '[' {
        result += x + 100 * y;
      }
    }
  }
  result as i64
}
// move the robot 1 step
fn move_robot2(dir: char, from: &Position, mut mat: Vec<Vec<char>>) -> (Position, Vec<Vec<char>>) {
  let to: Position = match dir {
    '>' => Position::new(from.x + 1, from.y),
    'v' => Position::new(from.x, from.y + 1),
    '<' => Position::new(from.x - 1, from.y),
    '^' => Position::new(from.x, from.y - 1),
    _ => from.clone(),
  };
  match mat[to.y as usize][to.x as usize] {
    '#' => (from.clone(), mat),
    '.' => (to, mat),
    '@' => (to, mat),
    '[' => {
      if box_is_movable(&to, dir, &mat) {
        mat = move_box2(dir, &to, mat);
        (to, mat)
      } else {
        (from.clone(), mat)
      }
    }
    ']' => {
      if box_is_movable(&(Position::new(to.x - 1, to.y)), dir, &mat) {
        mat = move_box2(dir, &Position::new(to.x - 1, to.y), mat);
				(to, mat)
      } else {
				(from.clone(), mat)
			}
      
    }
    _ => (from.clone(), mat),
  }
}

// move a box
fn move_box2(dir: char, from: &Position, mut mat: Vec<Vec<char>>) -> Vec<Vec<char>> {
  println!("move_box2 {:?}", from);
  match dir {
    '>' => {
      if has_box(&Position::new(from.x + 2, from.y), &mat) {
        mat = move_box2(dir, &Position::new(from.x + 2, from.y), mat);
      }
      mat[from.y as usize][from.x as usize +1] = '[';
      mat[from.y as usize][from.x as usize + 2] = ']';
      mat[from.y as usize][from.x as usize] = '.';
    }
    '<' => {
      if has_box(&Position::new(from.x-2,from.y), &mat) {
        mat = move_box2(dir, &Position::new(from.x - 2, from.y), mat);
      }
      mat[from.y as usize][from.x as usize -1] = '[';
      mat[from.y as usize][from.x as usize] = ']';
      mat[from.y as usize][from.x as usize + 1] = '.';
    }
    'v' => {
      if has_box(&Position::new(from.x, from.y +1), &mat) || has_box(&Position::new(from.x +1, from.y +1), &mat) {
        let left = get(from.x,from.y+1,&mat);
				let right = get(from.x+1, from.y+1,&mat);

				if left == '[' {
					mat = move_box2(dir, &Position::new(from.x, from.y + 1), mat);
      	} else if left == ']' {
					mat = move_box2(dir, &Position::new(from.x-1, from.y + 1), mat);
				}
      	if right =='['  {
					mat = move_box2(dir, &Position::new(from.x+1, from.y + 1), mat);
				}
				 
		}
      mat[from.y as usize +1][from.x as usize] = '[';
      mat[from.y as usize+1][from.x as usize + 1] = ']';
      mat[from.y as usize][from.x as usize] = '.';
			mat[from.y as usize][from.x as usize +1] = '.';
			
    }
    '^' => {
      if has_box(&Position::new(from.x, from.y-1), &mat)  ||has_box(&Position::new(from.x+1, from.y-1), &mat) {
				let left = get(from.x,from.y-1,&mat);
				let right = get(from.x+1, from.y-1,&mat);
				if left == '[' {
					mat = move_box2(dir, &Position::new(from.x, from.y - 1), mat);
      	} else if left == ']' {
					mat = move_box2(dir, &Position::new(from.x-1, from.y - 1), mat);
				}
      	if right =='['  {
					mat = move_box2(dir, &Position::new(from.x+1, from.y - 1), mat);
				}
			} 
      
		
      mat[from.y as usize-1][from.x as usize] = '[';
      mat[from.y as usize-1][from.x as usize + 1] = ']';
      mat[from.y as usize][from.x as usize] = '.';
			mat[from.y as usize][from.x as usize+1] = '.';
			
    }
    _ => panic!("invalid direction: {}", dir),
  }
  mat
}

fn box_is_movable(from: &Position, dir: char, mat: &Vec<Vec<char>>) -> bool {
  println!("box_is_movable {:?}", from);
  match dir {
    '>' => {
			let to = Position::new(from.x+1,from.y);
			(to.x as usize) < mat[0].len() -2  && (is_free(&Position::new(to.x+1,to.y), mat) || has_box(&to, mat) && box_is_movable(&to, dir, mat) )
		},
    '<' => {
			let to = Position::new(from.x-1,from.y);
			is_free(&to, mat) || has_box(&Position::new(to.x-1,to.y), mat) && box_is_movable(&Position::new(to.x -1,to.y), dir, mat)
		},
    '^' => {
			let to = Position::new(from.x,from.y-1);
			let mut boxes: Vec<Position> = vec![];
      let next_left = mat[to.y as usize][to.x as usize];
      let next_right = mat[to.y as usize][to.x as usize + 1];
      if next_left == '[' {
        boxes.push(Position::new(to.x, to.y));
      } else {
        if next_left == ']' {
          boxes.push(Position::new(to.x - 1, to.y));
        }
        if next_right == '[' {
          boxes.push(Position::new(to.x + 1, to.y));
        }
      }
      let mut boxes_movable = true;
			
      for b in boxes {
        boxes_movable = boxes_movable && box_is_movable(&b, dir, mat);
      }
      is_free_or_has_box(&to, mat) && is_free_or_has_box(&Position::new(to.x + 1, to.y), mat) && boxes_movable
    }
    'v' => {
			let to = Position::new(from.x,from.y+1);
      let mut boxes: Vec<Position> = vec![];
      let next_left = mat[to.y as usize][to.x as usize];
      let next_right = mat[to.y as usize][to.x as usize + 1];
      if next_left == '[' {
        boxes.push(Position::new(to.x, to.y));
      } else {
        if next_left == ']' {
          boxes.push(Position::new(to.x - 1, to.y));
        }
        if next_right == '[' {
          boxes.push(Position::new(to.x + 1, to.y));
        }
      }
      let mut boxes_movable = true;
      for b in boxes {
        boxes_movable = boxes_movable && box_is_movable(&b, dir, mat);
      }
      is_free_or_has_box(&to, mat) && is_free_or_has_box(&Position::new(to.x + 1, to.y), mat) && boxes_movable
    }

    _ => panic!("invalid direction"),
  }
}

fn is_free(pos: &Position, mat: &Vec<Vec<char>>) -> bool {
  ['.'].contains(&mat[pos.y as usize][pos.x as usize])
}
fn is_free_or_has_box(pos: &Position, mat: &Vec<Vec<char>>) -> bool {
  ['.', '[', ']'].contains(&mat[pos.y as usize][pos.x as usize])
}

fn has_box(pos: &Position, mat: &Vec<Vec<char>>) -> bool {
	println!("{:?}: {}",pos,mat[pos.y as usize][pos.x as usize]);
  pos.x < mat[0].len() as i32 && pos.y < mat.len() as i32 && ['[', ']'].contains(&mat[pos.y as usize][pos.x as usize])
}
fn show_mat(mat:&Vec<Vec<char>>,pos:&Position) {
	for (y,row) in mat.iter().enumerate() {
		for (x,c) in row.iter().enumerate() {
			if x == pos.x as usize && y == pos.y as usize {
				print!("{}",'@')
			} else {
				print!("{}",c);
		}
		}	
		println!("");
	}
}
fn get(x:i32,y:i32,mat:&Vec<Vec<char>>) -> char {
	mat[y as usize][x as usize]
}