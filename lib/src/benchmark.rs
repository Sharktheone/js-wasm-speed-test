use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

use futures::future;
use futures::lock::Mutex as AsyncMutex;
use reqwest::RequestBuilder;


const BENCHMARK_CONNECTIONS: u16 = 8192;


// We somehow need to get information from the process (cpu usage, memory usage, etc.) to also include it in the benchmark
// - Maybe just pass the pid to the benchmark function and let it handle it?
// - Maybe do a ResourceInfo struct and pass it to the benchmark function?
pub fn benchmark(request: RequestBuilder, duration: Duration) -> (f32, Vec<(bool, u16, String)>) {
    static FINISHED: AtomicBool = AtomicBool::new(false);
    let request = Arc::new(request);
    let res = Arc::new(Mutex::new(vec![]));


    let r = Arc::clone(&res);
    let handle = thread::spawn(move || {
        let tasks = future::join_all((0..BENCHMARK_CONNECTIONS).map(|_| {
            let request = Arc::clone(&request);
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

    handle.join().unwrap();

    let res = Arc::try_unwrap(res).unwrap().into_inner().unwrap();

    let rps = res.len() as f32 / duration.as_secs_f32();


    (rps, res)
}


pub fn benchmark_no_validate(request: RequestBuilder, duration: Duration) -> f32 {
    static FINISHED: AtomicBool = AtomicBool::new(false);
    static REQUESTS: AtomicU64 = AtomicU64::new(0);
    let request = Arc::new(request);


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

    handle.join().unwrap();

    REQUESTS.load(Ordering::SeqCst) as f32 / duration.as_secs_f32()
}
