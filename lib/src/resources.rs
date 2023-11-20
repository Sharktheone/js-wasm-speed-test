use std::thread;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

#[derive(Debug, Clone)]
pub struct ResourceUsage {
    pub cpu: f32,
    pub mem: u64,
    pub elapsed: u128,
}


pub struct ResourceMonitor {
    pid: Pid,
    sys: System,
    stop: bool,
    pub resources: Vec<ResourceUsage>,
    last: usize,
}

impl ResourceMonitor {
    pub fn new(pid: u32) -> Self {
        ResourceMonitor {
            pid: Pid::from_u32(pid),
            sys: System::new_all(),
            stop: false,
            resources: vec![],
            last: 0,
        }
    }

    pub fn start(&mut self, start: &std::time::Instant) {
        loop {
            self.sys.refresh_process(self.pid);
            self.sys.refresh_cpu();
            if let Some(sys) = self.sys.process(self.pid) {
                self.resources.push(ResourceUsage {
                    cpu: sys.cpu_usage(),
                    mem: sys.memory(),
                    elapsed: start.elapsed().as_micros(),
                });
            }
            if self.stop {
                break;
            }
            thread::sleep(std::time::Duration::from_millis(10)); //TODO: maybe do this in respect of what time we needed for the loop
        }
    }

    pub fn get_usage_since_last(&mut self) -> Vec<ResourceUsage> {
        let last = self.last;
        self.last = self.resources.len();
        self.resources.get(last..).unwrap().to_vec()
    }

    pub fn get_current_index(&self) -> usize {
        self.resources.len()
    }

    pub fn stop(&mut self) {
        self.stop = true;
    }
}