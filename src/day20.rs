use itertools::iproduct;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter;

const DRAGON: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

const FRAME: i32 = 1;

#[derive(Debug, Clone)]
pub struct Tile {
    id: i64,
    top: String,
    bottom: String,
    left: String,
    right: String,
    data: Vec<String>, // TODO: HashMap<(i32, i32), char> is probably saner.
}

impl Tile {
    pub fn rev(s: &String) -> String {
        return s.chars().rev().collect();
    }

    pub fn rot90(&self) -> Tile {
        let h = self.data.len();
        let w = self.data[0].len();
        Tile {
            id: self.id,
            top: Tile::rev(&self.left),
            right: self.top.clone(),
            bottom: Tile::rev(&self.right),
            left: self.bottom.clone(),
            data: (0..w)
                .map(|j| {
                    (0..h)
                        .map(|i| self.data[h - i - 1].as_bytes()[j] as char)
                        .collect()
                })
                .collect(),
        }
    }

    pub fn flipv(&self) -> Tile {
        Tile {
            id: self.id,
            top: self.bottom.clone(),
            right: Tile::rev(&self.right),
            bottom: self.top.clone(),
            left: Tile::rev(&self.left),
            data: self.data.iter().rev().map(|s| s.clone()).collect(),
        }
    }

    pub fn fliph(&self) -> Tile {
        Tile {
            id: self.id,
            top: Tile::rev(&self.top),
            bottom: Tile::rev(&self.bottom),
            right: self.left.clone(),
            left: self.right.clone(),
            data: self.data.iter().map(|s| Tile::rev(s)).collect(),
        }
    }

    pub fn align_to_right(&self, other_tile: &Tile) -> Tile {
        if self.top == other_tile.right {
            self.rot90().fliph()
        } else if self.right == other_tile.right {
            self.fliph()
        } else if self.left == other_tile.right {
            self.clone()
        } else if self.bottom == other_tile.right {
            self.rot90()
        } else if Tile::rev(&self.top) == other_tile.right {
            self.rot90().rot90().rot90()
        } else if Tile::rev(&self.right) == other_tile.right {
            self.rot90().rot90()
        } else if Tile::rev(&self.left) == other_tile.right {
            self.flipv()
        } else if Tile::rev(&self.bottom) == other_tile.right {
            self.rot90().flipv()
        } else {
            panic!("Oopsies");
        }
    }

    pub fn rotations(&self) -> impl Iterator<Item = Tile> {
        let mut curr = self.clone();
        iter::repeat_with(move || {
            let tmp = curr.clone();
            curr = curr.rot90();
            tmp
        })
        .take(4)
    }

    pub fn orientations(&self) -> impl Iterator<Item = Tile> {
        self.rotations().chain(self.fliph().rotations())
    }
}

// Map: tile_id -> Tile
pub fn parse_tiles(input: &str) -> HashMap<i64, Tile> {
    input
        .split("\n\n")
        .map(|x| {
            let mut y = x.lines();
            let tstr = y.next().unwrap().to_string();
            let id = tstr[5..tstr.len() - 1].parse::<i64>().unwrap(); // "Tile XXXX:"
            let mut data: Vec<String> = Vec::new();
            let top: String = y.next().unwrap().to_string();
            data.push(top.clone());
            let mut left: String = top[0..1].to_string();
            let mut right: String = top[top.len() - 1..].to_string();
            let mut bottom: String = "".to_string();
            while let Some(line) = y.next() {
                left += &line[0..1].to_string();
                right += &line[top.len() - 1..].to_string();
                bottom = line.to_string();
                data.push(bottom.clone());
            }
            (
                id,
                Tile {
                    id,
                    top,
                    bottom,
                    left,
                    right,
                    data,
                },
            )
        })
        .collect()
}

// Map: edge chars -> list of tile ids
pub fn collect_tiles_by_edge(tiles_by_id: &HashMap<i64, Tile>) -> HashMap<String, HashSet<i64>> {
    let mut edge_map: HashMap<String, HashSet<i64>> = HashMap::new();
    for tile in tiles_by_id.values() {
        for edge in [&tile.left, &tile.right, &tile.top, &tile.bottom].iter() {
            for hash in [*edge, &edge.chars().rev().collect()].iter() {
                edge_map
                    .entry(hash.to_string())
                    .or_insert(HashSet::new())
                    .insert(tile.id);
            }
        }
    }
    edge_map
}

pub fn find_corners(edge_map: &HashMap<String, HashSet<i64>>) -> Vec<i64> {
    // Collect tiles that have a unique edge.
    let mut counter: HashMap<i64, i64> = HashMap::new();
    for vs in edge_map.values() {
        if vs.len() == 1 {
            for v in vs {
                *counter.entry(*v).or_insert(0) += 1;
            }
        }
    }
    // Filter to tiles that have two unique edges. 4 because we've hashed both orientations of every edge.
    counter
        .iter()
        .filter(|(_, v)| **v == 4)
        .map(|(k, _)| *k as i64)
        .collect()
}

struct Grid {
    cells: HashMap<(i32, i32), char>,
    width: i32,
    height: i32,
}

impl Grid {
    pub fn find_all(&self, query: &Tile) -> Option<HashSet<(i32, i32)>> {
        query.orientations().find_map(|q| {
            let matched_cells: HashSet<(i32, i32)> = iproduct!((0..self.height), (0..self.width))
                .filter_map(|(i, j)| self.find_at(&q, (i, j)))
                .flat_map(|vec| vec)
                .collect();
            if matched_cells.is_empty() {
                return None;
            }
            Some(matched_cells)
        })
    }

    pub fn find_at(&self, query: &Tile, offset: (i32, i32)) -> Option<Vec<(i32, i32)>> {
        query
            .data
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != ' ')
                    .map(move |(j, c)| {
                        let coord = (i as i32 + offset.0, j as i32 + offset.1);
                        match self.cells.get(&coord) {
                            Some(src_c) if *src_c == c => Some(coord),
                            _ => None,
                        }
                    })
            })
            // http://xion.io/post/code/rust-iter-patterns.html
            .collect()
    }
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let tiles_by_id = parse_tiles(input);
    let map: HashMap<String, HashSet<i64>> = collect_tiles_by_edge(&tiles_by_id);
    find_corners(&map).iter().product()
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &str) -> usize {
    let tiles_by_id = parse_tiles(input);
    let n = (tiles_by_id.len() as f64).sqrt() as i32;
    let edge_map: HashMap<String, HashSet<i64>> = collect_tiles_by_edge(&tiles_by_id);

    // Arbitrarily pick one corner to occupy (0, 0)
    let corners = find_corners(&edge_map);
    let tile_id = corners.first().unwrap();
    let corner_tile = tiles_by_id.get(tile_id).unwrap();
    let mut tiles_arrangement: HashMap<(i32, i32), Tile> = HashMap::new();
    for y in 0..n {
        for x in 0..n {
            match (y, x) {
                (0, 0) => {
                    // Make sure that corner_tile's edge occupies top-left.
                    let top_match = edge_map.get(&corner_tile.top).unwrap().len() == 1;
                    let left_match = edge_map.get(&corner_tile.left).unwrap().len() == 1;
                    let flipped_tile = match (top_match, left_match) {
                        (true, true) => corner_tile.clone(),
                        (false, true) => corner_tile.flipv(),
                        (true, false) => corner_tile.fliph(),
                        (false, false) => corner_tile.flipv().fliph(),
                    };
                    tiles_arrangement.insert((0, 0), flipped_tile);
                }
                (_, 0) => {
                    let prev = tiles_arrangement.get(&(y - 1, x)).unwrap();
                    let next_tile_id = edge_map
                        .get(&prev.bottom)
                        .unwrap()
                        .iter()
                        .find(|id| **id != prev.id)
                        .unwrap();
                    let next_tile = tiles_by_id.get(next_tile_id).unwrap();
                    let aligned_tile = next_tile
                        .align_to_right(&prev.flipv().rot90())
                        .rot90()
                        .rot90()
                        .rot90()
                        .flipv();
                    assert!(
                        aligned_tile.top == prev.bottom,
                        "Mismatch for first file in new row"
                    );
                    tiles_arrangement.insert((y, x), aligned_tile);
                }
                _ => {
                    let prev = tiles_arrangement.get(&(y, x - 1)).unwrap();
                    let next_tile_id = edge_map
                        .get(&prev.right)
                        .unwrap()
                        .iter()
                        .find(|id| **id != prev.id)
                        .unwrap();
                    let next_tile = tiles_by_id.get(next_tile_id).unwrap();
                    let aligned_tile = next_tile.align_to_right(prev);
                    assert!(
                        aligned_tile.left == prev.right,
                        "Mismatch for next tile in row"
                    );
                    tiles_arrangement.insert((y, x), aligned_tile);
                }
            }
        }
    }

    // Join grid into a single unified 2d grid.
    let m = corner_tile.top.len() as i32;
    let cells: HashMap<(i32, i32), char> = iproduct!((0..n), (0..n), (FRAME..(m - FRAME)), (FRAME..(m - FRAME))).map(
        |(y, x, i, j)| {
            let tile = tiles_arrangement.get(&(y, x)).unwrap();
            let coord = (
                (y * (m - FRAME * 2) + (i - FRAME)),
                (x * (m - FRAME * 2) + (j - FRAME)),
            );
            (coord, tile.data[i as usize].as_bytes()[j as usize] as char)
        }
    ).collect();
    
    let w = n * (m - FRAME * 2);
    let g = Grid {
        cells,
        width: w,
        height: w,
    };

    // For debugging, print out the 2d grid.
    for y in 0..w {
        println!(
            "{}",
            (0..w)
                .map(|x| g.cells.get(&(y, x)).unwrap())
                .collect::<String>()
        );
    }

    let dragon = Tile {
        id: 0,
        // HACK: Re-using structure so we can use rotate / flip
        top: String::new(),
        bottom: String::new(),
        left: String::new(),
        right: String::new(),
        data: DRAGON.iter().map(|s| s.to_string()).collect(),
    };

    // Find an orientation that yields dragons.
    let dragon_cells = g.find_all(&dragon).unwrap();
    let non_dragon_cells = g
        .cells
        .iter()
        .filter(|(coord, c)| **c == '#' && !dragon_cells.contains(coord))
        .count();
    non_dragon_cells
}
