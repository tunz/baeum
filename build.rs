extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/libexec.c")
        .compile("libexec.a");
}
