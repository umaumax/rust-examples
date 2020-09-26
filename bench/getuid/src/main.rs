use nix::unistd::getuid;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    for _i in 0..1_000_000 {
        let _ = getuid();
    }

    let end = start.elapsed();
    println!("{} sec", end.as_secs_f64());
}
