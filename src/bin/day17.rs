use advent24::get_lines;
use regex::Regex;
const FILENAME: &'static str = "./data/d17-input.txt";
const RE: &str = r"(\d+)";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}

struct Machine {
  a: u64,
  b: u64,
  c: u64,
  pointer: usize,
  program: Vec<u64>,
  result: Vec<u64>,
  a_backup: u64,
  b_backup: u64,
  c_backup: u64,
  program_backup: Vec<u64>,
}
impl Machine {
  fn parse(mut self, filename: &str) -> Machine {
    let re = Regex::new(&RE).unwrap();
    let lines = get_lines(filename);
    self.a = re.find(&lines[0]).unwrap().as_str().parse::<u64>().unwrap();
    self.b = re.find(&lines[1]).unwrap().as_str().parse::<u64>().unwrap();
    self.c = re.find(&lines[2]).unwrap().as_str().parse::<u64>().unwrap();
    self.program = re.find_iter(&lines[4]).map(|s| s.as_str().parse::<u64>().unwrap()).collect();
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
  fn run_with_a(mut self, a: u64) -> Machine {
    //println!("Program Len: {}",self.program.len());
    self = self.reset();
    self.a = a;
		self.a_backup = a;
    //println!("{:?}",self.program);
    //print!("Result: ");
    //while self.a != 0 {
    while self.pointer < self.program.len() {
      self = self.step();
      //(self.a,self.b) = calc(self.a);
      //print!("{}",self.b);
      //self.result.push(self.b);
    }
    //print!(" -- {:?} -- ", self.result);
    if self.result == self.program {
      println!("{:?}, len: {}, a: {} ", self.result, self.result.len(), self.a);
      println!("a was {}",self.a_backup);
			//panic!("end");
    }
    //println!("{:?}",self.result);
    self
  }
  fn step(mut self) -> Machine {
    let op = self.program[self.pointer];
    let arg = self.program[self.pointer + 1];
    let stop = false;
    //println!(" Execute {} on {}",op,arg);
    match op {
      0 => self.a = self.a / (2_u64.pow(self.combo(arg) as u32)),
      1 => self.b = self.b ^ arg,
      2 => {
        //println!("A: {:o}",self.a);
        self.b = self.combo(arg) % 8
      }
      3 => {
        if self.a != 0 {
          self.pointer = arg as usize
        }
      }
      4 => self.b = self.b ^ self.c,
      5 => {
        self.result.push(self.combo(arg) % 8);
        //println! ("result {:?} for a = {:o}",self.result,self.a);
        //println!("print {}",self.combo(arg) % 8);
        /* 	if self.result.len() > self.program.len() { /*||  self.result.len() >0  && self.result.last().unwrap() != &self.program[self.result.len()-1] */ {
          stop = true;
        }*/
      }
      6 => self.b = self.a / (2_u64.pow(self.combo(arg) as u32)),
      7 => self.c = self.a / (2_u64.pow(self.combo(arg) as u32)),
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
  fn combo(&self, v: u64) -> u64 {
    match v {
      0 | 1 | 2 | 3 => v,
      4 => self.a,
      5 => self.b,
      6 => self.c,
      _ => panic!("Wrong command"),
    }
  }
  fn reset(mut self) -> Machine {
    self.a = self.a_backup;
    self.b = self.b_backup;
    self.c = self.c_backup;
    self.program = self.program_backup.to_vec();
    self.result = vec![];
    self.pointer = 0;
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
    program_backup: vec![],
  };
  machine = machine.parse(filename);
  machine.run();
  println!("");
  0
}
fn part2(filename: &str) -> i64 {
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
    program_backup: vec![],
  };
  machine = machine.parse(filename);
  //machine = machine.run_with_a(35100000000000);
  //machine.run_with_a(282000000000000);
  //202 941 428 935 210
  //for i in 35 100 000 000 000..282 000 000 000 000 {
  //	if i % 1000 == 0 {
  //		println!("{}",i);
  //	}

	let target = machine.program.iter().fold (0,|acc,n| acc*10+n);
	println!("target: {}", target);
	let mut input = [1,7,7,7,7,7,7,7,7, 7, 7, 7, 7, 7, 7,7];

	for p0 in 0..8 {
		input[15] = p0;
		let mut a = 0;
		for v in input {
			a = a * 8 + v;
		}
		machine = machine.run_with_a(a);
		let output = machine.result.iter().fold (0,|acc,n| acc*10+n);
		println!("{}",output);
		if output > target {
			println!("element 0: {}", p0-1);
			break
		}
		println!("{:?} / ",machine.result);
	}
	let mut a = 202322936867375 -1;
	loop {
		//println!("a= {}",a);
		machine = machine.run_with_a(a);
		a = a-1;
	} 
	/* for h in 0..8 {
  for a in 0..8 {
    for b in 0..8 {
      for c in 0..8 {
        for d in 0..8 {
					for e in 0..8 {
						for f in 0..8 {
							for g in 0..8 {
					
          let input = [5,6,0,0,1,3,7,3,h,a, b, c, d, e, f, g];
          let mut a = 0;
          for v in input {
            a = a * 8 + v;
          }
          println!("A:     {}, input: {:?}", a, input);
          machine = machine.run_with_a(a);
        }
      }
    }}}}}
  } //} */
  
}

/* fn calc (a: u64 )-> (u64,u64) {
  let mut b = a % 8;
  b = b ^ 1;
  let c = a / (2_u64.pow(b as u32));
  b = b ^ 4;
  b = b ^ c;

  (a / 8, b % 8)
} */
