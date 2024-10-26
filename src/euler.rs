use rug::{ops::CompleteRound, Float};

use crate::common::progress;

pub fn maclaurin(l: u64, p: u32, trace: bool) -> Float {
    let mut e = Float::with_val(p, 1);
    let mut t = Float::with_val(p, 1);
    for i in 1..=l {
        if trace {
            progress(((80 * i) / l) as usize);
        }
        t /= i; // t is 1/i!
        e += &t;
    } // this is just the sum of 1/i! for i from 0 to l
      // classic formula, very fast convergence
    if trace {
        println!();
    }
    e
}

pub fn interest(n: u64, p: u32, trace: bool) -> Float {
    let mut e = Float::with_val(p, 1);
    for i in 0..n {
        if trace {
            progress(((40 * i) / n) as usize);
        }
        e /= 2; // e will be 2⁻ⁿ
    }
    e = 1 + e; // 1 + 2⁻ⁿ
    for i in 0..n {
        if trace && (0 == (i % 50)) {
            progress(40 + ((40 * i) / n) as usize);
        }
        e = (&e * &e).complete(p); // square it n times
    } // (1 + 2⁻ⁿ) ^ 2ⁿ
      // it's a version of the classic limit, (1 + 1/n)ⁿ as n → ∞
    if trace {
        println!();
    }
    e
}
