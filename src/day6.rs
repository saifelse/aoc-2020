use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::collections::HashMap;

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    lazy_static! {
        // NB: It looks like aoc_runner is stripping the trailing \n\n whitespace from `input`.
        static ref GROUP_RE: Regex = Regex::new(r"(?s)(.+?)(\n\n|$)").unwrap();
        static ref ANSWER_RE: Regex = Regex::new(r"[a-z]").unwrap();
    }
    let mut sum = 0;
    for caps in GROUP_RE.captures_iter(input) {
        let mut hash_set: HashSet<&str> = HashSet::new();
        for mat in ANSWER_RE.find_iter(&caps[1]) {
            hash_set.insert(mat.as_str());
        }
        sum += hash_set.len();
    }
    sum
}

 
#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    lazy_static! {
        static ref GROUP_RE: Regex = Regex::new(r"(?s)(.+?)(\n\n|$)").unwrap();
        static ref ANSWER_RE: Regex = Regex::new(r"[a-z]").unwrap();
    }
    let mut sum = 0;
    for caps in GROUP_RE.captures_iter(input) {
        let mut hash_map: HashMap<&str, i32> = HashMap::new();
        let mut people = 0;
        for line in caps[1].lines() {
            people += 1;
            for mat in ANSWER_RE.find_iter(line) {
                let key = mat.as_str();
                let val = match hash_map.get(key) {
                    Some(v) => *v,
                    None => 0,
                };
                hash_map.insert(key, val + 1);
            }
        }
        sum += hash_map.iter().filter(|(_, v)| **v == people).count();
    }
    sum
}

// 30x faster
#[aoc(day6, part1, ch)]
pub fn solve_part1_ch(input: &str) -> usize {
    let mut sum = 0;
    let mut counts: [i32; 26] = [0; 26];
    for line in input.lines().chain(std::iter::once("")) {
        match line {
            "" => {
                sum += counts.iter().filter(|v| **v > 0).count();
                counts = [0; 26];
            },
            _ => {
                for c in line.chars() {
                    counts[(c as u8 - b'a') as usize] += 1;
                }
            }
        }
    }
    sum
}


// 30x faster
#[aoc(day6, part2, ch)]
pub fn solve_part2_ch(input: &str) -> usize {
    let mut sum = 0;
    let mut counts: [i32; 26] = [0; 26];
    let mut people = 0;
    for line in input.lines().chain(std::iter::once("")) {
        match line {
            "" => {
                sum += counts.iter().filter(|v| **v == people).count();
                people = 0;
                counts = [0; 26];
            },
            _ => {
                people += 1;
                for c in line.chars() {
                    counts[(c as u8 - b'a') as usize] += 1;
                }
            }
        }
    }
    sum
}
