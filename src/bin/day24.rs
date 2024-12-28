use std::collections::HashMap;

use advent24::get_lines;
const FILENAME: &'static str = "./data/d24-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  //println!("Part 2: {}", part2(FILENAME));
}
#[derive(Debug)]
enum Op {
  OR,
  AND,
  XOR,
}
#[derive(Debug)]
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
	println!("{:?}",r);
	let result : i64= r.into_iter().map(|(_,n)|n).enumerate().fold(0,|acc,(i,b)| acc + b  * 2_i64.pow(i as u32));
	
  println!("{:?}", result);
  0
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
