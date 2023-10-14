//Note that fortran should be installed, but it's a part of gcc, and openblas
//brew install openblas :)
extern crate blas_src;
extern crate packed_simd;
use crate::constants::*;
use packed_simd::f64x8;

use ndarray::Array1;

pub fn dot_product_simd(vec1: &Vec<Precision>, vec2: &Vec<Precision>) -> Precision {
    return vec1
        .chunks_exact(8)
        .map(f64x8::from_slice_unaligned)
        .zip(vec2.chunks_exact(8).map(f64x8::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f64x8>()
        .sum();
}

#[allow(dead_code)]
pub fn naive_native_rust(vec1: &Vec<Precision>, vec2: &Vec<Precision>) -> Precision {
    let mut output = 0.0;
    for i in 0..vec1.len() {
        output += vec1[i as usize] * vec2[i as usize];
    }
    return output;
}

#[allow(dead_code)]
pub fn native_rust(vec1: &Vec<Precision>, vec2: &Vec<Precision>) -> Precision {
    //https://stackoverflow.com/questions/65984936/efficient-simd-dot-product-in-rust
    //https://rust.godbolt.org/z/xEY3v1
    return vec1
        .iter()
        .zip(vec2)
        .map(|(&vec1, &vec2)| vec1 * vec2)
        .map(f64::from)
        .sum();
}

#[allow(dead_code)]
pub fn native_rust2(vec1: &Vec<Precision>, vec2: &Vec<Precision>) -> Precision {
    return vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
}

//TODO
/*
pub fn par_iter(vec1 : &Vec<f64>, vec2 : &Vec<f64>) -> f64{
    // what is par_iter doing? splitting for multiple threads
    // https://github.com/rayon-rs/rayon/blob/master/src/iter/plumbing/README.md
    return vec1
        .chunks_exact(8)
        .map(f64x8::from_slice_unaligned)
        .zip(vec2.chunks_exact(8).map(f64x8::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f64x8>()
        .sum();
}
*/

//TODO -> PAR ITER !
#[allow(dead_code)]
pub fn rust_ndarray_blas(vec1: &Array1<Precision>, vec2: &Array1<Precision>) -> Precision {
    //Using Blas as a backup
    //https://stackoverflow.com/questions/54028589/what-is-the-fastest-way-to-calculate-the-dot-product-of-two-f64-vectors-in-rust
    let result = vec1.dot(vec2);
    return result;
}