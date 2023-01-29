This repository serves as an introduction on how to use Enzyme as a Rust user.

For the meantime, we will also use it to verify some layouts, before we move them into the propper Rust compiler test folders.


A certain example x.rs in the examples folder can be executed using
cargo clean && cargo +enzyme run --release --example X -Z unstable-options --config 'lto="fat"'


For the sin.rs example this equates to 
cargo clean && cargo +enzyme run --release --example sin -Z unstable-options --config 'lto="fat"'

