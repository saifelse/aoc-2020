use itertools::iproduct;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::iter;

const WILDCARD: char = ' ';
const MARK: char = '#';
const DRAGON: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

const FRAME: usize = 1;

type TileId = i64;

#[derive(Debug, Clone)]
pub struct Tile {
    id: TileId,
    top: String,
    bottom: String,
    left: String,
    right: String,
    data: Vec<String>, // TODO: HashMap<(i32, i32), char> is probably cleaner.
}

impl Tile {
    pub fn from_data(id: TileId, data: Vec<String>) -> Tile {
        // Kindof gross... we could compute these on the fly...
        let top = data.first().unwrap().clone();
        let bottom = data.last().unwrap().clone();
        let mut iter = data.iter();
        let mut left: String = String::new();
        let mut right: String = String::new();
        while let Some(line) = iter.next() {
            left += &line[0..1].to_string();
            right += &line[line.len() - 1..].to_string();
        }
        return Tile {
            id,
            top,
            left,
            right,
            bottom,
            data,
        };
    }

    // Map: tile_id -> Tile
    pub fn parse_tiles(input: &str) -> HashMap<TileId, Tile> {
        input
            .split("\n\n")
            .map(|x| {
                let mut lines = x.lines();
                let id_str = lines.next().unwrap().to_string(); // "Tile XXXX:"
                let id = id_str[5..id_str.len() - 1].parse::<TileId>().unwrap();
                let data: Vec<String> = lines.map(|s| s.to_string()).collect();
                (id, Tile::from_data(id, data))
            })
            .collect()
    }

    pub fn width(&self) -> usize {
        self.data.first().unwrap().len()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn rev(s: &String) -> String {
        return s.chars().rev().collect();
    }

    pub fn at(&self, i: usize, j: usize) -> char {
        return self.data[i].as_bytes()[j] as char;
    }

    pub fn enumerate_chars(&self) -> impl Iterator<Item = ((usize, usize), char)> {
        let x = self.clone(); // TODO: Correct way to specify lifetime, so we don't need this clone
        iproduct!((0..x.height()), (0..x.width())).map(move |(i, j)| ((i, j), x.at(i, j)))
    }

    pub fn rotate(&self) -> Tile {
        let h = self.height();
        let w = self.width();
        Tile::from_data(
            self.id,
            (0..w)
                .map(|j| (0..h).map(|i| self.at(h - i - 1, j)).collect())
                .collect(),
        )
    }

    pub fn flip(&self) -> Tile {
        Tile::from_data(self.id, self.data.iter().map(|s| Tile::rev(s)).collect())
    }

    pub fn rotations(&self) -> impl Iterator<Item = Tile> {
        let mut curr = self.clone();
        iter::repeat_with(move || {
            let tmp = curr.clone();
            curr = curr.rotate();
            tmp
        })
        .take(4)
    }

    pub fn orientations(&self) -> impl Iterator<Item = Tile> {
        self.rotations().chain(self.flip().rotations())
    }
}


struct Grid {
    cells: HashMap<(usize, usize), char>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn find_all(&self, query: &Tile) -> Option<HashSet<(usize, usize)>> {
        query.orientations().find_map(|q| {
            let matched_cells: HashSet<(usize, usize)> =
                iproduct!((0..self.height), (0..self.width))
                    .filter_map(|(i, j)| self.find_at(&q, (i, j)))
                    .flat_map(|vec| vec)
                    .collect();
            if matched_cells.is_empty() {
                return None;
            }
            Some(matched_cells)
        })
    }

    pub fn find_at(&self, query: &Tile, offset: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        query
            .enumerate_chars()
            .filter(|(_, c)| *c != WILDCARD)
            .map(move |((i, j), c)| {
                let coord = (i + offset.0, j + offset.1);
                match self.cells.get(&coord) {
                    Some(src_c) if *src_c == c => Some(coord),
                    _ => None,
                }
            })
            // http://xion.io/post/code/rust-iter-patterns.html
            .collect()
    }

    pub fn from_tiles(tiles_arrangement: &HashMap<(usize, usize), Tile>) -> Grid {
        let k = tiles_arrangement.keys().max().unwrap();
        let dim_w = k.0 + 1;
        let dim_h = k.1 + 1;
        let t = &tiles_arrangement[k];
        let tile_h = t.height() - 2 * FRAME;
        let tile_w = t.width() - 2 * FRAME;
        Grid {
            height: dim_h * tile_h,
            width: dim_w * tile_w,
            cells: iproduct!((0..dim_h), (0..dim_w), (0..tile_h), (0..tile_w))
                .map(|(y, x, i, j)| {
                    let tile = &tiles_arrangement[&(y, x)];
                    let coord = ((y * tile_h + i), (x * tile_w + j));
                    (coord, tile.at(i + FRAME, j + FRAME))
                })
                .collect(),
        }
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            (0..self.height)
                .map(|y| {
                    (0..self.width)
                        .map(|x| &self.cells[(&(y, x))])
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n")
                .as_str(),
        )
    }
}


struct Solver;

impl Solver {
    // Map: edge chars -> list of tile ids
    pub fn collect_tiles_by_edge(
        tiles_by_id: &HashMap<TileId, Tile>,
    ) -> HashMap<String, HashSet<TileId>> {
        let mut edge_map: HashMap<String, HashSet<TileId>> = HashMap::new();
        for tile in tiles_by_id.values() {
            // TODO: Would be nice if we could do: `for edge in tile.edges()`
            for edge in [&tile.left, &tile.right, &tile.top, &tile.bottom].iter() {
                for hash in [*edge, &Tile::rev(edge)].iter() {
                    edge_map
                        .entry(hash.to_string())
                        .or_insert(HashSet::new())
                        .insert(tile.id);
                }
            }
        }
        edge_map
    }

    pub fn find_corners(edge_map: &HashMap<String, HashSet<TileId>>) -> Vec<TileId> {
        // Collect tiles that have a unique edge.
        let mut counter: HashMap<TileId, i32> = HashMap::new();
        for vs in edge_map.values() {
            if vs.len() == 1 {
                for v in vs {
                    *counter.entry(*v).or_insert(0) += 1;
                }
            }
        }
        // Filter to tiles that have two unique edges. 4 because we've hashed both orientations of every edge.
        // NB: Assumes that no edge is palindromic.
        counter
            .iter()
            .filter(|(_, v)| **v == 4)
            .map(|(k, _)| *k)
            .collect()
    }

    pub fn layout_tiles(
        edge_map: &HashMap<String, HashSet<TileId>>,
        tiles_by_id: &HashMap<TileId, Tile>,
    ) -> HashMap<(usize, usize), Tile> {
        let n = (tiles_by_id.len() as f64).sqrt() as usize; // Arrange into an `n x n` grid
        let mut tiles_arrangement: HashMap<(usize, usize), Tile> = HashMap::new();
        for (y, x) in iproduct!((0..n), (0..n)) {
            let aligned_tile = match (y, x) {
                (0, 0) => {
                    // Arbitrarily pick one corner to occupy (0, 0)
                    let corners = Solver::find_corners(edge_map);
                    let next_tile_id = corners.first().unwrap();
                    let next_tile = &tiles_by_id[next_tile_id];
                    next_tile
                        .orientations()
                        .find(|t| [&t.top, &t.left].iter().all(|e| edge_map[*e].len() == 1))
                }
                (_, 0) => {
                    let prev_tile = &tiles_arrangement[&(y - 1, x)];
                    let next_tile_id = edge_map[&prev_tile.bottom]
                        .iter()
                        .find(|id| **id != prev_tile.id)
                        .unwrap();
                    let next_tile = &tiles_by_id[next_tile_id];
                    next_tile.orientations().find(|t| t.top == prev_tile.bottom)
                }
                _ => {
                    let prev_tile = &tiles_arrangement[&(y, x - 1)];
                    let next_tile_id = edge_map[&prev_tile.right]
                        .iter()
                        .find(|id| **id != prev_tile.id)
                        .unwrap();
                    let next_tile = &tiles_by_id[next_tile_id];
                    next_tile.orientations().find(|t| t.left == prev_tile.right)
                }
            };
            tiles_arrangement.insert((y, x), aligned_tile.unwrap());
        }
        tiles_arrangement
    }
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let tiles_by_id = Tile::parse_tiles(input);
    let edge_map: HashMap<String, HashSet<TileId>> = Solver::collect_tiles_by_edge(&tiles_by_id);
    Solver::find_corners(&edge_map).iter().product()
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &str) -> usize {
    let tiles_by_id = Tile::parse_tiles(input);
    let edge_map: HashMap<String, HashSet<TileId>> = Solver::collect_tiles_by_edge(&tiles_by_id);
    let tiles_arrangement = Solver::layout_tiles(&edge_map, &tiles_by_id);
    let g = Grid::from_tiles(&tiles_arrangement);
    println!("Grid: {:?}", g);
    let dragon = Tile::from_data(-1, DRAGON.iter().map(|s| s.to_string()).collect());
    let dragon_cells = g.find_all(&dragon).unwrap();
    let non_dragon_cells = g
        .cells
        .iter()
        .filter(|(coord, c)| **c == MARK && !dragon_cells.contains(coord))
        .count();
    non_dragon_cells
}
