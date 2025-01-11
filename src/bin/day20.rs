use advent24::get_lines;
const FILENAME: &'static str = "./data/d20-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}
fn part1(filename: &str) -> i64 {
  let mut race = Race {
    mat: vec![],
		path: vec![],
    cheats: vec![],
    start: (0, 0),
    end: (0, 0),
  };
  race = race.parse(filename);
  race = race.mark_track((0, 0), 1);
  race = race.check_cheats((9999999, 9999999));
  race.get_cheats().iter().filter(|p| p.4 >= 100).collect::<Vec<_>>().len() as i64
  
}

fn part2(filename: &str) -> i64 {
  let mut race = Race {
    mat: vec![],
		path: vec![],
    cheats: vec![],
    start: (0, 0),
    end: (0, 0),
  };
  race = race.parse(filename);
  race = race.mark_track((0, 0), 1);
  race.check_cheats2() as i64
  
}
struct Race {
  mat: Vec<Vec<i32>>,
	path: Vec<(usize,usize)>,
  cheats: Vec<(usize, usize, usize, usize, i32)>,
  start: (usize, usize),
  end: (usize, usize),
}
impl Race {
  fn parse(mut self, filename: &str) -> Race {
    for (y, line) in get_lines(filename).iter().enumerate() {
      let mut row: Vec<i32> = vec![];
      for (x, c) in line.chars().enumerate() {
        row.push(if c == '#' { -1 } else { 0 });
        self.start = if c == 'S' { (x, y) } else { self.start };
        self.end = if c == 'E' { (x, y) } else { self.end };
      }
      self.mat.push(row);
    }
    /* for line in self.mat.iter() {
      println!("{:?}", line);
    } */

    self
  }
  fn mark_track(mut self, mut pos: (usize, usize), time: i32) -> Race {
    if pos == (0, 0) {
      pos = self.start;
    }
    self.mat[pos.1][pos.0] = time;
    if pos == self.end {
      return self;
    }
		for p in self.neighbors(pos) {
      if self.mat[p.1][p.0] == 0 {
        self = self.mark_track(p, time + 1);
				self.path.splice(0..0,[p]);
      }
    }
		
    self
  }

  fn check_cheats(mut self, mut pos: (usize, usize)) -> Race {
		if pos == (9999999, 9999999) {
      pos = self.start;
    }
    let old_val = self.mat[pos.1][pos.0] + 1;
    for p1 in self.neighbors(pos) {
			if self.mat[p1.1][p1.0] == -1 {
				let mut max_benefit = 0;
				let mut max_pos =(0,0);
      	for p2 in self.neighbors(p1) {
        	if p2 != pos && self.mat[p2.1][p2.0] >0 {
			    	let val = self.mat[p2.1][p2.0];
					let benefit = val - old_val -1;
          if benefit >  max_benefit {
							max_benefit = benefit;
							max_pos = p2;
						}
          }
        if !self.cheats.contains(&(pos.0, pos.1, max_pos.0, max_pos.1, max_benefit)) {
					self.cheats.push((pos.0, pos.1, max_pos.0, max_pos.1, max_benefit));
				}
			}
		}
     if self.mat[p1.1][p1.0] == old_val {
        self = self.check_cheats(p1);
      }
    }
    self
  }

  fn neighbors(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    if p.0 > 0 {
      result.push((p.0 - 1, p.1));
    }
    if p.1 > 0 {
      result.push((p.0, p.1 - 1));
    }
    if p.0 < self.mat[0].len() - 1 {
      result.push((p.0 + 1, p.1));
    }
    if p.1 < self.mat.len() - 1 {
      result.push((p.0, p.1 + 1));
    }
    result
  }

	fn check_cheats2(&self)-> i32 {
		let mut result : Vec<(i32,i32,i32,i32,i32)> = vec![];
		for y in 0..self.mat[0].len() {
			println!("{}",y);
			for x in 0..self.mat.len() {
				if self.get (x as i32,y as i32) > -1 {
					let cheats = self.get_cheats2((x,y));
					for cheat in cheats {
						if ! result.contains(&cheat) {
							result.push(cheat.clone());
						}
					}
					
				}
		}
		}
		//println!("{:?}", result);
		result.iter().filter(|(_,_,_,_,benefit)| *benefit >= 100).collect::<Vec<_>>().len() as i32
	}



	fn get_cheats2 (&self, p: (usize, usize)) -> Vec<(i32, i32, i32, i32, i32)> {
		let mut result: Vec<(i32,i32,i32,i32,i32)> = vec![];
		let x = p.0 as i32;
		let y = p.1 as i32;
		let limit_x = self.mat[0].len() as i32;
		let limit_y = self.mat.len() as i32;
		for dx  in -20..21 {
			for dy in -20..21 {
				let steps = (dx as i32).abs()+(dy as i32).abs() ;
				if steps <= 20 && x + dx >= 0 && y+dy >= 0 && x+dx < limit_x && y + dy < limit_y {
					if self.get(x+dx,y+dy) -steps - self.get(x,y) > 99 {
						result.push ((p.0 as i32,p.1 as i32, x+dx,y+dy, self.get(x+dx,y+dy) - steps - self.get(x,y)));
					}
				}
			}
		}

		result.iter().filter(|(_,_,_,_,benefit)| *benefit >= 100).map(|r|*r).collect()
	}
  fn get_cheats(&self) -> Vec<(usize, usize, usize, usize, i32)> {
    self.cheats.to_vec()
  }

	fn get (&self, x: i32, y:i32) -> i32 {
		self.mat[y as usize][x as usize]
	}
}
