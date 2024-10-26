use crate::common::progress;
use rug::{ops::CompleteRound, Float};

pub fn classic_taylor(l: i64, trace: bool) -> f64 {
    // the taylor series for ln(1+x) = x - x^2/2 + x^3/3 + ...
    // when x=1, we get ln(2) = 1 - 1/2 + 1/3 - 1/4 + 1/5 - 1/6 + ...
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
    t + 0.5 / i as f64 // terminate by adding half of the next term
                       // this is the euler transform for alternating series
                       // it accelerates convergence
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
            // ln(x) = 1 + ln(x/e)
            two /= e; // we want x > 1, so we only apply this when x > e
            ret += &del;
        }
        two = (&two * &two).complete(prec); // ln(x) = ln(x²) / 2
        del /= 2; // halve the amount we will add, & square the argument
    }
    if trace {
        println!();
    }
    ret
}
