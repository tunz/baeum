use std::path::PathBuf;
use std::fs;
use std::io;
use std::io::prelude::*;
use conf::Conf;

#[derive(Clone)]
#[derive(Debug)]
pub struct Seed {
    filepath: PathBuf,
}

impl Seed {
    pub fn new(conf: &Conf, buf: &Vec<u8>) -> Seed {
        let new_seed = Seed::create_seed(conf);
        new_seed.save_buf(&buf);
        new_seed
    }

    pub fn new_from_file(conf: &Conf, filepath: &str) -> Seed {
        let new_seed = Seed::create_seed(conf);
        new_seed.copy_from_file(filepath);
        new_seed
    }

    fn create_seed(conf: &Conf) -> Seed {
        let seed_count = {
            let mut log = conf.log.write().unwrap();
            log.seed_count = log.seed_count + 1;
            log.seed_count
        };
        let path = conf.output_dir.join("queue").join(format!("tc-{}", seed_count));
        Seed { filepath: path }
    }

    pub fn load_buf(&self) -> Vec<u8> {
        let mut buf = vec![];
        fs::File::open(&self.filepath).unwrap().read_to_end(&mut buf).unwrap();
        buf
    }

    pub fn save_buf(&self, buf: &Vec<u8>) {
        let mut f = fs::File::create(&self.filepath).unwrap();
        f.write_all(buf).unwrap();
    }

    fn copy_from_file(&self, path: &str) {
        let mut buf: Vec<u8> = vec![];
        fs::File::open(&path).unwrap().read_to_end(&mut buf).unwrap();
        self.save_buf(&buf)
    }
}

pub fn load_seed_files(conf: &Conf, seed_dir: &str) -> io::Result<Vec<Seed>> {
    debug!("[*] Load seed files...");
    let seeds = try!(fs::read_dir(seed_dir))
        .filter_map(|entry| entry.ok())
        .filter_map(|e| e.path().to_str().and_then(|s| Some(String::from(s))))
        .map(|s| Seed::new_from_file(&conf, &s))
        .collect::<Vec<Seed>>();
    debug!("{:?}", seeds);
    Ok(seeds)
}
