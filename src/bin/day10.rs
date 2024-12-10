use advent24::get_lines;
const FILENAME: &'static str = "./data/d10-input.txt";
pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}
fn part1(filename: &str) -> i64 {
  let mut result = 0;
  let mat:Vec<Vec<u32>> = get_lines(filename).into_iter().map(|line| line.chars().map(|c| c.to_string().parse::<u32>().unwrap()).collect()).collect();
  for th in trailheads(&mat) {
    let mut cur = vec![th];
    for _i in 0..9 {
      cur = next_steps(&mat, &cur);
    }
    result += cur.len();
  }
  result as i64
}
fn part2(filename: &str) -> i64 {
  let mut result = 0;
  let mat:Vec<Vec<u32>> = get_lines(filename).into_iter().map(|line| line.chars().map(|c| c.to_string().parse::<u32>().unwrap()).collect()).collect();
  for th in trailheads(&mat) {
    let mut cur = vec![th];
    for _i in 0..9 {
      cur = next_steps2(&mat, &cur);
    }
    result += cur.len();
  }
  result as i64
}
fn next_steps(mat: &Vec<Vec<u32>>, positions: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
  let mut result: Vec<(usize, usize)> = vec![];
  for pos in positions {
    let a = pos.0 as isize;
    let b = pos.1 as isize;
    for (x, y) in [(a - 1, b), (a + 1, b), (a, b - 1), (a, b + 1)] {
      if x >= 0 && y >= 0 && x < mat[0].len() as isize && y < mat.len() as isize && mat[y as usize][x as usize] == mat[pos.1][pos.0] + 1 {
        if !result.contains(&(x as usize, y as usize)) {
          result.push((x as usize, y as usize));
        }
      }
    }
  }
  result
}
fn next_steps2(mat: &Vec<Vec<u32>>, positions: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
  let mut result: Vec<(usize, usize)> = vec![];
  for pos in positions {
    let a = pos.0 as isize;
    let b = pos.1 as isize;
    for (x, y) in [(a - 1, b), (a + 1, b), (a, b - 1), (a, b + 1)] {
      if x >= 0 && y >= 0 && x < mat[0].len() as isize && y < mat.len() as isize && mat[y as usize][x as usize] == mat[pos.1][pos.0] + 1 {
        result.push((x as usize, y as usize));
      }
    }
  }
  result
}
fn trailheads(mat: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
  let mut result: Vec<(usize, usize)> = vec![];
  for x in 0..mat[0].len() {
    for y in 0..mat.len() {
      if mat[y][x] == 0 {
        result.push((x, y));
      }
    }
  }
  result
}
