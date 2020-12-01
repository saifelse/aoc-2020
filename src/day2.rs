use regex::bytes::Regex as BytesRegex;
use regex::Regex;
use lazy_static::lazy_static;
use atoi::atoi;

// Hypothesis was that using bytes would be faster than using strings, but this turned out to be false.
// I'm guessing that since aoc_runner has already parsed the input as unicode, we've already lost the
// benefit of using bytes. It's also possible the third-party implementation of atoi may be slower than
// the built-in parse::<i32>
#[aoc(day2, part1, re_bytes)]
pub fn solve_part1_rebytes(input: &str) -> i32 {
    lazy_static! {
        static ref LINE_RE: BytesRegex = BytesRegex::new(r"(?m-u)(\d+)-(\d+) (.): (.+)$").unwrap();
    }
    let mut total: i32 = 0;
    for cap in LINE_RE.captures_iter(input.as_bytes()) {
        let min = atoi::<u32>(&cap[1]).unwrap();
        let max = atoi::<u32>(&cap[2]).unwrap();
        let req = cap[3].iter().next().unwrap();
        let pw = &cap[4];
        let mut count: u32 = 0;
        for b in pw.iter() {
            if b == req {
                count += 1;
            }
        }
        if min <= count && count <= max {
            total += 1;
        }
    }
    return total;
}


#[aoc(day2, part1, re_str)]
pub fn solve_part1_restr(input: &str) -> i32 {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"(?m)(\d+)-(\d+) (.): (.+)$").unwrap();
    }
    let mut total: i32 = 0;
    for cap in LINE_RE.captures_iter(input) {
        let min = cap[1].parse::<i32>().unwrap();
        let max = cap[2].parse::<i32>().unwrap();
        let req = cap[3].chars().next().unwrap();
        let pw = &cap[4];
        let mut count: i32 = 0;
        for b in pw.chars() {
            if b == req {
                count += 1;
            }
        }
        if min <= count && count <= max {
            total += 1;
        }
    }
    return total;
}


#[aoc(day2, part2, re_str)]
pub fn solve_part2_restr(input: &str) -> i32 {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"(?m)(\d+)-(\d+) (.): (.+)$").unwrap();
    }
    let mut total: i32 = 0;
    for cap in LINE_RE.captures_iter(input) {
        let pos1 = cap[1].parse::<i32>().unwrap();
        let pos2 = cap[2].parse::<i32>().unwrap();
        let req = cap[3].bytes().next().unwrap();
        let pw = &cap[4].as_bytes();
        let is_match = (pw[(pos1 - 1) as usize] == req) != (pw[(pos2 - 1) as usize] == req);
        if is_match {
            total += 1;
        }
    }
    return total;
}

// This is slightly slower than solve_part2_restr (1.6578s vs 1.5890)
#[aoc(day2, part2, re_str_count)]
pub fn solve_part2_restrcount(input: &str) -> i32 {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"(?m)(\d+)-(\d+) (.): (.+)$").unwrap();
    }
    LINE_RE.captures_iter(input).filter(|cap| {
        let pos1 = cap[1].parse::<i32>().unwrap();
        let pos2 = cap[2].parse::<i32>().unwrap();
        let req = cap[3].bytes().next().unwrap();
        let pw = &cap[4].as_bytes();
        (pw[(pos1 - 1) as usize] == req) != (pw[(pos2 - 1) as usize] == req)
    }).count() as i32
}

