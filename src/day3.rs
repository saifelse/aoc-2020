const TREE: u8 = b'#';

struct Map<'a> {
    map: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> Map<'a> {
    fn parse(input: &str) -> Map {
        let map = input.as_bytes();
        let width = input.find('\n').unwrap();
        let height = (map.len() + 1) / (width + 1);
        Map { map, width, height }
    }
    fn traverse(&self, dj: usize, di: usize) -> i64 {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut count = 0;
        while i < self.height - di {
            j = (j + dj) % self.width;
            i = i + di;
            let byte = self.map[i * (self.width + 1) + j];
            if byte == TREE {
                count += 1;
            }
        }
        return count;
    }
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let map = Map::parse(input);
    map.traverse(3, 1)
}


#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let map = Map::parse(input);
    ([(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] as [(usize, usize); 5])
        .iter()
        .map(|(dj, di)| map.traverse(*dj, *di))
        .product()
}
