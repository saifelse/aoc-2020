#[aoc(day13, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut iter = input.lines();
    let earliest = iter.next().unwrap().parse::<i32>().unwrap();
    // let buses: Vec<i32> = iter.next().unwrap().split(',').filter(|b| *b != "x").map(|b| b.parse::<i32>().unwrap()).collect();
    let buses: Vec<i32> = iter
        .next()
        .unwrap()
        .split(',')
        .filter_map(|b| match b.parse::<i32>() {
            Ok(bus_id) => Some(bus_id),
            Err(_) => None,
        })
        .collect();
    // Technically the `1` below should be conditional based on if earliest % b == 0.
    let div: (i32, i32) = buses
        .iter()
        .map(|b| ((earliest / b + 1) * b, *b))
        .min()
        .unwrap();
    div.1 * (div.0 - earliest)
}

// Successive remainders from each iteration of the extended euclidean algorithm.
struct R {
    r: i64,
    s: i64,
    t: i64,
    i: i64,
}

pub struct BezoutCoeff {
    x: i64,
    y: i64,
}

/// Solve the diophantine equation a * x + b * y = gcd(a, b) for given a, b.
pub fn solve_bezouts_identity(a: i64, b: i64) -> BezoutCoeff {
    // This implementation uses the extended euclidean algorithm.
    // See: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
    // See: http://math0.wvstateu.edu/~baker/cs405/code/Euclid.html
    // See: https://en.wikipedia.org/wiki/Bézout%27s_identity
    let mut r1 = R {r: a, s: 1, t: 0, i: -1};
    let mut r2 = R {r: b, s: 0, t: 1, i: 1};
    while r2.r != 0 {
        let q = r1.r / r2.r;
        let rn = R {
            r: r1.r - q * r2.r,
            t: q * r2.t + r1.t,
            s: q * r2.s + r1.s,
            i: r1.i,
        };
        r1 = r2;
        r2 = rn;
    }
    BezoutCoeff {
        x: -r1.i * r1.s,
        y: r1.i * r1.t,
    }
}

/// Multiply two numbers (a, b) in a given modulus (m).
///
/// a and m must be no larger than (i64::MAX / 2).
///
/// The naive implementation of (a * b).rem_euclid(m) works in many cases,
/// but will be incorrect for sufficiently large a and b such that the
/// product overflows i64::MAX.
///
pub fn mult(a: i64, b: i64, m: i64) -> i64 {
    // Accumulate the result of a * b through repeated additions.
    let mut r = 0;
    // Ensure `a` and `b` are positive.
    let mut a = a.rem_euclid(m);
    let mut b = b.rem_euclid(m);
    // Iteratively decrement `b` until the result accumulates in `r`.
    while b != 0 {
        // If `b` is odd, add `a` once, so that we can decrement `b` to be even.
        if b & 1 == 1 {
            // TODO: How are we confident that r won't overflow?
            r += a;
            b -= 1;
        // If `b` is even, we can rewrite (a * b) as (2 * a) * (b / 2), so that we can halve b,
        // ensuring runtime is ~O(lg B) instead of O(B).
        } else {
            a = (a * 2) % m;
            b >>= 1;
        }
    }
    r.rem_euclid(m)
}

pub struct ChineseRemainder {
    r: i64, // remainder
    m: i64, // modulus
}

/// Solve the congruence {R = crs[i].r (modulo crs[i].m) for all i}.
///
/// crs[i].m must all be coprime.
///
/// See: https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Existence_(constructive_proof)
/// NB: https://rust-lang.github.io/api-guidelines/flexibility.html#c-generic
pub fn solve_chinese_remainder_theorem<I: IntoIterator<Item = ChineseRemainder>>(crs: I) -> ChineseRemainder {
    crs.into_iter().fold(ChineseRemainder {r: 0, m: 1}, |a, b| {
        let coeffs = solve_bezouts_identity(a.m, b.m);
        // Since a.m and b.m are coprimes, the gcd(a.m, b.m) = 1, so we have a solution for: a.m * x + b.m * y = 1
        let m = a.m * b.m;
        // NB: The naive construction of r as follow may hit overflow errors.
        // let r = (b.r * a.m * coeffs.x + a.r * b.m * coeffs.y).rem_euclid(m);
        let r = (mult(mult(b.r, a.m, m), coeffs.x, m) + mult(mult(a.r, b.m, m), coeffs.y, m)).rem_euclid(m);
        // (r, m) is mathematically expected to solve the congruence, but if we wanted to sanity check:
        // if r.rem_euclid(a.m) != a.r || r.rem_euclid(b.m) != b.r {
        //     panic!("CRT failed, likely an overflow?");
        // }
        ChineseRemainder {r, m}
    })
}

pub fn solve_chinese_remainder_theorem_brute<I: IntoIterator<Item = ChineseRemainder>>(crs: I) -> ChineseRemainder {
    crs.into_iter().fold(ChineseRemainder {r: 0, m: 1}, |a, b| {
        let mut r = a.r;
        while r % b.m != b.r {
            r += a.m;
        }
        ChineseRemainder {r, m: a.m * b.m}
    })
}


#[aoc(day13, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let mut iter = input.lines();
    iter.next(); // ignore your "earliest timestamp" for this part.
    let bus_data: Vec<(usize, i64)> = iter
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, b)| *b != "x")
        .map(|(i, b)| (i, b.parse::<i64>().unwrap()))
        .collect();
    solve_chinese_remainder_theorem(
        bus_data.iter().map(|(i, b)| ChineseRemainder {r: (b - (*i as i64)).rem_euclid(*b), m: *b})
    ).r
}

// NB: This is 15% faster than the more complex number theory solution.
#[aoc(day13, part2, brute)]
pub fn solve_part2_brute(input: &str) -> i64 {
    let mut iter = input.lines();
    iter.next(); // ignore your "earliest timestamp" for this part.
    let bus_data: Vec<(usize, i64)> = iter
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, b)| *b != "x")
        .map(|(i, b)| (i, b.parse::<i64>().unwrap()))
        .collect();
    solve_chinese_remainder_theorem_brute(
        bus_data.iter().map(|(i, b)| ChineseRemainder {r: (b - (*i as i64)).rem_euclid(*b), m: *b})
    ).r
}
