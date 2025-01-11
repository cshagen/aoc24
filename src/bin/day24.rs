use std::collections::HashMap;

use advent24::get_lines;
const FILENAME: &'static str = "./data/d24-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}
#[derive(Debug, Clone, PartialEq)]
enum Op {
  OR,
  AND,
  XOR,
}
#[derive(Debug, Clone)]
struct Gate {
  in1: String,
  in2: String,
  out: String,
  op: Op,
}

fn part1(filename: &str) -> i64 {
  let mut scan_gates = false;
  let mut gates: Vec<Gate> = vec![];
  let mut vals: HashMap<String, i64> = HashMap::new();
  for line in get_lines(filename) {
    if line.len() == 0 {
      scan_gates = true;
    } else if scan_gates {
      let elts: Vec<&str> = line.split_whitespace().collect();
      gates.push(Gate {
        in1: elts[0].to_string(),
        in2: elts[2].to_string(),
        out: elts[4].to_string(),
        op: match elts[1] {
          "OR" => Op::OR,
          "AND" => Op::AND,
          "XOR" => Op::XOR,
          _ => panic!("Invalid operation"),
        },
      });
    } else {
      println!("{}", line);
      let elts = line.split_once(":").unwrap();
      vals.insert(elts.0.to_string(), elts.1.trim().parse::<i64>().unwrap());
    }
  }
  println!("{:?}", gates);
  println!("{:?}", vals);
  let mut done = false;

  while !done {
    done = true;
    for g in gates.iter() {
      let executed: bool;
      (executed, vals) = exec(g, vals);
      done = done && executed;
    }
    println!("{:?}", vals);
  }

  let mut r = vals.into_iter().filter(|e| e.0.chars().nth(0).unwrap() == 'z').collect::<Vec<(String, i64)>>();
  r.sort();
  //println!("{:?}",r);
  let result: i64 = r.into_iter().map(|(_, n)| n).enumerate().fold(0, |acc, (i, b)| acc + b * 2_i64.pow(i as u32));

  println!("{:?}", result);
  0
}

fn part2(filename: &str) -> &str {
  let mut scan_gates = false;
  let mut gates: Vec<Gate> = vec![];
  let mut vals: HashMap<String, i64> = HashMap::new();
  for line in get_lines(filename) {
    if line.len() == 0 {
      scan_gates = true;
    } else if scan_gates {
      let elts: Vec<&str> = line.split_whitespace().collect();
      gates.push(Gate {
        in1: elts[0].to_string(),
        in2: elts[2].to_string(),
        out: elts[4].to_string(),
        op: match elts[1] {
          "OR" => Op::OR,
          "AND" => Op::AND,
          "XOR" => Op::XOR,
          _ => panic!("Invalid operation"),
        },
      });
    } else {
      let elts = line.split_once(":").unwrap();
      vals.insert(elts.0.to_string(), elts.1.trim().parse::<i64>().unwrap());
    }
  }
  //println!("======== INPUTS =======");
  let mut icounter = 0;
  for gate in gates.iter() {
    if ['x', 'y'].contains(&gate.in1.chars().nth(0).unwrap()) {
      //  println!("{} {:?} {} -> {}", gate.in1, gate.op, gate.in2, gate.out);
      icounter += 1;
    }
    //println!("{:?}",gate);
  }
  //println!("======== PROCESSING =======");
  let mut pcounter = 0;
  for gate in gates.iter() {
    if !['x', 'y'].contains(&gate.in1.chars().nth(0).unwrap()) && gate.out.chars().nth(0).unwrap() != 'z' {
      //  println!("{} {:?} {} -> {}", gate.in1, gate.op, gate.in2, gate.out);
      pcounter += 1
    }
  }
  let mut rcounter = 0;
  //println!("======== RESULTS =======");
  for gate in gates.iter() {
    if gate.out.chars().nth(0).unwrap() == 'z' {
      //  println!("{} {:?} {} -> {}", gate.in1, gate.op, gate.in2, gate.out);
      rcounter += 1;
    }
  }

  println!("Gates: Input {}, Process {}, Output {}", icounter, pcounter, rcounter);
  //let mut inputs : Vec<&str> = vec![];
  //for i in 1..44 {
  //	inputs.push()
  //}
/*   for input in [
    "x01", "x02", "x03", "x04", "x05", "x06", "x07", "x08", "x09", "x10", "x11", "x12", "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27", "x28",
    "x29", "x30", "x31", "x32", "x33", "x34", "x35", "x36", "x37", "x38", "x39", "x40", "x41", "x42", "x43", "x44",
  ] {
    let res = find_path_to_next_output(input, &gates);

    

    //println!("{:?}",res);
  } */
	let result = find_target("x36", 2, &gates);
	for g in result {
		println!("{} {} ({:?}) {}",g.in1,g.in2,g.op,g.out);
	}
	//println!("{:?}", result);

  "work in progress"
}
fn exec(g: &Gate, mut vals: HashMap<String, i64>) -> (bool, HashMap<String, i64>) {
  let r = vals.get(&g.out);
  if r.is_some() {
    return (true, vals);
  }
  let a = vals.get(&g.in1);
  let b = vals.get(&g.in2);
  if a.is_some() && b.is_some() {
    vals.insert(
      g.out.clone(),
      match g.op {
        Op::OR => {
          if *a.unwrap() > 0 || *b.unwrap() > 0 {
            1
          } else {
            0
          }
        }
        Op::AND => {
          if *a.unwrap() > 0 && *b.unwrap() > 0 {
            1
          } else {
            0
          }
        }
        Op::XOR => {
          if *a.unwrap() != *b.unwrap() {
            1
          } else {
            0
          }
        }
      },
    );
    return (true, vals);
  } else {
    return (false, vals);
  }
}

fn find_target(input: &str, max: usize, gates: &Vec<Gate>) -> Vec<Gate> {
  let mut result: Vec<Gate> = vec![];
  let mut backlog = find_gates_for_input(input, gates);
  let mut counter = 0;

  while backlog.len() > 0 {
    let next_gate = backlog.splice(0..1, []).collect::<Vec<Gate>>()[0].clone();
		result.push(next_gate.clone());
    if next_gate.out.chars().nth(0).unwrap() == 'z' {
      counter += 1;
    }
    if counter == max {
      return result;
    }
    let mut next_gates = find_gates_for_input(&next_gate.out, gates);
    backlog.append(&mut next_gates);
  }

  result
}


fn find_gates_for_input(input: &str, gates: &Vec<Gate>) -> Vec<Gate> {
  gates.iter().filter(|g| g.in1.clone() == input || g.in2 == input).map(|g| g.clone()).collect()
}

