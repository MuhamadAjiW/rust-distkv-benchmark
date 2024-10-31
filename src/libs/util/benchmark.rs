use std::{
    fs,
    io::{self, Write},
    thread, time,
};

use rand::Rng;
use reed_solomon_erasure::galois_8::ReedSolomon;

use crate::libs::class::benchmark_data::BenchmarkData;

fn simulate_data_transfer(
    filename: &str,
    object_size: usize,
    target_bandwidth: Option<usize>,
) -> io::Result<()> {
    let start = time::Instant::now();
    let mut file = fs::File::create(format!("{}.temp", filename))?;

    // Simulate good connection
    let simulation_str = "0".repeat(object_size);
    write!(file, "{simulation_str}")?;

    if let Some(target_bandwidth) = target_bandwidth {
        let target_bandwidth_bytes = target_bandwidth as f64 / 8.0;
        let target_duration = object_size as f64 / target_bandwidth_bytes;
        let elapsed = start.elapsed().as_secs_f64();
        if elapsed < target_duration {
            thread::sleep(time::Duration::from_secs_f64(target_duration - elapsed));
        }
    }

    Ok(())
}

pub fn remove_temps() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(extension) = path.extension() {
            if extension == "temp" {
                fs::remove_file(&path)?;
            }
        }
    }

    Ok(())
}

pub fn get_avg_bandwidth(
    iteration_count: usize,
    object_size: usize,
    target_bandwidth: Option<usize>,
) -> io::Result<f64> {
    let filename = "bw_test";
    let mut avg_bandwidth: f64 = 0.0;

    for _ in 0..iteration_count {
        let start = time::Instant::now();
        simulate_data_transfer(filename, object_size, target_bandwidth)
            .expect("Failed to simulate data transfer");
        let end = start.elapsed();

        avg_bandwidth += object_size as f64 / end.as_secs_f64() * 8.0;
    }
    avg_bandwidth = avg_bandwidth / iteration_count as f64;

    Ok(avg_bandwidth)
}

pub fn run_benchmark(
    iteration_count: usize,
    multiplication_count: usize,
    multiplication_factor: usize,
    shard_count: usize,
    parity_count: usize,
    object_size: usize,
    target_bandwidth: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    // Data Variable
    let mut benchmark_data = BenchmarkData::new();

    // Derived variables
    let object_size = object_size;
    let shard_size = object_size / shard_count;
    let parity_count = parity_count;
    let total_count = shard_count + parity_count;
    let mut target_bandwidth = target_bandwidth;

    // Reusable Variables
    let mut start: time::Instant;
    let mut setup_time: time::Duration;
    let mut recovery_time: time::Duration;
    let filename = "benchmark";

    benchmark_data.print_csv_header();
    for _ in 0..(multiplication_count + 1) {
        benchmark_data.avg_bandwidth =
            get_avg_bandwidth(iteration_count, object_size, target_bandwidth)
                .expect("Bandwidth test failed to execute");

        for iteration in 1..(iteration_count + 1) {
            benchmark_data.iteration = iteration;
            benchmark_data.object_size = object_size;
            benchmark_data.fail_tolerance = parity_count;

            let mut master_copy: Vec<Vec<u8>> = [].to_vec();

            for _ in 0..(total_count) {
                let array: Vec<u8> = (0..shard_size).map(|_| 0).collect();
                master_copy.push(array);
            }

            // Erasure coding----------------------------------------------------------------------
            start = time::Instant::now();
            let r = ReedSolomon::new(shard_count, parity_count).unwrap(); // 3 data shards, 2 parity shards
            r.encode(&mut master_copy).unwrap();
            let mut shards: Vec<_> = master_copy.iter().cloned().map(Some).collect();

            // Simulate file sending
            for _ in 0..(total_count - 1) {
                simulate_data_transfer(filename, shard_size, target_bandwidth)
                    .expect("Failed to simulate data transfer");
            }

            setup_time = start.elapsed();

            for _ in 0..(parity_count) {
                let index: usize = rng.gen_range(0, total_count);
                shards[index] = None;
            }

            start = time::Instant::now();

            // Simulate recovery
            r.reconstruct(&mut shards).unwrap();

            // Simulate file sending
            for _ in 0..(parity_count) {
                simulate_data_transfer(filename, shard_size, target_bandwidth)
                    .expect("Failed to simulate data transfer");
            }

            recovery_time = start.elapsed();

            let result: Vec<_> = shards.into_iter().filter_map(|x| x).collect();
            assert!(r.verify(&result).unwrap());
            assert_eq!(master_copy, result);

            benchmark_data.ec_setup_time = setup_time;
            benchmark_data.ec_recovery_time = recovery_time;
            benchmark_data.ec_shard = total_count;
            benchmark_data.ec_memory_usage = total_count * shard_size;
            benchmark_data.ec_bandwidth_setup_usage = (total_count - 1) * shard_size;
            benchmark_data.ec_bandwidth_recovery_usage = parity_count * shard_size;

            // Duplication----------------------------------------------------------------------
            for _ in 0..parity_count {
                master_copy.pop();
            }

            let mut replications: Vec<Vec<Vec<u8>>> = [].to_vec();
            replications.push(master_copy);

            start = time::Instant::now();

            for _ in 0..parity_count {
                // Simulate recovery
                replications.push(replications[0].clone());

                // Simulate file sending
                simulate_data_transfer(filename, object_size, target_bandwidth)
                    .expect("Failed to simulate data transfer");
            }

            setup_time = start.elapsed();

            // Simulate lost data
            for _ in 0..(parity_count) {
                replications.pop();
            }

            start = time::Instant::now();
            for _ in 0..(parity_count) {
                // Simulate recovery
                replications.push(replications[0].clone());

                // Simulate file sending
                simulate_data_transfer(filename, object_size, target_bandwidth)
                    .expect("Failed to simulate data transfer");
            }
            recovery_time = start.elapsed();

            benchmark_data.r_setup_time = setup_time;
            benchmark_data.r_recovery_time = recovery_time;
            benchmark_data.r_node_count = parity_count + 1;
            benchmark_data.r_memory_usage = (parity_count + 1) * object_size;
            benchmark_data.r_bandwidth_setup_usage = parity_count * object_size;
            benchmark_data.r_bandwidth_recovery_usage = parity_count * object_size;

            benchmark_data.print_csv_contents();
        }
        // object_size += multiplication_factor;
        // shard_size += multiplication_factor / shard_count;
        // parity_count += multiplication_factor;
        target_bandwidth = Some(target_bandwidth.unwrap() + multiplication_factor);
    }
    Ok(())
}
