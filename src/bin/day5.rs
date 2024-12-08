use advent24::get_lines;

const FILENAME: &'static str = "./data/d5-input.txt";

pub fn main() {
    println!("Part 1: {}", part1(FILENAME));
    println!("Part 2: {}", part2(FILENAME));
}
fn part1(filename: &str) -> i32 {
    let mut result = 0;
    let mut rules: Vec<(&str, &str)> = vec![];
    let mut page_lists: Vec<Vec<&str>> = vec![];
    let mut in_rule_section = true;

    let input = get_lines(filename);
    for line in input.iter() {
        if line.len() == 0 {
            in_rule_section = false;
        } else {
            if in_rule_section {
                rules.push(line.split_once('|').unwrap());
            } else {
                let pages: Vec<&str> = line.split(',').collect();
                page_lists.push(pages);
            }
        }
    }
    for page_list in page_lists {
       if verify_page_list(&page_list, &rules) {
            result += midpage_number(&page_list);
        }
    }
    result
}

fn part2(filename: &str) -> i32 {
    let mut result = 0;
    let mut rules: Vec<(&str, &str)> = vec![];
    let mut page_lists: Vec<Vec<&str>> = vec![];
    let mut rule_section = true;
    let input = get_lines(filename);
    for line in input.iter() {
        if line.len() == 0 {
            rule_section = false;
        } else {
            if rule_section {
                rules.push(line.split_once('|').unwrap());
            } else {
                let pages: Vec<&str> = line.split(',').collect();
                page_lists.push(pages);
            }
        }
    }
    for mut page_list in page_lists {
        if !verify_page_list(&page_list, &rules) {
            while !verify_page_list(&page_list, &rules) {
                for rule in rules.iter() {
                    let p1 = page_list.iter().position(|v| v == &rule.0);
                    let p2 = page_list.iter().position(|v| v == &rule.1);
                    if p1.is_some() && p2.is_some() {
                        if p1 >= p2 {
                            let temp = page_list[p1.unwrap()];
                            page_list[p1.unwrap()] = page_list[p2.unwrap()];
                            page_list[p2.unwrap()] = temp;
                        }
                    }
                }
            }
            result += midpage_number(&page_list);
        }
    }
    result
}

fn verify_page_list(page_list: &Vec<&str>, rules: &Vec<(&str, &str)>) -> bool {
    let mut list_is_ok = true;
    for rule in rules.iter() {
        let p1 = page_list.iter().position(|v| v == &rule.0);
        let p2 = page_list.iter().position(|v| v == &rule.1);
        if p1.is_some() && p2.is_some() {
            if p1 >= p2 {
                list_is_ok = false;
            }
        }
    }
    list_is_ok
}

fn midpage_number(v: &Vec<&str>) -> i32 {
    let l = v.len() as f32;
    v[(l / 2.0).floor() as usize].parse::<i32>().unwrap()
}
