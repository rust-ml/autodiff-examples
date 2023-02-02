#[autodiff()]
fn sin(x: &Box<f32>) -> f32 {
    f32::sin(**x)
}

#[autodiff(mode = "reverse", Active, Duplicated)]
fn cos_box(x: &Box<f32>, df_dx: &mut Box<f32>, factor: f32) {
    let _ = sin(x);
    unreachable!()
}

fn main() {
    let x = Box::<f32>::new(3.14);
    let mut df_dx = Box::<f32>::new(0.0);
    cos_box(&x, &mut df_dx, 1.0);
    assert!(*df_dx == f32::cos(*x));
}
