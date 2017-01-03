use seed::Seed;
use conf::Conf;
use utils::get_random;
use mutate;
use exec;

fn fuzz_one(conf:&Conf, seed:&Seed, q:&Vec<Seed>) -> Vec<Seed> {
    let mut new_seeds = vec![];
    let content = seed.load_buf();

    for _ in 0..10 {
        let content = mutate::mutate(&content, q);
        let feedback = exec::run_target(&conf, &content);
        if feedback.newnode > 0 {
            let new_seed = Seed::new(conf, &content);
            new_seeds.push(new_seed);
        }
    }

    new_seeds
}

pub fn fuzz(conf:Conf, seeds:Vec<Seed>) {
    let mut q = seeds;

    loop {
        let new_seeds = {
            let seed = &q[get_random(q.len())];
            fuzz_one(&conf, seed, &q)
        };
        q.extend(new_seeds);
    }
}
