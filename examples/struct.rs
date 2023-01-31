#![feature(abi_unadjusted)]

use std::io;

// Will be represented as {f32, i16, i16} when passed by reference
// will be represented as i64 if passed by value
struct Foo {
    c1: i16,
    a: f32,
    c2: i16,
}

fn sin(x: &Foo) -> f32 {
    assert!(x.c1 < x.c2);
    f32::sin(x.a)
}

#[autodiff(sin, mode = "reverse", Active, Duplicated)]
extern "unadjusted" {
    fn cos_struct(x: &Foo, df_dx: &mut Foo, factor: f32);
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let c2 = s.trim_end().parse::<i16>().unwrap();
    dbg!(c2);

    let foo = Foo { c1: 4, a: 3.14, c2 };
    let mut df_dfoo = Foo { c1: 4, a: 0.0, c2 };

    dbg!(df_dfoo.a);
    dbg!(unsafe { cos_struct(&foo, &mut df_dfoo, 1.0) });
    dbg!(df_dfoo.a);
    dbg!(f32::cos(foo.a));
}
