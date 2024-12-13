use advent24::get_lines;
const FILENAME: &'static str = "./data/d12-input.txt";
pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}
fn part1(filename: &str) -> i64 {
  let mut visited: Vec<(usize, usize)> = vec![];
  let mut region: Vec<(usize, usize)> = vec![];
  let mut mat: Vec<Vec<char>> = vec![];
  let mut result = 0;
  for line in get_lines(filename) {
    mat.push(line.chars().collect());
  }
  for x in 0..mat[0].len() {
    for y in 0..mat.len() {
      if !visited.contains(&(x, y)) {
        //println!("{},{}:{}",x,y,mat[y][x]);
        visited.push((x, y));
        region.push((x, y));
        let mut done = false;
        while !done {
          let mut add_me: Vec<(usize, usize)> = vec![];
          for c in region.iter() {
            let surrounding = neighbors(&mat, c.0, c.1);
            let new_neighbors: Vec<&(usize, usize)> = surrounding.iter().filter(|pos| !region.contains(*pos) && !add_me.contains(*pos)).collect();
            for n in new_neighbors {
              add_me.push(*n);
              visited.push(*n);
            }
          }
          if add_me.len() > 0 {
            for p in add_me {
              region.push(p);
            }
          } else {
            result += region.len() * perimeter(&region, mat[0].len() - 1, mat.len() - 1);
            done = true;
          }
        }
        region = vec![];
      }
    }
  }
  result as i64
}

fn part2(filename: &str) -> i64 {
  let mut visited: Vec<(usize, usize)> = vec![];
  let mut region: Vec<(usize, usize)> = vec![];
  let mut mat: Vec<Vec<char>> = vec![];
  let mut result = 0;
  for line in get_lines(filename) {
    mat.push(line.chars().collect());
  }
  for x in 0..mat[0].len() {
    for y in 0..mat.len() {
      if !visited.contains(&(x, y)) {
        visited.push((x, y));
        region.push((x, y));
        let mut done = false;
        while !done {
          let mut add_me: Vec<(usize, usize)> = vec![];
          for c in region.iter() {
            let surrounding = neighbors(&mat, c.0, c.1);
            let new_neighbors: Vec<&(usize, usize)> = surrounding.iter().filter(|pos| !region.contains(*pos) && !add_me.contains(*pos)).collect();
            for n in new_neighbors {
              add_me.push(*n);
              visited.push(*n);
            }
          }
          if add_me.len() > 0 {
            for p in add_me {
              region.push(p);
            }
          } else {
            let sid = sides(&region, mat[0].len() - 1, mat.len() - 1);
            result += region.len() * sid;
            done = true;
          }
        }
        region = vec![];
      }
    }
  }
  result as i64
}

fn neighbors(mat: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
  let val = mat[y][x];
  let mut result: Vec<(usize, usize)> = vec![];
  let mut candidates: Vec<(usize, usize)> = vec![];
  if x > 0 {
    candidates.push((x - 1, y));
  }
  if y > 0 {
    candidates.push((x, y - 1));
  }
  if x < mat[0].len() - 1 {
    candidates.push((x + 1, y));
  }
  if y < mat.len() - 1 {
    candidates.push((x, y + 1));
  }
  for p in candidates {
    if mat[p.1][p.0] == val {
      result.push(p);
    }
  }
  result
}

fn perimeter(region: &Vec<(usize, usize)>, max_x: usize, max_y: usize) -> usize {
  let mut result: usize = 0;
  for (x, y) in region.iter() {
    if *x == 0 || !region.contains(&(*x - 1, *y)) {
      result += 1;
    }
    if *x == max_x || !region.contains(&(*x + 1, *y)) {
      result += 1;
    }
    if *y == 0 || !region.contains(&(*x, *y - 1)) {
      result += 1;
    }
    if *y == max_y || !region.contains(&(*x, *y + 1)) {
      result += 1;
    }
  }
  result
}

fn sides(region: &Vec<(usize, usize)>, max_x: usize, max_y: usize) -> usize {
  let mut counter = 0;
  for x in 0..max_x + 1 {
    for y in 0..max_y + 1 {
      if region.contains(&(x, y)) {
        if (x == 0 || !region.contains(&(x - 1, y))) && (y == 0 || !region.contains(&(x, y - 1)) || region.contains(&(x - 1, y - 1))) {
          counter += 1;
        }
        if (x == max_x || !region.contains(&(x + 1, y))) && (y == 0 || !region.contains(&(x, y - 1)) || region.contains(&(x + 1, y - 1))) {
          counter += 1;
        }
        if (y == 0 || !region.contains(&(x, y - 1))) && (x == 0 || !region.contains(&(x - 1, y)) || region.contains(&(x - 1, y - 1))) {
          counter += 1;
        }
        if (y == max_y || !region.contains(&(x, y + 1))) && (x == 0 || !region.contains(&(x - 1, y)) || region.contains(&(x - 1, y + 1))) {
          counter += 1;
        }
      }
    }
  }
  counter
}
