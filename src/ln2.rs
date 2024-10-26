use crate::common::progress;
use rug::{ops::CompleteRound, Float};

pub fn classic_taylor(l: i64, trace: bool) -> f64 {
    let mut i = 1;
    let mut t = 0.0;
    while i <= l {
        if trace {
            progress(((80 * i.abs()) / l) as usize);
        }
        t += 1f64 / i as f64;
        i += i.signum();
        i = -i;
    }
    if trace {
        println!();
    }
    t + 0.5 / i as f64
}

pub fn log_props(prec: u32, e: &Float, trace: bool) -> Float {
    let mut two = Float::with_val(prec, 2);
    let mut ret = Float::with_val(prec, 0);
    let mut del = Float::with_val(prec, 1);
    for i in 0..=prec {
        if trace {
            progress(((i * 80) / prec) as usize);
        }
        if two > *e {
            two /= e;
            ret += &del;
        }
        two = (&two * &two).complete(prec);
        del /= 2;
    }
    if trace {
        println!();
    }
    ret
}
