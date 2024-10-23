fn main() {
    let data: Vec<i32> = (1..999).collect();

    aes_cbc::measure_performance(&data)
}
