use seed::Seed;
use conf::Conf;
use mutate;
use exec;

fn fuzz_one(conf:&Conf, seed:&Seed) -> Vec<Seed> {
  let mut new_seeds = vec![];
  let content = seed.load_buf();

  let mutated_content = mutate::mutate(&content);
  let interesting = exec::run_target(&conf, &mutated_content);
  if interesting {
    let new_seed = Seed::new(conf, &mutated_content);
    new_seeds.push(new_seed);
  }

  new_seeds
}

pub fn fuzz(conf:Conf, seeds:Vec<Seed>) {
  let mut q = seeds;

  loop {
    let mut cur = 0; // TODO: scheduling?

    while cur < q.len() {
      let new_seeds = fuzz_one(&conf, &q[cur]);
      q.extend(new_seeds);
      cur += 1;
    }
  }
}
