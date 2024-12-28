use advent24::get_lines;
const FILENAME: &'static str = "./data/d23-input.txt";

pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  //println!("Part 2: {}", part2(FILENAME));
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
      let triple = (a.clone(),b.clone(),c.clone());// sort_triple((a, b, &c));
			let sorted =sort_triple((a, b, &c));
      if !(*a).eq(&c) 
					&& starts_with_t((&triple.0,&triple.1,&triple.2))
					&& !contains(&triples,&sorted) 
					&& (map.contains(&(c.clone(),a.clone())) || map.contains(&(a.clone(),c.clone())))
					{
						println!("{:?}",sorted);
        triples.push(sorted);
      }
    }
  }
	let result : Vec<&(String,String,String)> = triples.iter().collect();
  result.len() as i64
  
}

fn find_values_for(key: &str, map: &Vec<(String, String)>) -> Vec<String> {
  map.iter().filter(|t| t.0 == key).map(|t| t.1.to_string()).collect()
}

fn contains(triples: &Vec<(String, String, String)>, t: &(String, String, String)) -> bool {
  let a = t.0.to_string();
  let b = t.1.to_string();
  let c = t.2.to_string();
  if triples.contains(&(a.clone(), b.clone(), c.clone())) || triples.contains(&(a.clone(), c.clone(), b.clone())) 
	|| triples.contains(&(b.clone(), a.clone(), c.clone())) || triples.contains(&(b.clone(), c.clone(), a.clone())) 
	|| triples.contains(&(c.clone(), a.clone(), b.clone())) || triples.contains(&(c.clone(), b.clone(), a.clone())) {
    true
  } else {
    false
  }
}
fn starts_with_t (triple: (&str,&str,&str)) -> bool{
	let (a,b,c) = triple;
	a.chars().nth(0).unwrap()  == 't' || b.chars().nth(0).unwrap() =='t' || c.chars().nth(0).unwrap()=='t'
}
fn sort_triple (t:(&str, &str, &str)) -> (String,String,String) {
	let mut v = vec![t.0,t.1,t.2];
	v.sort();
	//println!("{:?} / {:?}", t, v);
	
	(v[0].to_string(),v[1].to_string(),v[2].to_string())
}