use std::ops::Range;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

use futures::future;
use futures::lock::Mutex as AsyncMutex;
use reqwest::RequestBuilder;
use crate::errors::TestError;

use crate::resources::ResourceMonitor;

const BENCHMARK_CONNECTIONS: u16 = 8192;

pub struct BenchmarkResult {
    pub rps: f32,
    pub status: Option<Vec<(bool, u16, String)>>,
    pub usage: (Range<usize>, usize),
}


pub fn benchmark(request: &RequestBuilder, duration: Duration, monitor: &ResourceMonitor) -> Result<BenchmarkResult, TestError> {
    static FINISHED: AtomicBool = AtomicBool::new(false);
    let res = Arc::new(Mutex::new(vec![]));

    let mut resource_range = Range {
        start: monitor.get_current_index(),
        end: 0,
    };


    let r = Arc::clone(&res);
    let handle = thread::spawn(move || {
        let tasks = future::join_all((0..BENCHMARK_CONNECTIONS).map(|_| {
            let res = Arc::clone(&r);
            let status = Arc::new(AsyncMutex::new(vec![]));


            async move {
                while !&FINISHED.load(Ordering::SeqCst) {
                    let res = request.try_clone().unwrap().send().await.unwrap();
                    let mut status = status.lock().await;
                    status.push(res);
                }

                let status = Arc::try_unwrap(status).unwrap().into_inner();
                let mut result = Vec::with_capacity(status.len());

                for status in status {
                    let code = status.status().as_u16();
                    let text = status.text().await.unwrap();

                    result.push((false, code, text));
                }

                res.lock().unwrap().append(&mut result);
            }
        }));

        futures::executor::block_on(tasks);
    });

    thread::sleep(duration);

    FINISHED.store(true, Ordering::SeqCst);
    let index_finish_signal = monitor.get_current_index();

    handle.join().unwrap();

    resource_range.end = monitor.get_current_index();

    let res = Arc::try_unwrap(res).unwrap().into_inner().unwrap();

    let rps = res.len() as f32 / duration.as_secs_f32();


    Ok(BenchmarkResult {
        rps,
        status: Some(res),
        usage: (resource_range, index_finish_signal),
    })
}


pub fn benchmark_no_validate(request: &RequestBuilder, duration: Duration, monitor: &ResourceMonitor) -> Result<BenchmarkResult, TestError> {
    static FINISHED: AtomicBool = AtomicBool::new(false);
    static REQUESTS: AtomicU64 = AtomicU64::new(0);
    let request = Arc::new(request);

    let mut resource_range = Range {
        start: monitor.get_current_index(),
        end: 0,
    };

    let handle = thread::spawn(move || {
        let tasks = future::join_all((0..BENCHMARK_CONNECTIONS).map(|_| {
            let request = Arc::clone(&request);
            async move {
                while !FINISHED.load(Ordering::SeqCst) {
                    request.try_clone().unwrap().send().await.unwrap();
                    REQUESTS.fetch_add(1, Ordering::SeqCst);
                }
            }
        }));

        futures::executor::block_on(tasks);
    });

    thread::sleep(duration);

    FINISHED.store(true, Ordering::SeqCst);
    let index_finish_signal = monitor.get_current_index();

    handle.join().unwrap();

    resource_range.end = monitor.get_current_index();

    let rps = REQUESTS.load(Ordering::SeqCst) as f32 / duration.as_secs_f32();

    Ok(BenchmarkResult {
        rps,
        status: None,
        usage: (resource_range, index_finish_signal),
    })
}
