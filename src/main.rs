use rug::{rational::MiniRational, Float};

mod common;
mod euler;
mod ln2;
mod pi;
use euler::{interest, maclaurin};
use ln2::{classic_taylor, log_props};
use pi::{pi_arclength, pi_area, pi_classic};

fn main() {
    println!("Hello, world!");
    let prec: u32 = 256;
    // initial & minimum Δt, 2⁻²⁰ and 2⁻¹²⁸, respectively
    let d: Float = Float::with_val(prec, &*MiniRational::from((1, 1 << 20)).borrow());
    let m: Float = Float::with_val(prec, &*MiniRational::from((1, 1u128 << 80)).borrow());

    println!("= PI =");
    println!(
        "arclength diff. eq.: {}",
        pi_arclength(d.clone(), m.clone(), prec, true)
    );
    println!("area integral: {}", pi_area(1 << 20, true));
    println!("arctan taylor series: {}", pi_classic(1 << 22, true));
    println!();

    println!("= e =");
    println!("maclaurin series: {}", maclaurin(1 << 20, prec, true));
    let e = interest(2040, 2048, true);
    println!("interest formula: {}", &e);
    println!();

    println!("= NATURAL LOG OF 2 =");
    println!("ln taylor series: {}", classic_taylor(1 << 22, true));
    println!("log properties: {}", log_props(400, &e, false));
}
