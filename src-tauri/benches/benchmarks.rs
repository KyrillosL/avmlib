//Note that fortran should be installed, but it's a part of gcc, and openblas
//brew install openblas :)

extern crate blas_src;
extern crate packed_simd;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ndarray::Array1;
use ndarray_rand::rand_distr::Uniform;
use rand::Rng; // 0.6.5

use avmlib::dot_product::*;
use avmlib::constants::*;

fn generate_native_vec(in_vec_size: u64) -> Vec<Precision> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0.0, RANGE_f64);
    let data: Vec<Precision> = (0..in_vec_size).map(|_| rng.sample(&range)).collect();
    return data;
}

fn dot_product_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Dot Product");
    for i in VEC_SIZES {
        let vec1 = generate_native_vec(i);
        let vec2 = generate_native_vec(i);

        //Using ndarrays for so2
        //We could have used for the generation
        //let x = Array1::random(VEC_SIZE, Uniform::<f64>::new(0., RANGE_f64));
        //let y = Array1::random(VEC_SIZE, Uniform::<f64>::new(0., RANGE_f64));
        let x = Array1::from_vec(vec1.clone());
        let y = Array1::from_vec(vec2.clone());

        group.bench_with_input(BenchmarkId::new("Naive Native", i), &i, |b, &_s| {
            b.iter(|| naive_native_rust(black_box(&vec1), black_box(&vec2)));
        });


        group.bench_with_input(BenchmarkId::new("Native 1", i), &i, |b, &_s| {
            b.iter(|| native_rust(black_box(&vec1), black_box(&vec2)));
        });

        group.bench_with_input(BenchmarkId::new("Native 2", i), &i, |b, &_s| {
            b.iter(|| native_rust2(black_box(&vec1), black_box(&vec2)));
        });

        group.bench_with_input(BenchmarkId::new("SIMD f64x8", i), &i, |b, &_s| {
            b.iter(|| dot_product_simd(black_box(&vec1), black_box(&vec2)));
        });

        group.bench_with_input(BenchmarkId::new("Blas ndarray", i), &i, |b, &_s| {
            b.iter(|| rust_ndarray_blas(black_box(&x), black_box(&y)));
        });

    }
    group.finish();

    //TODO :  par_iter parallelism + SIMD f64x8
    //blas > all only when VECTOR_SIZE is large.
}

criterion_group! {
  name = dot_products;
  config = Criterion::default().sample_size(10);
  targets = dot_product_benchmarks
}
criterion_main!(dot_products);
