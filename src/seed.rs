use std::fs;
use std::io;

#[derive(Clone)]
#[derive(Debug)]
pub struct Seed {
  filepath: String
}

pub fn load_seed_files(seed_dir:&str) -> io::Result<Vec<Seed>> {
  debug!("[*] Load seed files...");
  let seeds = try!(fs::read_dir(seed_dir))
                .filter_map(|entry| entry.ok())
                .filter_map(|e| e.path().to_str().and_then(|s| Some(String::from(s))))
                .map(|s| Seed { filepath: s })
                .collect::<Vec<Seed>>();
  debug!("{:?}", seeds);
  Ok(seeds)
}
