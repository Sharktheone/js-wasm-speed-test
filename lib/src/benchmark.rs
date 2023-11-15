use std::sync::{Arc, Mutex};
use futures::lock::Mutex as AsyncMutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::Duration;
use futures::future;
use reqwest::RequestBuilder;

pub fn benchmark(request: RequestBuilder, connections: u16, duration: Duration) -> (f64, Vec<(bool, u16, String)>) {
    static FINISHED: AtomicBool = AtomicBool::new(false);
    let request = Arc::new(request);
    let res = Arc::new(Mutex::new(vec![]));


    let r = Arc::clone(&res);
    let handle = thread::spawn(move || {
        let tasks = future::join_all((0..connections).map(|_| {
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

    //TODO: calculate requests/s

    let res = Arc::try_unwrap(res).unwrap().into_inner().unwrap();

    (0.0, res)
}


pub fn benchmark_no_validate(request: RequestBuilder, connections: u64, duration: Duration) -> f64 {
    static FINISHED: AtomicBool = AtomicBool::new(false);
    static REQUESTS: AtomicU64 = AtomicU64::new(0);
    let request = Arc::new(request);


    let handle = thread::spawn(move || {
        let tasks = future::join_all((0..connections).map(|_| {
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

    //TODO: calculate requests/s

    0.0
}
