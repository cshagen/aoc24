use advent24::get_lines;
const FILENAME: &'static str = "./data/d22-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}

fn part1(filename: &str) -> i64 {
  let starts: Vec<i64> = get_lines(filename).into_iter().map(|s| s.parse().unwrap()).collect();

  starts.into_iter().fold(0, |sum, num| sum + get_sequence_from(num, 2000).last().unwrap()) as i64
}

fn part2(filename: &str) -> i64 {
  let starts: Vec<i64> = get_lines(filename).into_iter().map(|s| s.parse().unwrap()).collect();
  let pricelists: Vec<Vec<i64>> = starts.into_iter().map(|n| get_prices_from(n, 2000)).collect();
  let mut max = 0;
  for a1 in -9..10 {
    println!("{}", a1);
    for a2 in -9..10 {
      for a3 in -9..10 {
        for a4 in -9..10 {
          let seq = vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()];
          let mut sum = 0;
          for pricelist in pricelists.iter() {
            sum += get_max_for_seq(&pricelist, &seq);
          }
          if sum > max {
            max = sum;
            println!("[{},{},{},{}] Sum: {}", a1, a2, a3, a4, sum);
          }
        }
      }
    }
  }
  //println!("{:?}", get_prices_from(123, 10));
  0
}
fn get_sequence_from(start: i64, count: usize) -> Vec<i64> {
  let mut result: Vec<i64> = vec![start];
  for _i in 0..count {
    result.push(next_number(*result.last().unwrap()));
  }
  result
}
fn next_number(n: i64) -> i64 {
  let mut s = n;
  s = ((s * 64) ^ s) % 16777216;
  s = ((s / 32) ^ s) % 16777216;
  (s * 2048 ^ s) % 16777216
}
fn get_prices_from(start: i64, count: usize) -> Vec<i64> {
  get_sequence_from(start, count).into_iter().map(|n| n % 10).collect()
}
fn get_max_for_seq(list: &Vec<i64>, seq: &Vec<i64>) -> i64 {
  let diffs: Vec<i64> = list.iter().enumerate().map(|(i, n)| if i > 0 { n - list[i - 1] } else { 0 }).collect();
  //let mut max = 0;
  for (i, _v) in diffs.iter().enumerate().filter(|(i,_v)| *i > 0 && *i < diffs.len()-4) {
    if seq[..] == diffs[i..i + 4] {
      //if list[i + 3] > max {
        return list[i + 3];
      //}
			//continue;
    }
  }
  //max
	0
}
