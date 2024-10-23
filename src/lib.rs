use std::time::Instant;
use sysinfo::{System, SystemExt};

pub fn process_data(data: &[i32]) -> i32 {
    data.iter()
        .map(|&x| x * x) 
        .filter(|&x| x <= 1000) 
        .sum() 
}

pub fn measure_performance(data: &[i32]) {
    let start_time = Instant::now();

    let result = process_data(data);

    let elapsed_time = start_time.elapsed();
    let memory_usage = get_memory_usage();

    // Print the results
    println!("Processed Result: {}", result);
    println!("Running Time: {:.6?} seconds", elapsed_time);
    println!("Memory Usage: {:.6} MB", memory_usage);
}

pub fn get_memory_usage() -> f64 {
    let mut sys = System::new_all();
    sys.refresh_memory();
    
    let used_memory = sys.used_memory(); 
    used_memory as f64 / (1024.0 * 1024.0) 
}


