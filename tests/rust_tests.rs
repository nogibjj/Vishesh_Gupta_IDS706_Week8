use aes_cbc::{process_data, get_memory_usage};

#[test]
fn test_process_data_basic() {
    let data = [1, 2, 3, 4];
    let result = process_data(&data);
    assert_eq!(result, 30);
}

#[test]
fn test_process_data_large_values() {
    let data = [100, 20, 30, 4];
    let result = process_data(&data);
    assert_eq!(result, 1316);
}

#[test]
fn test_process_data_all_filtered() {
    let data = [50, 51, 52];
    let result = process_data(&data);
    assert_eq!(result, 0);
}

#[test]
fn test_get_memory_usage() {
    let memory_usage = get_memory_usage();
    assert!(memory_usage > 0.0, "Memory usage should be greater than 0");
}
