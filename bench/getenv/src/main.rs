use std::env;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    for _i in 0..1_000_000 {
        let _ = env::var("DISPLAY").unwrap();
    }

    let end = start.elapsed();
    println!("{} sec", end.as_secs_f64());
}
