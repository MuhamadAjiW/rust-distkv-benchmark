mod libs;

use libs::util::benchmark::{get_avg_bandwidth, remove_temps, run_benchmark};
use std::io::{self};

extern crate reed_solomon_erasure;

fn main() -> io::Result<()> {
    // Benchmarking configuration
    let iteration_count = 100;
    let multiplication_count = 10;
    // let multiplication_factor = 4;
    let multiplication_factor = 1024 * 1024 * 10;
    // let multiplication_factor = 100000000;

    // Shard configuration
    let shard_count: usize = 4;
    let recovery_count: usize = 2;

    // Memory configuration
    let object_size = 1024 * 1024 * 10;

    // Bandwidth configuration
    // let target_bandwidth = Some(1000000);
    let target_bandwidth = Some(100000000);

    run_benchmark(
        iteration_count,
        multiplication_count,
        multiplication_factor,
        shard_count,
        recovery_count,
        object_size,
        target_bandwidth,
    )
    .expect("Benchmark failed to execute");
    get_avg_bandwidth(iteration_count, object_size, target_bandwidth)
        .expect("Bandwidth check failed to execute");

    remove_temps().expect("Failed deleting temporary files");
    // println!("Benchmarking finished");

    Ok(())
}
