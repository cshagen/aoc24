use advent24::get_lines;
use regex::Regex;
const FILENAME: &'static str = "./data/d17-input.txt";
const RE: &str = r"(\d+)";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}

struct Machine {
  a: usize,
  b: usize,
  c: usize,
  pointer: usize,
  program: Vec<usize>,
  result: Vec<usize>,
	a_backup: usize,
	b_backup: usize,
	c_backup: usize,
	program_backup: Vec<usize>
}
impl Machine {
  fn parse(mut self, filename: &str) -> Machine {
    let re = Regex::new(&RE).unwrap();
    let lines = get_lines(filename);
    self.a = re.find(&lines[0]).unwrap().as_str().parse::<usize>().unwrap();
    self.b = re.find(&lines[1]).unwrap().as_str().parse::<usize>().unwrap();
    self.c = re.find(&lines[2]).unwrap().as_str().parse::<usize>().unwrap();
    self.program = re.find_iter(&lines[4]).map(|s| s.as_str().parse::<usize>().unwrap()).collect();
		self.a_backup = self.a;
		self.b_backup = self.b;
		self.c_backup = self.c;
		self.program_backup = self.program.to_vec();
    //println!("{} {} {} {:?}",self.a,self.b,self.c,self.program);
    self
  }
  fn run(mut self) -> Machine {
    while self.pointer < self.program.len() {
      self = self.step();
			/* let l = self.result.len();
			if l > 0 && (l > self.program.len()  || self.result[l-1] != self.program[l-1]) {
				break;
			}  */
    }
    println!("{:?} ", self.result);
    self
  }
	fn run_with_a(mut self, a: usize) -> Machine {
		//println!("Program Len: {}",self.program.len());
		self = self.reset();
		self.a = a;
		//println!("{:?}",self.program);
    while self.pointer < self.program.len() {
      self = self.step();
		
    }
		if self.result == self.program {
    	println!("{:?}, len: {}, a: {} ", self.result, self.result.len(), self.a);
		}
    self
  }
  fn step(mut self) -> Machine {
    let op = self.program[self.pointer];
    let arg = self.program[self.pointer + 1];
		let mut stop = false;
    //println!(" Execute {} on {}",op,arg);
    match op {
      0 => self.a = self.a / (2_usize.pow(self.combo(arg) as u32)),
      1 => self.b = self.b ^ arg,
      2 => self.b = self.combo(arg) % 8,
      3 => {
        if self.a != 0 {
          self.pointer = arg
        }
      }
      4 => self.b = self.b ^ self.c,
      5 => { 
				self.result.push(self.combo(arg) % 8);
				if self.result.len() > self.program.len() || self.result.len() >0 && self.result.last().unwrap() != &self.program[self.result.len()-1] {
					stop = true;
				}
			 } ,
      6 => self.b = self.a / (2_usize.pow(self.combo(arg) as u32)),
      7 => self.c = self.a / (2_usize.pow(self.combo(arg) as u32)),
      _ => panic!("invalid opcode"),
    }
    if op != 3 || self.a == 0 {
      self.pointer += 2;
    }
		if stop {
			self.pointer = self.program.len()
		}
		self
  }
  fn combo(&self, v: usize) -> usize {
    match v {
      0 | 1 | 2 | 3 => v,
      4 => self.a,
      5 => self.b,
      6 => self.c,
      _ => panic!("Wrong command"),
    }
  }
  fn check_start_val(mut self, a: usize) -> (bool,Machine) {
    self.a = a;
    self = self.run();
    if self.program == self.result {
      return (true,self);
    } else {
      return (false,self);
    }
	}
	fn reset(mut self) -> Machine{
		self.a = self.a_backup;
		self.b = self.b_backup;
		self.c = self.c_backup;
		self.program = self.program_backup.to_vec();
		self.result=vec![];
		self.pointer = 0;
		self
	} 
	fn reverse(mut self) -> Machine {
		println!("A: {}, B: {}, p: {}", self.a, self.b, self.b % 8);
		self.c = self.b ^ self.c;
		self.a = self.a *8 +3;
		self.b = self.b ^ 4;
		self.c = self.a *(2_usize.pow(self.b as u32));
		self.b = self.b ^ 1;
		self.a = self.b;
		//println!("A: {} PRINT: {}",self.a, self.b % 8);
		self
	}

	fn reset_reverse(mut self) -> Machine {
		self.a = 0;
		self.b = 8;
		self.c = 1;
	
		self
	}
}

fn part1(filename: &str) -> i64 {
  let mut machine = Machine {
    a: 0,
    b: 0,
    c: 0,
    program: vec![],
    pointer: 0,
    result: vec![],
		a_backup: 0,
		b_backup: 0,
		c_backup: 0,
		program_backup: vec![]
  };
  machine = machine.parse(filename);
  machine.run();
  println!("");
  0
}
fn part2(filename: &str) -> i64 {
  let mut machine = Machine{a:0,b:0,c:0,program: vec![],pointer:0, result: vec![], a_backup: 0, b_backup: 0, c_backup: 0, program_backup: vec![]};
  machine = machine.parse(filename);
	//machine = machine.run_with_a(35100000000000);
	//machine.run_with_a(282000000000000);
	
	for i in 35100000000000..282000000000000 {
		if i % 1000 == 0 {
			println!("{}",i);
		}
		machine = machine.run_with_a(i);
	} 
	// machine = machine.reverse();
  0
}
