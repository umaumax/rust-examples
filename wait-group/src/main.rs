use crossbeam_utils::sync::WaitGroup;
use std::thread;

fn main() {
    let wg = WaitGroup::new();

    for i in 0..4 {
        let wg = wg.clone();
        thread::spawn(move || {
            println!("{}", i);
            drop(wg);
        });
    }

    wg.wait();
    println!("Done!");
}
