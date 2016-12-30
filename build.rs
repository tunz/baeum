extern crate gcc;

fn main() {
      gcc::compile_library("libexec.a", &["src/libexec.c"]);
}
