use std::char;

// Good enough for part1... but very slow.
// NB: std::time::Instant can be used to time sections of code.
pub fn naive_run(v: &mut Vec<u32>, idx: usize) -> usize {
    let max_v = v.len() as u32;
    let current = v[idx];

    // Remove 3 items at the following index.
    let r = (idx + 1).rem_euclid(v.len());
    let re = if (r + 3) <= v.len() { r + 3 } else { v.len() };
    let removed: Vec<u32> = v.splice(r..re, std::iter::empty()).collect();
    let still_needed = 3 - removed.len();
    let more_removed: Vec<u32> = v.splice(0..(still_needed), std::iter::empty()).collect();

    // Find the next destination.
    let mut dest = current - 1;
    if dest == 0 {
        dest = max_v;
    }
    while removed.contains(&dest) || more_removed.contains(&dest) {
        dest -= 1;
        if dest == 0 {
            dest = max_v;
        }
    }

    let dest_idx = v.iter().position(|&r| r == dest).unwrap();

    // Insert the removed items after the destination
    let ins_idx = dest_idx + 1;
    v.splice(
        ins_idx..ins_idx,
        removed.iter().chain(more_removed.iter()).map(|x| *x),
    );

    // Return the next index
    let curr_idx = v.iter().position(|&r| r == current).unwrap();
    (curr_idx + 1).rem_euclid(v.len())
}

#[derive(Debug)]
struct Node<T> {
    // TODO: Can we model this as a doubly-linked list? It's complex :grimacing:
    // https://rust-unofficial.github.io/too-many-lists/fourth-final.html
    elem: T,
    next: T,
    prev: T,
}

struct NodeRing {
    // TODO: Could we replace i64 with a generic?
    node_map: Vec<Node<i64>>,
    curr: i64,
}

impl NodeRing {
    // TODO: Rewrite this using FromIterator
    pub fn new(vecs: Vec<i64>) -> NodeRing {
        let mut node_map = Vec::with_capacity(vecs.len());
        for (i, v) in vecs.iter().enumerate() {
            let node = Node {
                elem: *v,
                next: vecs[(i + 1).rem_euclid(vecs.len())],
                prev: vecs[(i - 1).rem_euclid(vecs.len())],
            };
            node_map.push(node);
        }
        // Sort so that to_idx(a.elem) gives the correct index
        node_map.sort_by(|a, b| a.elem.partial_cmp(&b.elem).unwrap());

        NodeRing {
            // Starts at the first node specified
            curr: *vecs.first().unwrap(),
            // Doubly link adjacent nodes
            node_map,
        }
    }
    pub fn to_idx(val: i64) -> usize {
        return (val - 1) as usize;
    }

    pub fn get_next(&self, curr: i64) -> i64 {
        return self.find(curr).next;
    }

    pub fn find_mut(&mut self, val: i64) -> &mut Node<i64> {
        &mut self.node_map[NodeRing::to_idx(val)]
    }

    pub fn find(&self, val: i64) -> &Node<i64> {
        &self.node_map[NodeRing::to_idx(val)]
    }

    pub fn run(&mut self) {
        // Assumption that node_map contains 1..N
        let max_v = self.node_map.len() as i64;

        // Splice out 3 nodes: curr -> (m1 -> m2 -> m3 ->) next
        let m1 = self.get_next(self.curr);
        let m2 = self.get_next(m1);
        let m3 = self.get_next(m2);
        let next = self.get_next(m3);
        self.find_mut(self.curr).next = next;
        self.find_mut(next).prev = self.curr;

        // Find the destination node: first node less than curr.
        let mut dest = self.curr;
        loop {
            // dest is 1-indexed:
            //   -1 to become 0-indexed
            //   -1 to find the predecessor
            //   % to ensure -1 wraps around to (len - 1)
            //   +1 to become 1-indexed again.
            dest = (dest - 2).rem_euclid(max_v) + 1;
            if ![m1, m2, m3].contains(&dest) {
                break;
            }
        }
        // Find the successor to dest for splicing.
        let destn = self.get_next(dest);

        // Splice in the 3 removed nodes: dest -> (m1 -> m2 -> m3 ->) destn
        self.find_mut(dest).next = m1;
        self.find_mut(m1).prev = dest;
        self.find_mut(m3).next = destn;
        self.find_mut(destn).prev = m3;

        // The next iteration repeats with `next`.
        self.curr = next;
    }

    // TODO: Rewrite this using IntoIterator / Iterator
    pub fn get_list(&self, start_val: i64) -> Vec<i64> {
        let mut v: Vec<i64> = Vec::new();
        // Start at `start_val` and loop until it's hit again.
        let mut n = start_val;
        loop {
            v.push(n);
            n = self.get_next(n);
            if n == start_val {
                break;
            }
        }
        v
    }
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &str) -> String {
    let mut v: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut idx = 0;
    for _ in 0..100 {
        idx = naive_run(&mut v, idx);
    }
    let one_idx = v.iter().position(|&r| r == 1).unwrap();
    v[one_idx + 1..v.len()]
        .iter()
        .chain(v[0..one_idx].iter())
        .map(|d| char::from_digit(*d, 10).unwrap())
        .collect()
}

#[aoc(day23, part1, linked_vec)]
pub fn solve_part1_linked_vec(input: &str) -> String {
    let v: Vec<i64> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();
    let mut l = NodeRing::new(v);
    for _ in 0..100 {
        l.run();
    }
    l.get_list(1)
        .iter()
        .skip(1) // output excludes the starting 1
        .map(|d| char::from_digit(*d as u32, 10).unwrap())
        .collect()
}

#[aoc(day23, part2, linked_vec)]
pub fn solve_part2_linked_vec(input: &str) -> i64 {
    let v: Vec<i64> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .chain(10..1000001)
        .collect();
    let mut l = NodeRing::new(v);
    for _ in 0..10000000 {
        l.run();    
    }
    let c1 = l.get_next(1);
    let c2 = l.get_next(c1);
    c1 * c2
}
