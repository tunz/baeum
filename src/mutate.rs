use seed::Seed;

pub fn mutate(seed:&Seed) -> Vec<Seed> {
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

