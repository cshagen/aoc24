use advent24::get_lines;
use std::collections::HashMap;
const FILENAME: &'static str = "./data/d8-input.txt";
pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}

fn part1(filename: &str) -> i32 {
  let mut mat: Vec<Vec<char>> = vec![];
  let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
  let mut antinodes: Vec<(usize, usize)> = Vec::new();
  for line in get_lines(filename) {
    mat.push(line.chars().collect());
  }
  // make an antenna register
  for y in 0..mat[0].len() {
    for x in 0..mat.len() {
      let c = mat[y][x];
      if c != '.' {
        if antennas.contains_key(&c) {
          antennas.get_mut(&c).unwrap().push((x, y));
        } else {
          antennas.insert(c, vec![(x, y)]);
        }
      }
    }
  }
  // analye pairs of matching antennas
  for (_c,ants) in antennas {
    //println!("--- {:?}",ants);
    for pos1 in ants.iter() {
      for pos2 in ants.iter() {
        if pos1.0 != pos2.0 || pos1.1 != pos2.1 {
          let (x1,y1) = (pos1.0 as isize,pos1.1 as isize);
          let (x2,y2) = (pos2.0 as isize,pos2.1 as isize);
          let (xdiff,ydiff) = (x1-x2, y1-y2);
          let candidates = [(x1 + xdiff,y1+ydiff),(x2 - xdiff,y2-ydiff)];
          for p in candidates {
            if valid_cell(p,&mat) {
              let newpos = (p.0 as usize, p.1 as usize);
              if !antinodes.contains(&newpos) {
                //println!("{:?}",newpos);
              antinodes.push(newpos);
              }
            }
          }
          }
      }
    }
  }
  antinodes.len() as i32
}

fn part2(filename: &str) -> i32 {
  let mut mat: Vec<Vec<char>> = vec![];
  let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
  let mut antinodes: Vec<(usize, usize)> = Vec::new();
  for line in get_lines(filename) {
    mat.push(line.chars().collect());
  }
  // make an antenna register
  for y in 0..mat[0].len() {
    for x in 0..mat.len() {
      let c = mat[y][x];
      if c != '.' {
        if antennas.contains_key(&c) {
          antennas.get_mut(&c).unwrap().push((x, y));
        } else {
          antennas.insert(c, vec![(x, y)]);
        }
      }
    }
  }
  // analye pairs of matching antennas
  for (_c,ants) in antennas {
    for pos1 in ants.iter() {
      for pos2 in ants.iter() {
        if pos1.0 != pos2.0 || pos1.1 != pos2.1 {
          let (x1,y1) = (pos1.0 as isize,pos1.1 as isize);
          let (x2,y2) = (pos2.0 as isize,pos2.1 as isize);
          let (xdiff,ydiff) = (x1-x2, y1-y2);
          let mut cand = (x1,y1);
          while valid_cell(cand,&mat) {
            let newpos = (cand.0 as usize, cand.1 as usize);
            if !antinodes.contains(&newpos) {
              antinodes.push(newpos);
            }
            cand = (cand.0 + xdiff, cand.1 + ydiff);
          }
          cand = (x2-xdiff,y2-ydiff);
          while valid_cell(cand,&mat) {
            let newpos = (cand.0 as usize, cand.1 as usize);
            if !antinodes.contains(&newpos) {
              antinodes.push(newpos);
            }
            cand = (cand.0 - xdiff, cand.1 - ydiff);
          }
        }
      }
    }
  }
  antinodes.len() as i32
}

fn valid_cell(pos: (isize, isize), mat: &Vec<Vec<char>>) -> bool {
  (0..mat[0].len() as isize).contains(&pos.0) && (0..mat.len() as isize).contains(&pos.1)
}
