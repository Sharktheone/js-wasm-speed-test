use std::cell::RefCell;
use std::sync::atomic::AtomicBool;
use std::sync::RwLock;
use std::thread;

use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

#[derive(Debug, Clone)]
pub struct ResourceUsage {
    pub cpu: f32,
    pub mem: u64,
    pub elapsed: u128,
}


#[derive(Debug)]
pub struct ResourceMonitor {
    pid: Pid,
    sys: RwLock<System>,
    stop: AtomicBool,
    pub resources: RwLock<Vec<ResourceUsage>>,
    last: usize,
}

impl ResourceMonitor {
    pub fn new(pid: u32) -> Self {
        ResourceMonitor {
            pid: Pid::from_u32(pid),
            sys: RwLock::new(System::new_all()),
            stop: AtomicBool::new(false),
            resources: RwLock::new(vec![]),
            last: 0,
        }
    }

    pub fn start(&self, start: &std::time::Instant) {
        loop {
            let mut sys = self.sys.write().unwrap();
            sys.refresh_process(self.pid);
            sys.refresh_cpu();
            if let Some(sys) = sys.process(self.pid) {
                self.resources.write().unwrap().push(ResourceUsage {
                    cpu: sys.cpu_usage(),
                    mem: sys.memory(),
                    elapsed: start.elapsed().as_micros(),
                });
            }
            if self.stop.load(std::sync::atomic::Ordering::SeqCst) {
                break;
            }
            thread::sleep(std::time::Duration::from_millis(10)); //TODO: maybe do this in respect of what time we needed for the loop
        }
    }

    pub fn get_usage_since_last(&mut self) -> Vec<ResourceUsage> {
        let last = self.last;
        let resources = self.resources.read().unwrap();
        self.last = resources.len();
       resources.get(last..).unwrap().to_vec()
    }

    pub fn get_current_index(&self) -> usize {
        self.resources.read().unwrap().len()
    }

    pub fn stop(&self) {
        self.stop.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}