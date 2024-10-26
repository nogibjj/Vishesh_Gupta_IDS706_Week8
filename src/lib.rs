use std::fs::File;
use std::io::Write;
use std::time::Instant;
use sysinfo::{System, SystemExt};

pub fn process_data(data: &[i32]) -> i32 {
    data.iter().map(|&x| x * x).filter(|&x| x <= 1000).sum()
}

pub fn measure_performance(data: &[i32]) {
    let start_time = Instant::now();

    let result = process_data(data);

    let elapsed_time = start_time.elapsed();
    let memory_usage = get_memory_usage();

    // Print the results to the console
    println!("Processed Result: {}", result);
    println!("Running Time: {:.6?} seconds", elapsed_time);
    println!("Memory Usage: {:.6} MB", memory_usage);

    // Save results to a .md file
    save_results_to_md(result, elapsed_time, memory_usage);
}

pub fn get_memory_usage() -> f64 {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let used_memory = sys.used_memory();
    used_memory as f64 / (1024.0 * 1024.0)
}

pub fn save_results_to_md(result: i32, elapsed_time: std::time::Duration, memory_usage: f64) {
    let mut file = File::create("performance_results.md").expect("Unable to create file");

    let content = format!(
        "# Performance Results\n\n\
        - **Processed Result**: {}\n\
        - **Running Time**: {:.6?} seconds\n\
        - **Memory Usage**: {:.6} MB\n",
        result, elapsed_time, memory_usage
    );

    file.write_all(content.as_bytes()).expect("Unable to write data");
}