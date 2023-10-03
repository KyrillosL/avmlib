use std::time::Instant;
use rand::{distributions::Uniform, Rng}; // 0.6.5


type Precision = f64;
const VEC_SIZE: usize = 100000;
const VEC_SIZE_f64: f64 = 100000.0;

fn generate_native_vec() -> Vec<f64>{
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0.0, VEC_SIZE_f64);
    let data: Vec<f64> = (0..VEC_SIZE).map(|_| rng.sample(&range)).collect();
    return data;
}

fn native_rust(vec1 : &Vec<f64>, vec2 : &Vec<f64>) -> f64 {
    let mut output = 0.0;
    for i in 0..VEC_SIZE {
        output += vec1[i] * vec2[i];
    }
    return output;
}

pub(crate) fn benchmark_fn(f: &dyn Fn(&Vec<f64>, &Vec<f64>) -> f64) {
    let vec1 = generate_native_vec();
    let vec2 = generate_native_vec();
    let start_time = Instant::now();
    f(&vec1, &vec2);
    let elapsed_time = start_time.elapsed();
    println!("time: {:?}", elapsed_time);
}


pub(crate) fn compute_benchmarks() {

    //results : debug : ~7ms, release : ~30µs
    print!("native: ");
    benchmark_fn(&native_rust);

    println!("")
}

//Results :
    //Without rustflags = ["-Ctarget-cpu=native"]
    //cargo build --release + cargo run --release -> 128/130ms
    //cargo build + cargo run -> 2.52s

//With rustflags = ["-Ctarget-cpu=native"]
    //cargo build --release + cargo run --release -> 128/130ms
    //cargo build + cargo run -> 2.49s
pub(crate) fn sort_benchmark() {
    let start_time = Instant::now();
    for i in 0..20{
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 100000);
        let mut data: Vec<u64> = (0..100000).map(|_| rng.sample(&range)).collect();
        // Code to benchmark
        data.sort();
    }
    let elapsed_time = start_time.elapsed();

    println!("Elapsed time: {:?}", elapsed_time);
}