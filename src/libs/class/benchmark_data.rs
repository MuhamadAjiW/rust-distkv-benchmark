use std::time;

pub struct BenchmarkData {
    pub iteration: usize,
    pub object_size: usize,
    pub fail_tolerance: usize,
    pub avg_bandwidth: f64,
    pub ec_setup_time: time::Duration,
    pub ec_recovery_time: time::Duration,
    pub ec_shard: usize,
    pub ec_memory_usage: usize,
    pub ec_bandwidth_setup_usage: usize,
    pub ec_bandwidth_recovery_usage: usize,
    pub r_setup_time: time::Duration,
    pub r_recovery_time: time::Duration,
    pub r_node_count: usize,
    pub r_memory_usage: usize,
    pub r_bandwidth_setup_usage: usize,
    pub r_bandwidth_recovery_usage: usize,
}

impl BenchmarkData {
    pub fn new() -> Self {
        BenchmarkData {
            iteration: 0,
            object_size: 0,
            fail_tolerance: 0,
            avg_bandwidth: 0.0,
            ec_setup_time: time::Duration::new(0, 0),
            ec_recovery_time: time::Duration::new(0, 0),
            ec_shard: 0,
            ec_memory_usage: 0,
            ec_bandwidth_setup_usage: 0,
            ec_bandwidth_recovery_usage: 0,
            r_setup_time: time::Duration::new(0, 0),
            r_recovery_time: time::Duration::new(0, 0),
            r_node_count: 0,
            r_memory_usage: 0,
            r_bandwidth_setup_usage: 0,
            r_bandwidth_recovery_usage: 0,
        }
    }

    pub fn print_csv_header(&self) {
        println!("iteration,object_size(byte),fail_tolerance,avg_bandwidth(bit/s),ec_setup_time(s),ec_recovery_time(s),ec_shard,ec_memory_usage(byte),ec_bandwidth_setup_usage(byte),ec_bandwidth_recovery_usage(byte),r_setup_time(s),r_recovery_time(s),r_node_count,r_memory_usage(byte),r_bandwidth_setup_usage(byte),r_bandwidth_recovery_usage(byte)");
    }

    pub fn print_csv_contents(&self) {
        print!("{},", self.iteration);
        print!("{},", self.object_size);
        print!("{},", self.fail_tolerance);
        print!("{},", self.avg_bandwidth);
        print!("{},", self.ec_setup_time.as_secs_f64());
        print!("{},", self.ec_recovery_time.as_secs_f64());
        print!("{},", self.ec_shard);
        print!("{},", self.ec_memory_usage);
        print!("{},", self.ec_bandwidth_setup_usage);
        print!("{},", self.ec_bandwidth_recovery_usage);
        print!("{},", self.r_setup_time.as_secs_f64());
        print!("{},", self.r_recovery_time.as_secs_f64());
        print!("{},", self.r_node_count);
        print!("{},", self.r_memory_usage);
        print!("{},", self.r_bandwidth_setup_usage);
        print!("{},\n", self.r_bandwidth_recovery_usage);
    }
}
