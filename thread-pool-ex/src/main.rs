use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::channel;
use std::sync::{Arc, Barrier};
use std::thread;
use threadpool::ThreadPool;

fn main() {
    let mut pool = ThreadPool::with_name("worker".into(), 2);
    let test_count = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = channel();

    // このバリアは指定した回数waitされるまで待機し続ける(指定した回数に到達するとカウントはリセットされる)
    // waitする回数がこの指定した倍数でないと無限待機になることに注意
    // 下記の場合，0,1,2,4が有効な値
    let barrier = Arc::new(Barrier::new(2));
    for i in 0..4 {
        let barrier = barrier.clone();
        let tx = tx.clone();
        let test_count = test_count.clone();
        // NOTE: move keyword enables to force the closure to take ownership of `i`
        pool.execute(move || {
            assert_eq!(thread::current().name(), Some("worker"));
            println!("thread name = {:?}", thread::current().name());
            println!("index = {}", i);
            test_count.fetch_add(1, Ordering::Relaxed);
            barrier.wait();

            tx.send(i)
                .expect("channel will be there waiting for the pool");
        });
    }
    pool.join();

    // Increase thread capacity of the pool
    pool.set_num_threads(8);

    // Decrease thread capacity of the pool
    // No active threads are killed
    pool.set_num_threads(4);

    assert_eq!(test_count.load(Ordering::Relaxed), 4);
    assert_eq!(rx.iter().take(4).fold(0, |a, b| a + b), 0 + 1 + 2 + 3);
}
