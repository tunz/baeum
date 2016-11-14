use seed::Seed;

fn fuzz_one(seed:&Seed) -> Vec<Seed> {
  let mut new_seeds = vec![];
  //for i in 1..seed.len() {
  //  match seed.mutate(i).exec().classify() {
  //    Some(newSeed) => newSeeds.push(newSeed),
  //    None => ()
  //  }
  //}
  new_seeds.push(seed.clone());
  new_seeds.push(seed.clone());
  new_seeds
}

pub fn fuzz(target:&str) {
  let mut q = vec![];
  let mut cur = 0;
  let init_seed = Seed { filepath: "test".to_string() };
  q.push(init_seed.clone());
  loop {
    while cur < q.len() {
      let new_seeds = fuzz_one(&q[cur]);
      q.extend(new_seeds);
      cur += 1;
    }
  }
}
