#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let width = input.find('\n').unwrap();
    let map = input.as_bytes();
    let height = (map.len() + 1) / (width + 1);
    let tree = "#".as_bytes().iter().next().unwrap();
    let mut i: usize= 0;
    let mut j: usize = 0;
    let mut count = 0;
    while i < height - 1 {
        j = (j + 3) % width;
        i = i + 1;
        let byte = map[i * (width + 1) + j];
        if byte == *tree {
            count += 1;
        }
    }
    return count;
}

fn traverse(dj: usize, di: usize, width: usize, height: usize, map: &[u8], obstacle: &u8) -> u32 {
    let mut i: usize= 0;
    let mut j: usize = 0;
    let mut count = 0;
    while i < height - di {
        j = (j + dj) % width;
        i = i + di;
        let byte = map[i * (width + 1) + j];
        if byte == *obstacle {
            count += 1;
        }
    }
    return count;
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let width = input.find('\n').unwrap();
    let map = input.as_bytes();
    let height = (map.len() + 1) / (width + 1);
    let tree = "#".as_bytes().iter().next().unwrap();
    return 
        1
        * traverse(1, 1, width, height, &map, tree)
        * traverse(3, 1, width, height, &map, tree)
        * traverse(5, 1, width, height, &map, tree)
        * traverse(7, 1, width, height, &map, tree)
        * traverse(1, 2, width, height, &map, tree);
}

