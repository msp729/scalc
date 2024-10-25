use rug::{ops::CompleteRound, Float};

use crate::common::progress;

pub fn maclaurin(l: u64, p: u32, trace: bool) -> Float {
    let mut e = Float::with_val(p, 1);
    let mut t = Float::with_val(p, 1);
    for i in 1..=l {
        if trace {
            progress(((80 * i) / l) as usize);
        }
        t /= i;
        e += &t;
    }
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
        e /= 2;
    }
    e = 1 + e;
    for i in 0..n {
        if trace {
            progress(40 + ((40 * i) / n) as usize);
        }
        e = (&e * &e).complete(p);
    }
    println!();
    e
}
