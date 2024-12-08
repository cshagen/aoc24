use advent24::get_lines;

const FILENAME: &'static str = "./data/d4-input.txt";

pub fn main() {
    println!("Part 1: {}", part1(FILENAME));
    println!("Part 2: {}", part2(FILENAME));
}
fn part1(filename: &str) -> i32 {
    let mut result = 0;
    let mut mat: Vec<Vec<char>> = vec![];
    for line in get_lines(filename) {
        mat.push(line.chars().collect());
    }
    for x in 0..mat[0].len() {
        for y in 0..mat.len() {
            if has_word_right(&mat, x, y, "XMAS") {
                result += 1;
            }
            if has_word_right(&mat, x, y, "SAMX") {
                result += 1;
            }
            if has_word_down(&mat, x, y, "XMAS") {
                result += 1;
            }
            if has_word_down(&mat, x, y, "SAMX") {
                result += 1;
            }
            if has_word_diag_up(&mat, x, y, "XMAS") {
                result += 1;
            }
            if has_word_diag_down(&mat, x, y, "XMAS") {
                result += 1;
            }
            if has_word_diag_up(&mat, x, y, "SAMX") {
                result += 1;
            }
            if has_word_diag_down(&mat, x, y, "SAMX") {
                result += 1;
            }
        }
    }
    result
}
fn part2(filename: &str) -> i32 {
    let mut result = 0;
    let mut mat: Vec<Vec<char>> = vec![];
    for line in get_lines(filename) {
        mat.push(line.chars().collect());
    }
	 for x in 0..mat[0].len() {
        for y in 0..mat.len() {
            if (has_word_diag_down(&mat, x, y, "MAS") || has_word_diag_down(&mat, x, y, "SAM")) &&
                (has_word_diag_up(&mat, x, y + 2, "MAS") || has_word_diag_up(&mat, x, y + 2, "SAM")) {
                  result += 1;
                }
            }
        }
    result 
}

fn has_word_right(mat: &Vec<Vec<char>>, x: usize, y: usize, w: &str) -> bool {
    for (i, c) in w.chars().enumerate() {
        if !has_char(mat, x + i, y, c) {
            return false;
        }
    }
    true
}
fn has_word_down(mat: &Vec<Vec<char>>, x: usize, y: usize, w: &str) -> bool {
    for (i, c) in w.chars().enumerate() {
        if !has_char(mat, x, y + i, c) {
            return false;
        }
    }
    true
}
fn has_word_diag_up(mat: &Vec<Vec<char>>, x: usize, y: usize, w: &str) -> bool {
    0 == w.chars().enumerate()
        .filter(|(i, c)| y < *i || !has_char(mat, x + i, y - i, *c))
        .count()
}
fn has_word_diag_down(mat: &Vec<Vec<char>>, x: usize, y: usize, w: &str) -> bool {
    for (i, c) in w.chars().enumerate() {
        if !has_char(mat, x + i, y + i, c) {
            return false;
        }
    }
    true
}
fn has_char(mat: &Vec<Vec<char>>, x: usize, y: usize, c: char) -> bool {
    x < mat[0].len() && y < mat.len() && mat[y][x] == c
}
