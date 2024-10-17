use rug::ops::CompleteRound;
use rug::Float;

pub fn pi_arclength(mut d: Float, m: Float, prec: u32, log: bool) -> Float {
    let mut x = Float::with_val(prec, 1u8);
    let mut y = Float::with_val(prec, 0u8);
    let mut t = Float::with_val(prec, 0u8);
    let mut h = 0u8;
    while d >= m && x > y {
        if h == 0 && log {
            print!(
                "{}\r",
                "-".repeat((56.5 * (1.0 - (&x - &y).complete(prec).to_f64())) as usize)
            );
        }
        h = (h + 1) % 250;
        let (x0, y0) = ((-&y).complete(prec), x.clone());
        let (x1, y1) = (
            &d * (&y0 / -3i8).complete(prec) - &y,
            &x + &d * (&x0 / 3u8).complete(prec),
        );
        let (x2, y2) = (
            &d * ((&y0 / 3u8).complete(prec) - &y1) - &y,
            &d * (&x1 - (&x0 / 3u8).complete(prec)) + &x,
        );
        let (x3, y3) = (
            &d * ((&y1 - &y0).complete(prec) - &y2) - &y,
            &d * ((&x2 + &x0).complete(prec) - &x1) + &x,
        );
        let (dx, dy) = (
            &d * ((&x0 + &x1).complete(prec) + &x1 + &x1 + &x2 + &x2 + &x2 + &x3) / 8u8,
            &d * ((&y0 + &y1).complete(prec) + &y1 + &y1 + &y2 + &y2 + &y2 + &y3) / 8u8,
        );
        if (&x + &dx).complete(prec) >= (&y + &dy).complete(prec) {
            x += dx;
            y += dy;
            t += &d;
        } else {
            d /= 2;
        }
    }
    if log {
        println!("")
    }
    t * 4
}

pub fn pi_area_n(n: u128, log: bool) -> u128 {
    let mut x = 1;
    let mut y = n;
    let mut r = 0;
    let mut h = 0u8;
    while x < y {
        if h == 0 && log {
            print!("{}\r", "-".repeat(((56 * (n - y + x)) / n) as usize))
        }
        h = (h + 1) % 250;
        r += y;
        x += 1;
        while x * x + y * y > n * n + n {
            y -= 1;
        }
    }
    r += (x + n) / 2;
    println!("");
    2 * r - x * x
}

pub fn pi_area(d: u128, log: bool) -> f64 {
    4.0 * pi_area_n(d, log) as f64 / (d * d) as f64
}
