fn main() {
    println!("cpu nums:{}", num_cpus::get());
    println!("physical_cpus nums:{}", num_cpus::get_physical());
}
