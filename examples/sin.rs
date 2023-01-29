#![feature(abi_unadjusted)]

fn sin(x: f32) -> f32 {
    f32::sin(x)
}
#[autodiff(sin, mode = "reverse", Active, Active)]
extern "unadjusted" {
    fn cos(x: f32, factor: f32) -> f32;
}

fn main() {
    // Here we can use ==, even though we work on f32.
    // Enzyme will recognize the sin function and replace it with llvm's cos function (see below).
    // Calling f32::cos directly will also result in calling llvm's cos function.
    assert!(unsafe { cos(3.14, 1.0) } - f32::cos(3.14) == 0.0);
}

// Just for curious readers, this is the (inner) function that Enzyme does generate:
// define internal { float } @diffe_ZN3sin3sin17h18f17f71fe94e58fE(float %0, float %1) unnamed_addr #35 {
//   %3 = call fast float @llvm.cos.f32(float %0)
//   %4 = fmul fast float %1, %3
//   %5 = insertvalue { float } undef, float %4, 0
//   ret { float } %5
// }
