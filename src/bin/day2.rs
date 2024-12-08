use advent24::get_lines;
const FILENAME: &'static str = "./data/d2-input.txt";

pub fn main() {
    println!("Part 1: {}", part1(FILENAME));
    println!("Part 2: {}", part2(FILENAME));
}

fn part1(filename: &str) -> i32 {
    let mut result = 0;

    for line in get_lines(filename) {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let mut prev = levels[0];
        let mut safe = true;
        if levels[0] == levels[1] {
            safe = false;
        } else {
            let up = levels[0] < levels[1];
            for l in &levels[1..] {
                if (l - prev).abs() > 3 {
                    safe = false;
                }
                if up {
                    if l <= &prev {
                        safe = false;
                    }
                } else {
                    if l >= &prev {
                        safe = false
                    }
                }
                prev = *l;
            }
        }
        if safe {
            result += 1;
        }
    }
    result
}

fn part2(filename: &str) -> i32 {
    let mut result = 0;

    for line in get_lines(filename) {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        if safe(&levels, usize::MAX) {
            result += 1;
        } else {
            for i in 0..(levels.len()) {
                if safe(&levels, i) {
                    result += 1;
                    break;
                }
            }
        }
    }
result
}

fn safe(levels: &Vec<i32>, ignore: usize) -> bool {
    let mut prev: i32;
    let up: bool;
    let start: usize;
    if ignore == 0 {
        if levels[1] == levels[2] {
            return false;
        }
        prev = levels[1];
        up = levels[1] < levels[2];
        start = 2;
    } else if ignore == 1 {
        if levels[0] == levels[2] {
            return false;
        }
        prev = levels[0];
        up = levels[0] < levels[2];
        start = 2;
    } else {
        if levels[0] == levels[1] {
            return false;
        }
        prev = levels[0];
        up = levels[0] < levels[1];
        start = 1;
    };

    for (i, l) in levels[start..].iter().enumerate() {
        if ignore != (i + start) {
            let diff = l - prev;
            if diff.abs() > 3 || diff.abs() == 0 {
                return false;
            }
            if up && diff < 0 {
                return false;
            } else if !up && diff > 0 {
                return false;
            }

            prev = *l;
        }
    }

    return true;
}
