
use std::time::SystemTime;
use std::collections::HashSet;

const LOG_INTERVAL: u64 = 60; 

#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct LogInfo {
    pub seed_count: u32,
    pub crash_count: u32,
    pub uniq_crash_count: u32,
    pub exec_count: u64,
    pub total_node: u32,
}

pub struct LogData {
    pub start_time: SystemTime,
    pub last_time: SystemTime,
    pub infos: Vec<LogInfo>,
    pub crash_paths: HashSet<u64>,
}

pub struct Log {
    pub info: LogInfo,
    pub data: LogData,
}

impl LogInfo {
    pub fn new() -> Self {
        LogInfo {
            seed_count: 0,
            crash_count: 0,
            uniq_crash_count: 0,
            exec_count: 0,
            total_node: 0,
        }
    }
}

impl LogData {
    pub fn new() -> Self {
        LogData {
            start_time: SystemTime::now(),
            last_time: SystemTime::now(),
            infos: vec![],
            crash_paths: HashSet::new(),
        }
    }

    pub fn reset_last_time(&mut self) {
        self.last_time = SystemTime::now();
    }
}

impl Log {
    pub fn new() -> Self {
        Log {
            info: LogInfo::new(),
            data: LogData::new(),
        }
    }

    pub fn update(&mut self) {
        let t = self.data.last_time.elapsed().unwrap().as_secs();
        if t < LOG_INTERVAL {
            return;
        }
        self.data.reset_last_time();
        self.data.infos.push(self.info.clone());
    }
}

