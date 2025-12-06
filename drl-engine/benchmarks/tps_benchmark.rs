use std::env;
use std::time::Instant;

use drl_engine::DrlEngine;

/// Simple TPS benchmark for the HashHelix DRL engine.
///
/// Usage:
///   cargo run --release --bin tps_benchmark -- <steps>
///
/// If <steps> is omitted, defaults to 10_000_000.
fn main() {
    // Read CLI args: first arg after `--` is the step count (optional).
    let args: Vec<String> = env::args().collect();

    let steps: usize = if args.len() > 1 {
        // Try to parse the user-provided step count.
        match args[1].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!(
                    "Invalid step count '{}'. Please provide a positive integer.",
                    args[1]
                );
                std::process::exit(1);
            }
        }
    } else {
        // Default if no argument is provided.
        10_000_000
    };

    println!("HashHelix DRL TPS Benchmark");
    println!("Running {} steps of WDTP+NER...", steps);

    let mut engine = DrlEngine::new();

    let start = Instant::now();
    engine.step_n(steps);
    let duration = start.elapsed();

    let secs = duration.as_secs_f64();
    let tps = (steps as f64) / secs;

    println!("--- Benchmark complete ---");
    println!("Steps run: {}", steps);
    println!("Elapsed:   {:.6} seconds", secs);
    println!("Throughput: {:.2} steps/second", tps);
    println!("Final term a_n: {}", engine.last_term);
}

