#![feature(bench_black_box)]
use autodiff::autodiff;

#[derive(Debug)]
enum Kind {
    Float(f32),
    Double(f64),
}

#[autodiff(d_variants, Reverse, Active)]
fn variants(x: &Kind) -> f32 {
    match x {
        Kind::Float(x) => x*x,
        Kind::Double(x) => (x + x) as f32,
    }
}

fn main() {
    let input = Kind::Float(1.0);
    let mut output = Kind::Double(0.0);

    d_variants(&input, &mut output, 1.0);

    dbg!(&output);
}
