use std::time::Instant;

// Adjust this import if your function name is slightly different.
// It should match whatever you have in src/lib.rs.
use drl_engine::wdtp_sequence;

fn main() {
    // How many steps to simulate
    let n: usize = 10_000_000;

    println!("Running WDTP+NER benchmark for {n} steps...");

    let start = Instant::now();
    let seq = wdtp_sequence(n);
    let elapsed = start.elapsed();

    let seconds = elapsed.as_secs_f64();
    let tps = n as f64 / seconds;

    println!("Elapsed: {seconds:.6} seconds");
    println!("Throughput: {tps:.0} steps per second");
    if let Some(last) = seq.last() {
        println!("Final term a_{n} = {last}");
    }
}
