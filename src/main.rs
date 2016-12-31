extern crate clap;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate rand;
extern crate memmap;
extern crate byteorder;

use clap::{Arg, App, ArgMatches, AppSettings};
mod seed;
mod mutate;
mod fuzz;
mod conf;
mod exec;

fn arg_parse<'a> () -> ArgMatches<'a> {
  App::new("baeum")
    .setting(AppSettings::TrailingVarArg)
    .setting(AppSettings::AllowLeadingHyphen)
    .version("0.0.1")
    .author("Choongwoo Han <cwhan.tunz@gmail.com>")
    .about("A Reinforcement-Learning-Based Fuzzing")
    .arg(Arg::with_name("input")
       .short("i")
       .help("Directory of input seed files")
       .required(true)
       .takes_value(true))
    .arg(Arg::with_name("output")
       .short("o")
       .help("Directory of output files")
       .required(true)
       .takes_value(true))
    .arg(Arg::with_name("timeout")
       .short("t")
       .help("Timeout for each execution in milliseconds (default: 1000)")
       .takes_value(true))
    .arg(Arg::from_usage("<args>... 'commands to run'"))
    .get_matches()
}

fn main() {
  env_logger::init().unwrap();

  let matches = arg_parse();
  let seeds_dir = matches.value_of("input").unwrap();
  let output_dir = matches.value_of("output").unwrap();
  let args: Vec<&str> = matches.values_of("args").unwrap().collect();
  let t = matches.value_of("timeout").unwrap_or("1000").parse::<u64>()
            .expect("Fail to parse timeout option");

  debug!("Seed Dir: {}", seeds_dir);
  debug!("Output Dir: {}", output_dir);
  debug!("Command Line: {:?}", args);

  let conf = conf::Conf::new_without_filename(args, output_dir, t);
  let seeds = match seed::load_seed_files(&conf, seeds_dir) {
                Ok(v) => v,
                Err(e) => { println!("Error: {}", e); return}
              };

  exec::initialize(&conf);
  fuzz::fuzz(conf, seeds);
  exec::finalize();

  println!("Fuzzing is finished!");
}
