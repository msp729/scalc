use rug::Float;

mod pi;
use pi::{pi_arclength, pi_area};

fn main() {
    let prec: u32 = 256;
    let mut d: Float = Float::with_val(prec, 1);
    let mut m: Float = Float::with_val(prec, 1);
    for _ in 0..10 {
        m /= 100_000
    }
    d /= 1e6;
    println!("Hello, world!");
    println!("{}", pi_arclength(d.clone(), m.clone(), prec, true));
    println!("{}", pi_area(1 << 30, true));
}
