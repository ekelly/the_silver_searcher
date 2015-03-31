extern crate gcc;

use std::default::Default;

fn main() {
    let gcc_opts: gcc::Config = Default::default();
    gcc::compile_library("libminiz.a", &gcc_opts, &["src/c/miniz.c"]);
}
