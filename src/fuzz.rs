use seed::Seed;
use conf::Conf;
use mutate;
use exec;

fn fuzz_one(conf:&Conf, seed:&Seed) -> Vec<Seed> {
  let mut new_seeds = vec![];
  let content = seed.load_buf();

  let mutated_content = mutate::mutate(&content);
  let interesting = exec::run_target(&conf, &mutated_content);

  // XXX

  new_seeds
}

pub fn fuzz(conf:Conf, seeds:Vec<Seed>) {
  let mut q = seeds;

  loop {
    let mut cur = 0;

    while cur < q.len() {
      let new_seeds = fuzz_one(&conf, &q[cur]);
      q.extend(new_seeds);
      cur += 1;
    }
  }
}
