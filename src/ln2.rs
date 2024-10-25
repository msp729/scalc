use crate::common::progress;

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
    println!();
    t + 0.5 / i as f64
}
