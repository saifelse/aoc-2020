use std::collections::HashMap;

const OCCUPIED: char = '#';
const EMPTY: char = 'L';
const FLOOR: char = '.';

const ADJ: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut hash_map: HashMap<(i32, i32), char> = HashMap::new();
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            hash_map.insert((i as i32, j as i32), c);
        }
    }
    loop {
        let mut updates: HashMap<(i32, i32), char> = HashMap::new();
        for ((i ,j), c) in hash_map.iter() {
            match *c {
                EMPTY => {
                    let should_become_occupied = ADJ.iter().all(|(di, dj)| {
                        match hash_map.get(&(i + di, j + dj)) {
                            Some(&OCCUPIED) => false,
                            _ => true,
                        }
                    });
                    if should_become_occupied {
                        updates.insert((*i, *j), OCCUPIED);
                    }
                },
                OCCUPIED => {
                    let occupied_count: i32 = ADJ.iter().map(|(di, dj)| {
                        match hash_map.get(&(i + di, j + dj)) {
                            Some(&OCCUPIED) => 1i32,
                            _ => 0i32,
                        }
                    }).sum::<i32>();
                    if occupied_count >= 4 {
                        updates.insert((*i, *j), EMPTY);
                    }
                },
                FLOOR => {},
                _ => panic!(),
            };
        }
        if updates.is_empty() {
            break;
        }
        for ((i, j), c) in updates.iter() {
            hash_map.insert((*i, *j), *c);
        }
    }
    hash_map.iter().filter(|(_, c)| **c == OCCUPIED).count() as i32
}


#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut hash_map: HashMap<(i32, i32), char> = HashMap::new();
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            hash_map.insert((i as i32, j as i32), c);
        }
    }
    loop {
        let mut updates: HashMap<(i32, i32), char> = HashMap::new();
        for ((i ,j), c) in hash_map.iter() {
            match *c {
                EMPTY => {
                    let should_become_occupied = ADJ.iter().all(|(di, dj)| {
                        let mut doi = *di;
                        let mut doj = *dj;
                        while let Some(&FLOOR) = hash_map.get(&(i + doi, j + doj)) {
                            doi += *di;
                            doj += *dj;
                        }
                        match hash_map.get(&(i + doi, j + doj)) {
                            Some(&OCCUPIED) => false,
                            _ => true,
                        }
                    });
                    if should_become_occupied {
                        updates.insert((*i, *j), OCCUPIED);
                    }
                },
                OCCUPIED => {
                    let occupied_count: i32 = ADJ.iter().map(|(di, dj)| {
                        let mut doi = *di;
                        let mut doj = *dj;
                        while let Some(&FLOOR) = hash_map.get(&(i + doi, j + doj)) {
                            doi += di;
                            doj += dj;
                        }
                        match hash_map.get(&(i + doi, j + doj)) {
                            Some(&OCCUPIED) => 1i32,
                            _ => 0i32,
                        }
                    }).sum::<i32>();
                    if occupied_count >= 5 {
                        updates.insert((*i, *j), EMPTY);
                    }
                },
                FLOOR => {},
                _ => panic!(),
            };
        }
        if updates.is_empty() {
            break;
        }
        for ((i, j), c) in updates.iter() {
            hash_map.insert((*i, *j), *c);
        }
    }
    hash_map.iter().filter(|(_, c)| **c == OCCUPIED).count() as i32
}
