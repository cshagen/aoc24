use advent24::get_lines;

const FILENAME: &'static str = "./data/d15-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}
fn part1(filename: &str) -> i64 {
  let mut result = 0;
  let mut board: Vec<Vec<char>> = vec![];
  let mut steps: Vec<char> = vec![];
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
        steps.push(c);
      }
    }
  }
  let mut pos = start;
  for step in steps {
    (pos, board) = move_robot(step, pos, board);
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

fn part2(filename: &str) -> i64 {
  let mut result = 0;
  let mut board: Vec<Vec<char>> = vec![];
  let mut steps: Vec<char> = vec![];
  let mut parse_board = true;
  let mut start: (usize, usize) = (0, 0);
  for (y, line) in get_lines(filename).iter().enumerate() {
    if line == "" {
      parse_board = false;
    } else if parse_board {
			let mut l : Vec<char> = vec![];
			for c in line.chars() {
				match c {
					'#' => {l.push('#');l.push('#');},
					'O' => {l.push('[');l.push(']');},
					'.' => {l.push('.');l.push('.');},
					'@' => {l.push('@');l.push('.');},
					_ => (),
				}
			}
      board.push(l);
      match line.chars().position(|c| c == '@') {
        Some(x) => start = (x, y),
        None => (),
      }
    } else {
      for c in line.chars() {
        steps.push(c);
      }
    }
  }
  let mut pos = start;
  for step in steps {
    (pos, board) = move_robot2(step, pos, board);
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
fn move_robot(step: char, pos: (usize, usize), mut mat: Vec<Vec<char>>) -> ((usize, usize), Vec<Vec<char>>) {
  let new_pos: (usize, usize) = match step {
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
      (ok, mat) = move_box(step, new_pos, mat);
      if ok {
        ((new_pos.0, new_pos.1), mat)
      } else {
        ((pos.0, pos.1), mat)
      }
    }
    _ => (pos, mat),
  }
}
fn move_robot2(step: char, pos: (usize, usize), mut mat: Vec<Vec<char>>) -> ((usize, usize), Vec<Vec<char>>) {
  let new_pos: (usize, usize) = match step {
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
    '[' => {
      (ok, mat) = move_box2(step, new_pos, mat, true);
      if ok {
        ((new_pos.0, new_pos.1), mat)
      } else {
        ((pos.0, pos.1), mat)
      }
    },
		']' => {
      (ok, mat) = move_box2(step, new_pos, mat, false);
      if ok {
        ((new_pos.0, new_pos.1), mat)
      } else {
        ((pos.0, pos.1), mat)
      }
    },
    _ => (pos, mat),
  }
}
fn move_box(step: char, pos: (usize, usize), mut mat: Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {
  let new_pos: (usize, usize) = match step {
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
      (ok, mat) = move_box(step, new_pos, mat);
      if ok {
        mat[new_pos.1][new_pos.0] = 'O';
        mat[pos.1][pos.0] = '.';
      }
      (ok, mat)
    }
    _ => (false, mat),
  }
}

fn move_box2(step: char, pos: (usize, usize), mut mat: Vec<Vec<char>>, leftside: bool) -> (bool, Vec<Vec<char>>) {
  let new_pos: (usize, usize) = match step {
    '>' => if leftside {(pos.0 + 1, pos.1)} else {(pos.0,pos.1)},
    'v' => if leftside {(pos.0, pos.1 + 1)} else {(pos.0-1,pos.1+1)},
    '<' => if leftside {(pos.0 - 1, pos.1)} else {(pos.0-2,pos.1)},
    '^' => if leftside {(pos.0, pos.1 - 1)} else {(pos.0-1,pos.1-1)},
    _ => (pos.0, pos.1),
  };
  let ok: bool;
	if step == '>' && mat[new_pos.1][new_pos.0+1] == '.'{ //right
		mat[new_pos.1][new_pos.0] = '[';
		mat[new_pos.1][new_pos.0+1 ]= ']';
		mat[new_pos.1][new_pos.0-1] = '.';
		return (true,mat);
	}
	if step == '<' && mat[new_pos.1][new_pos.0] == '.'{ // left
		mat[new_pos.1][new_pos.0] = '[';
		mat[new_pos.1][new_pos.0+1 ]= ']';
		mat[new_pos.1][new_pos.0+2] = '.';
		
		return (true,mat);
	}
	if step == 'v' {                                                             // down...
		if mat[new_pos.1][new_pos.0] == '.' && mat[new_pos.1][new_pos.0+1] =='.' { // ... and free space...
			mat[new_pos.1][new_pos.0] = '[';
			mat[new_pos.1][new_pos.0+1 ]= ']';
			mat[new_pos.1-1][new_pos.0] = '.';
			mat[new_pos.1-1][new_pos.0+1] = '.';
			return (true,mat);
		} else {																																		// ... and box to be moved...
			let boxes = check_for_boxes(&mat, pos);
			if boxes.len() >0 {
				let ok =true;
				for b in boxes {}
				 (o,mat) = move_box2(step, (b.0,b.1), mat, leftside);
					ok = ok && o;
				}
				
				return (true,mat);
			} else {
				return (false, mat);
			}
		}
	}
	
	if step == '^' && mat[new_pos.1][new_pos.0] == '.' && mat[new_pos.1][new_pos.0+1] =='.' {
		mat[new_pos.1][new_pos.0] = '[';
		mat[new_pos.1][new_pos.0+1 ]= ']';
		mat[new_pos.1+1][new_pos.0] = '.';
		mat[new_pos.1+1][new_pos.0+1] = '.';
		
		return (true,mat);
	} 
	if step='>' && mat[new_pos.1][new_pos.0+1] == '[' {
		(ok,mat) = move_box2(step, (new_pos.0+1,new_pos.1),mat, true);
		if ok {
			mat[new_pos.1][new_pos.0] = '[';
			mat[new_pos.1][new_pos.0+1 ]= ']';
			mat[new_pos.1][new_pos.0-1] = '.';
			return (true,mat);
		} else {
			return (false,mat);
		}
	}
	if step='<' && mat[new_pos.1][new_pos.0] == ']'{
		(ok,mat) = move_box2(step, (new_pos.0,new_pos.1),mat, true);
		if ok {
			mat[new_pos.1][new_pos.0] = '[';
			mat[new_pos.1][new_pos.0+1 ]= ']';
			mat[new_pos.1][new_pos.0+2] = '.';
		return (true,mat);
		} else {
			return (false,mat);
		}
	}
	if step='v' && mat[new_pos.1][new_pos.0] == ']'{
		(ok,mat) = move_box2(step, (new_pos.0,new_pos.1),mat, true);
		if ok {
			mat[new_pos.1][new_pos.0] = '[';
			mat[new_pos.1][new_pos.0+1 ]= ']';
			mat[new_pos.1][new_pos.0+2] = '.';
		return (true,mat);
		} else {
			return (false,mat);
		}
	} 
	(false,mat) 
}

fn check_for_boxes (mat: &Vec<Vec<char>>, pos: (usize,usize)) -> Vec<(usize,usize)> {
	let mut result: Vec<(usize,usize)> = vec![];
	if mat[pos.1][pos.0] == '[' {
		result.push(pos);
	} else {
		if mat[pos.1][pos.0] == ']' {
			result.push((pos.0-1,pos.1))
		}
		if mat[pos.1][pos.0+1] == '[' {
			result.push((pos.0+1,pos.1))
		}
	}
	result
}