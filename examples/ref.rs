#![feature(abi_unadjusted)]

fn sin(x: &f32) -> f32 {
    f32::sin(*x)
}

#[autodiff(sin, mode = "reverse", Active, Duplicated)]
extern "unadjusted" {
    fn cos_ref(x: &f32, df_dx: &mut f32, factor: f32);
}

fn main() {
    // Here we can use ==, even though we work on f32.
    // Enzyme will recognize the sin function and replace it with llvm's cos function (see below).
    // Calling f32::cos directly will also result in calling llvm's cos function.
    let x = 3.14;
    let mut df_dx = 0.0;
    unsafe { cos_ref(&x, &mut df_dx, 1.0) };
    assert!(df_dx == f32::cos(x));
}

// Just for curious readers, this is the (inner) function that Enzyme does generate:
// define internal void @diffe_ZN3ref3sin17h39a807423b63d81cE(float* noalias nocapture noundef readonly align 4 dereferenceable(4) %0, float* nocapture %1, float %2) unnamed_addr #9 {
//   %4 = load float, float* %0, align 4
//   %5 = call fast float @llvm.cos.f32(float %4)
//   %6 = fmul fast float %2, %5
//   %7 = load float, float* %1, align 4
//   %8 = fadd fast float %7, %6
//   store float %8, float* %1, align 4
//   ret void
// }
