//Note that fortran should be installed, but it's a part of gcc, and openblas
//brew install openblas :)

extern crate blas_src;
use std::time::Instant;
use rand::{distributions::Uniform as uni_test, Rng}; // 0.6.5
use ndarray::Array1;
use ndarray_rand::{rand_distr::Uniform, RandomExt};

type Precision = f64;
const VEC_SIZE: usize = usize::MAX / 1000000000000;
const RANGE_f64: f64 = 1.0;

fn generate_native_vec() -> Vec<f64>{
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0.0, RANGE_f64);
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


pub fn dot_product_stackoverflow1(vec1 : &Vec<f64>, vec2 : &Vec<f64>) -> f64{
    //https://stackoverflow.com/questions/65984936/efficient-simd-dot-product-in-rust
    //https://rust.godbolt.org/z/xEY3v1
    vec1.iter()
        .zip(vec2)
        .map(|(&vec1, &vec2)|vec1*vec2)
        .map(f64::from)
        .sum()
}

pub fn dot_product_stackoverflow2(vec1 : &Array1<f64>, vec2 : &Array1<f64>) -> f64{
    //https://stackoverflow.com/questions/54028589/what-is-the-fastest-way-to-calculate-the-dot-product-of-two-f64-vectors-in-rust
    let result = vec1.dot(vec2);
    return result;
}



pub(crate) fn benchmark_fn<T>(vec1 : &T, vec2 : &T, f: &dyn Fn(&T, &T) -> f64) {
    let start_time = Instant::now();
    let result = f(&vec1, &vec2);
    let elapsed_time = start_time.elapsed();
    println!("time: {:?}, result : {}", elapsed_time, result);
}


pub(crate) fn compute_benchmarks() {
    println!("Vector size {}", VEC_SIZE);
    let vec1 = generate_native_vec();
    let vec2 = generate_native_vec();

    //Using ndarrays for so2
    //We could have used for the generation
    //let x = Array1::random(VEC_SIZE, Uniform::<f64>::new(0., RANGE_f64));
    //let y = Array1::random(VEC_SIZE, Uniform::<f64>::new(0., RANGE_f64));
    let x = Array1::from_vec(vec1.clone());
    let y = Array1::from_vec(vec2.clone());

    //results - release : : ~7ms, release : ~26ms
    print!("native: ");
    benchmark_fn(&vec1, &vec2, &native_rust);

    //results - release : : ~4ms, release : ~24ms
    print!("stackoverflow1: ");
    benchmark_fn(&vec1, &vec2, &dot_product_stackoverflow1);

    //results - release : ~18ms
    print!("stackoverflow2 - openblas: ");
    benchmark_fn(&x, &y, &dot_product_stackoverflow2);

    //blas > all only when VECTOR_SIZE is large. 
}
