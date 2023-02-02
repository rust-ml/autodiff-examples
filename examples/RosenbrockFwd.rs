#[autodiff()]
fn rosenbrock(y: *const [f64; 2]) -> f64 {
    let x = unsafe { *y };
    let mut res = 0.0;
    for i in 0..(x.len() - 1) {
        let a = x[i + 1] - x[i] * x[i];
        let b = x[i] - 1.0;
        res += 100.0 * a * a + b * b;
    }
    res
}

#[autodiff(mode = "forward", DuplicatedNoNeed, Duplicated)]
fn d_rosenbrock(y: &[f64; 2], df_dx: &mut [f64; 2]) -> f64 {
    let _ = rosenbrock(y);
    unreachable!()
}

fn main() {
    let x = [3.14, 2.4];
    let output = unsafe { rosenbrock(&x) };
    println!("{output}");
    let df_dx = unsafe { d_rosenbrock(&x, &mut [1.0, 0.0]) };
    let df_dy = unsafe { d_rosenbrock(&x, &mut [0.0, 1.0]) };

    // https://www.wolframalpha.com/input?i2d=true&i=x%3D3.14%3B+y%3D2.4%3B+D%5Brosenbrock+function%5C%2840%29x%5C%2844%29+y%5C%2841%29+%2Cy%5D
    assert!((df_dx - 9373.54).abs() < 0.1);
    assert!((df_dy - (-1491.92)).abs() < 0.1);
}
