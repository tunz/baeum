use seed::Seed;
use mutate::mutate;

pub fn fuzz(seeds:Vec<Seed>, args:Vec<&str>, output_dir:&str) {
  let mut q = seeds;

  loop {
    let mut cur = 0;

    while cur < q.len() {
      let new_seeds = mutate(&q[cur]);
      q.extend(new_seeds);
      cur += 1;
    }
  }
}
