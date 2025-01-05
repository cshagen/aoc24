use advent24::get_lines;
const FILENAME: &'static str = "./data/d23-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}

fn part1(filename: &str) -> i64 {
  let lines: Vec<String> = get_lines(filename);
  let mut triples: Vec<(String, String, String)> = vec![];
  let mut map: Vec<(String, String)> = vec![];
  //parse
  for line in lines {
    let els = line.split_once('-').unwrap();
    map.push((els.0.to_string(), els.1.to_string()));
  }
  // find matching triples
  for (a, b) in map.iter() {
    for c in find_values_for(&b, &map) {
      let triple = (a.clone(), b.clone(), c.clone()); // sort_triple((a, b, &c));
      let sorted = sort_triple((a, b, &c));
      if !(*a).eq(&c) && starts_with_t((&triple.0, &triple.1, &triple.2)) && !contains(&triples, &sorted) && (map.contains(&(c.clone(), a.clone())) || map.contains(&(a.clone(), c.clone()))) {
        triples.push(sorted);
      }
    }
  }
  let result: Vec<&(String, String, String)> = triples.iter().collect();
  result.len() as i64
}

fn part2(filename: &str) -> i32 {
  let lines: Vec<String> = get_lines(filename);
  let mut groups: Vec<Vec<String>> = vec![];
  let mut edges: Vec<(String, String)> = vec![];
  //parse
  for line in lines {
    let (a, b) = line.split_once('-').unwrap();
    groups.push(vec![a.to_string(), b.to_string()]);
    edges.push((a.to_string(), b.to_string()));
    edges.push((b.to_string(), a.to_string()));
  }
  // grow the groups
  let mut targetsize = 3;
  let mut done = false;
  while !done {
    let mut added = 0;
    for group in groups.iter_mut() {
      let mut addme: String = "".to_string();
      for c in find_values_for(&group[0], &edges) {
        let mut fit = true;
        for b in group[1..].iter() {
          if !edges.contains(&(c.clone(), b.clone())) {
            fit = false;
            break;
          }
        }
        if fit {
          addme = c;
          break;
        }
      }
      if addme != "" {
        group.push(addme.clone());
        added += 1;
      } else {
      }
    }
    if added == 0 {
      done = true;
    } else {
      targetsize += 1;
    }
    groups = groups.iter().filter(|g| g.len() == targetsize - 1).map(|g| g.to_vec()).collect();
  }
  let longest: Vec<Vec<String>> = groups.iter().filter(|g| g.len() == targetsize - 1).map(|g| g.to_vec()).collect();
  let mut result = longest[0].to_vec();
  result.sort();
  for s in result.iter() {
    print!("{},", s)
  }
  println!("");
  0
}

fn find_values_for(key: &str, map: &Vec<(String, String)>) -> Vec<String> {
  map.iter().filter(|t| t.0 == key).map(|t| t.1.to_string()).collect()
}

fn contains(triples: &Vec<(String, String, String)>, t: &(String, String, String)) -> bool {
  let a = t.0.to_string();
  let b = t.1.to_string();
  let c = t.2.to_string();
  if triples.contains(&(a.clone(), b.clone(), c.clone()))
    || triples.contains(&(a.clone(), c.clone(), b.clone()))
    || triples.contains(&(b.clone(), a.clone(), c.clone()))
    || triples.contains(&(b.clone(), c.clone(), a.clone()))
    || triples.contains(&(c.clone(), a.clone(), b.clone()))
    || triples.contains(&(c.clone(), b.clone(), a.clone()))
  {
    true
  } else {
    false
  }
}
fn starts_with_t(triple: (&str, &str, &str)) -> bool {
  let (a, b, c) = triple;
  a.chars().nth(0).unwrap() == 't' || b.chars().nth(0).unwrap() == 't' || c.chars().nth(0).unwrap() == 't'
}
fn sort_triple(t: (&str, &str, &str)) -> (String, String, String) {
  let mut v = vec![t.0, t.1, t.2];
  v.sort();
  //println!("{:?} / {:?}", t, v);

  (v[0].to_string(), v[1].to_string(), v[2].to_string())
}
