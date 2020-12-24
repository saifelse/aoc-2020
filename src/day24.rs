use lazy_static::lazy_static;
use num_complex::Complex;
use regex::Regex;
use std::collections::HashSet;

// TODO: Can we refactor some of the code from day17?
// A hexagonal grid can be represented by tiling the two vectors (1, -1) and (-1, 1).
// Position: starting tile.
const O: Complex<i32> = Complex::new(0, 0);
// Directions: We establish 6 "unit" directions from a given tile, built from (1, -1) and (-1, 1).
const W: Complex<i32> = Complex::new(-2, 0);
const E: Complex<i32> = Complex::new(2, 0);
const NW: Complex<i32> = Complex::new(-1, 1);
const NE: Complex<i32> = Complex::new(1, 1);
const SW: Complex<i32> = Complex::new(-1, -1);
const SE: Complex<i32> = Complex::new(1, -1);
const ADJ: [Complex<i32>; 6] = [W, E, NW, NE, SW, SE];

pub fn start_state(input: &str) -> HashSet<Complex<i32>> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"w|e|nw|ne|sw|se").unwrap();
    }
    let mut black_set: HashSet<Complex<i32>> = HashSet::new();
    for line in input.lines() {
        let x = LINE_RE.find_iter(line).fold(O, |acc, text| {
            acc + match text.as_str() {
                "w" => W,
                "e" => E,
                "nw" => NW,
                "ne" => NE,
                "sw" => SW,
                "se" => SE,
                _ => panic!("unexpected direction"),
            }
        });
        if black_set.contains(&x) {
            black_set.remove(&x);
        } else {
            black_set.insert(x);
        }
    }
    black_set
}

// If a tile is black and exactly 0 or 2+ of its neighbors are also black, the tile becomes white.
pub fn compute_to_flip_white(black_set: &HashSet<Complex<i32>>) -> Vec<Complex<i32>> {
    black_set
        .iter()
        .filter(|v| {
            let c = ADJ
                .iter()
                .filter(|dv| black_set.contains(&(*v + *dv)))
                .take(3) // "short-circuit" to true if we've already exceeded 2.
                .count();
            c == 0 || c > 2
        })
        .map(|v| v.clone())
        .collect()
}

// If a tile is white but exactly 2 of its neighbors are black, the tile becomes black.
pub fn compute_to_flip_black(black_set: &HashSet<Complex<i32>>) -> Vec<Complex<i32>> {
    black_set
        .iter()
        // Find all black-adjacent...
        .flat_map(|v| ADJ.iter().map(move |dv| v + dv))
        // ...but white cells
        .filter(|v| !black_set.contains(v))
        // and deduplicate.
        .collect::<HashSet<Complex<i32>>>()
        .iter()
        // Then filter to those that meet the flip criteria.
        .filter(|v| {
            let c = ADJ
                .iter()
                .filter(|dv| black_set.contains(&(*v + *dv)))
                .take(3) // "short-circuit" to false if we've already exceeded 2.
                .count();
            c == 2
        })
        .map(|v| v.clone())
        .collect()
}

pub fn run(black_set: &mut HashSet<Complex<i32>>) {
    let to_white: Vec<Complex<i32>> = compute_to_flip_white(black_set);
    let to_black: Vec<Complex<i32>> = compute_to_flip_black(black_set);
    for x in to_black {
        black_set.insert(x);
    }
    for x in to_white {
        black_set.remove(&x);
    }
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &str) -> usize {
    start_state(input).iter().count()
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut black_set: HashSet<Complex<i32>> = start_state(input);
    for _ in 0..100 {
        run(&mut black_set);
    }
    black_set.iter().count()
}
