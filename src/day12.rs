use regex::Regex;
use lazy_static::lazy_static;
use num_complex::Complex;

//         N (+i)
//           |
// (-1) W ---+--- E (+1)
//           |
//        S (-1)
// Position
const O: Complex<i32> = Complex::new(0, 0);
// Unit rotations (multiplicative)
const L: Complex<i32> = Complex::new(0, 1);
const R: Complex<i32> = Complex::new(0, -1);
// Unit movements (additive)
const N: Complex<i32> = Complex::new(0, 1);
const S: Complex<i32> = Complex::new(0, -1);
const E: Complex<i32> = Complex::new(1, 0);
const W: Complex<i32> = Complex::new(-1, 0);


#[aoc(day12, part1)]
pub fn solve_part1(input: &str) -> i32 {
    lazy_static! {
        // Named capture groups were 20% slower, but oh well. 
        static ref LINE_RE: Regex = Regex::new(r"(?P<act>[NSEWLRF])(?P<val>\d+)$").unwrap();
    }
    input.lines().fold((O, E), |(pos, dir), l| {
        let caps = LINE_RE.captures(l).unwrap();
        let val = caps.name("val").unwrap().as_str().parse::<i32>().unwrap();
        match caps.name("act").unwrap().as_str() {
            "N" => (pos + N * val, dir),
            "S" => (pos + S * val, dir),
            "E" => (pos + E * val, dir),
            "W" => (pos + W * val, dir),
            "L" => (pos, dir * L.powi(val / 90)),
            "R" => (pos, dir * R.powi(val / 90)),
            "F" => (pos + dir * val, dir),
            _ => panic!(),
        }
    }).0.l1_norm()
}


#[aoc(day12, part2)]
pub fn solve_part2(input: &str) -> i32 {
    lazy_static! {
        // Named capture groups were 20% slower, but oh well. 
        static ref LINE_RE: Regex = Regex::new(r"(?P<act>[NSEWLRF])(?P<val>\d+)$").unwrap();
    }
    input.lines().fold((O, 10 * E + 1 * N), |(pos, off), l| {
        let caps = LINE_RE.captures(l).unwrap();
        let val = caps.name("val").unwrap().as_str().parse::<i32>().unwrap();
        match caps.name("act").unwrap().as_str() {
            "N" => (pos, off + N * val),
            "S" => (pos, off + S * val),
            "E" => (pos, off + E * val),
            "W" => (pos, off + W * val),
            "L" => (pos, off * L.powi(val / 90)),
            "R" => (pos, off * R.powi(val / 90)),
            "F" => (pos + off * val, off),
            _ => panic!(),
        }
    }).0.l1_norm()
}
