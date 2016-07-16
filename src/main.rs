extern crate clap;
use clap::{Arg, App, ArgMatches};
mod fuzz;

fn arg_parse<'a> () -> ArgMatches<'a> {
    App::new("ROT")
    .version("0.1")
    .author("Choongwoo Han <cwhan.tunz@gmail.com>")
    .about("A XXX fuzzer")
    .arg(Arg::with_name("target")
         .help("Target program")
         .required(true)
         .index(1))
    .get_matches()
}

fn main() {
    let matches = arg_parse();
    let target = matches.value_of("target").unwrap();
    println!("Target program: {}", target);

    fuzz::fuzz(target);
}
