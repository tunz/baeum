use rand;
use rand::distributions::{IndependentSample, Range};

pub fn get_random(sz: usize) -> usize {
    let rnd = Range::new(0, sz);
    let mut rng = rand::thread_rng();
    rnd.ind_sample(&mut rng)
}
