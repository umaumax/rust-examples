use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time;
use std::time::Instant;
use threadpool::ThreadPool;

use rusage_ex::rusage;

#[derive(Debug)]
struct Matrix {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);

    let pid = std::process::id();
    println!("pid: {:?}", pid);

    let sleep_ms = 1000;
    let width = 3200;
    let height = 3200;
    let data = vec![1; width * height];
    let matrix = Matrix {
        width,
        height,
        data,
    };

    let rwlock = RwLock::new(matrix);
    let arc = Arc::new(rwlock);

    let (tx, rx) = channel();

    let start = Instant::now();
    for _ in 0..n_jobs {
        let local_rwlock = arc.clone();
        let tx = tx.clone();
        pool.execute(move || {
            let start = Instant::now();
            let start_rusage = rusage::getrusage_thread().unwrap();
            let tid = nix::unistd::gettid().as_raw();
            println!("tid: {:?} start", tid);

            let mut sum = 0 as i32;
            let matrix = local_rwlock.read().unwrap();
            for j in 0..matrix.height {
                for i in 0..matrix.width {
                    let elem = matrix.data[j * matrix.width + i];
                    sum += elem as i32;
                }
            }
            println!("tid: {:?} sum={:?}", tid, sum);

            thread::sleep(time::Duration::from_millis(sleep_ms));

            println!("tid: {:?} end", tid);
            let end_rusage = rusage::getrusage_thread().unwrap();
            let end = start.elapsed();
            let elapsed_time = end.as_secs_f64();
            let rusage_time_diff = rusage::calc_time_diff(&start_rusage, &end_rusage);
            let elapsed_cpu_time = rusage_time_diff;
            tx.send((elapsed_cpu_time, elapsed_time))
                .expect("channel will be there waiting for the pool");
        });
    }

    let mut elapsed_times = vec![];
    for v in rx.iter().take(n_jobs) {
        elapsed_times.push(v);
    }
    let end = start.elapsed();

    for (elapsed_cpu_time, elapsed_time) in elapsed_times {
        let cpu_ratio = elapsed_cpu_time as f64 / elapsed_time as f64 * 100.0;
        println!(
            "cpu time: {:.12}/{:.12} sec ({:.3}%)",
            elapsed_cpu_time, elapsed_time, cpu_ratio
        );
    }
    println!("entire elapsed time: {} sec", end.as_secs_f64());

    Ok(())
}
