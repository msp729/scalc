use crate::common::progress;
use rug::ops::CompleteRound;
use rug::Float;

pub fn pi_arclength(mut d: Float, m: Float, prec: u32, trace: bool) -> Float {
    let mut x = Float::with_val(prec, 1u8);
    let mut y = Float::with_val(prec, 0u8);
    let mut t = Float::with_val(prec, 0u8);
    let mut h = 0u8;
    while d >= m && x > y {
        // ok this is a very verbose way to do differential equations
        // basically, it's Runge-Kutta 4
        // given a function f(t,y) that evaluates the derivative
        // and initial conditions t & y, we calculate a timestep of Δt as follows:
        // k₀ = f(t,y), the derivative at this point
        // k₁ = f(t + Δt/3, y + k₀*Δt/3), stepping forward by ⅓ of Δt
        // k₂ = f(t + 2*Δt/3, y + (k₁-k₀/3)*Δt), stepping forward by ⅔ of Δt
        // k₃ = f(t + Δt, y + (k₂ - k₁ + k₀)*Δt), stepping forward by a full Δt
        // then, finally, use (k₀ + 3k₁ + 3k₂ + k₃)/8 as the slope to step forward
        // also this particular d.e. has x'=-y, y'=x, t'=1, and initial conditions x=1, y=0, t=0
        if h == 0 && trace {
            progress(1 + (80.0 * (1.0 - (&x - &y).complete(prec).to_f64())) as usize);
        }
        h = (h + 1) % 250;
        let (x0, y0) = ((-&y).complete(prec), x.clone()); // k₀
        let (x1, y1) = (
            // k₁
            &d * (&y0 / -3i8).complete(prec) - &y,
            &x + &d * (&x0 / 3u8).complete(prec),
        );
        let (x2, y2) = (
            // k₂
            &d * ((&y0 / 3u8).complete(prec) - &y1) - &y,
            &d * (&x1 - (&x0 / 3u8).complete(prec)) + &x,
        );
        let (x3, y3) = (
            // k₃
            &d * ((&y1 - &y0).complete(prec) - &y2) - &y,
            &d * ((&x2 + &x0).complete(prec) - &x1) + &x,
        );
        let (dx, dy) = (
            // unite the slopes
            &d * (x0 + &x1 + &x1 + &x1 + &x2 + &x2 + &x2 + &x3) / 8u8,
            &d * (y0 + &y1 + &y1 + &y1 + &y2 + &y2 + &y2 + &y3) / 8u8,
        );
        if (&x + &dx).complete(prec) >= (&y + &dy).complete(prec) {
            x += dx; // if we'll still have x>y, step on!
            y += dy;
            t += &d;
        } else {
            // if this step would cross the x=y line, reduce Δt
            d /= 2;
        }
    }
    if trace {
        println!()
    }
    t * 4 // because we're going from (1,0) to (1/√2, 1/√2), the angle we get should be π/4.
          // so we quadruple it
}

pub fn pi_area_n(n: u128, trace: bool) -> u128 {
    // kind of a nonsense algorithm
    let mut x = 1; // we want to estimate the integral from 0 to 1/√2 of sqrt(1-x²)
    let mut y = n; // so we start with y at n, and x at 1 (trust me this makes sense)
    let mut r = 0; // this is our "running total"
    while x < y {
        if (x % 250) == 0 && trace {
            progress(1 + ((80 * (n - y + x)) / n) as usize);
        }
        r += y; // y estimates the function's value, so we add it to the total
        x += 1; // move forward
        while x * x + y * y > n * n + n {
            y -= 1; // decrease y, to make it fit "within" the circle
                    // i use n(n+1) instead of n² because it allows us to overestimate,
                    // instead of always underestimating
                    // this improves accuracy (i'm pretty sure)
        }
    }
    r += (x + n) / 2; // now that we got all that, add the boundary values
                      // this is almost like simpson's rule or the trapezoid rule
    if trace {
        println!();
    }
    2 * r - x * x
    // x is approximately n/√2, and r is approximately ∫ 0 to (n/√2) sqrt(n^2-x^2) dx
    // so r, when doubled, is the whole quarter circle in the first quadrant
    // except it double counts that square
    // so we subtract x², which is approx. that square
}

pub fn pi_area(d: u128, trace: bool) -> f64 {
    4.0 * pi_area_n(d, trace) as f64 / (d * d) as f64
    // that other function returns an integer
    // approximately the right integral, but scaled up by n²
    // so we divide by n², and quadruple to get the full area of the unit circle
}

pub fn pi_classic(l: i64, trace: bool) -> f64 {
    // it's really just an obscure way to do
    // 4 - 4/3 + 4/5 - 4/7 + 4/9 - 4/11 + ...
    // because famously(?) π/4 = 1 - 1/3 + 1/5 - 1/7 + 1/9 - 1/11 + ...
    let mut t = 4f64;
    let mut i = 1i64;
    while i <= l {
        if trace {
            progress(1 + ((i.abs() * 80) / l) as usize);
        }
        i += 2 * i.signum();
        i *= -1;
        t += 4f64 / i as f64;
    }
    i += 2 * i.signum();
    i *= -1;
    t += 2f64 / i as f64; // and then we finish it off with the euler transform
                          // (add half the next term, fast way to improve accuracy for alternating sums)
    if trace {
        println!();
    }
    t
}
