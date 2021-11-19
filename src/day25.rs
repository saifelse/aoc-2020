const MOD: i64 = 20201227;

#[aoc(day25, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let mut iter = input.lines();
    let pk1 = iter.next().unwrap().parse::<i64>().unwrap();
    let pk2 = iter.next().unwrap().parse::<i64>().unwrap();
    let l1 = compute_loop_size(7, pk1);
    let l2 = compute_loop_size(7, pk2);
    let e1 = transform(pk1, l2);
    let e2 = transform(pk2, l1);
    assert!(e1 == e2);
    e1
}


pub fn compute_loop_size(subject: i64, public_key: i64) -> i64 {
    let mut v = 1;
    let mut i = 0;
    while v != public_key {
        v = (v * subject).rem_euclid(MOD);
        i += 1;
    }
    i
}
pub fn transform(subject: i64, loop_size: i64) -> i64 {
    let mut v = 1;
    for _ in 0..loop_size {
        v = (v * subject).rem_euclid(MOD);
    }
    v
}