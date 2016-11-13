use std::collections::LinkedList;
use seed::Seed;

fn fuzz_one(seed:&Seed) -> LinkedList<Seed> {
  let mut new_seeds = LinkedList::new();
  //for i in 1..seed.len() {
  //  match seed.mutate(i).exec().classify() {
  //    Some(newSeed) => newSeeds.push(newSeed),
  //    None => ()
  //  }
  //}
  new_seeds.push_back(seed.clone());
  new_seeds.push_back(seed.clone());
  new_seeds
}

pub fn fuzz(target:&str) {
  let mut q = LinkedList::new();
  let mut q_rem = LinkedList::new();
  let mut q_new = LinkedList::new();
  let init_seed = Seed { filepath: "test".to_string() };
  q.push_back(init_seed.clone());
  // XXX: I want to reference the seed instead of clone.
  //      It will consume memory twice. Any idea?
  q_rem.push_back(init_seed.clone());
  loop {
    for seed in q_rem.iter() {
      let new_seeds = fuzz_one(seed);
      for nseed in new_seeds {
        q.push_back(nseed.clone());
        q_new.push_back(nseed.clone());
      }
    }
    q_rem = q_new;
    q_new = LinkedList::new();
  }
}
