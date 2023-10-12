use packed_simd::f64x8;
use crate::constants::*;

pub fn dot_product_simd(vec1: &Vec<Precision>, vec2: &Vec<Precision>) -> Precision {
    return vec1
        .chunks_exact(8)
        .map(f64x8::from_slice_unaligned)
        .zip(vec2.chunks_exact(8).map(f64x8::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f64x8>()
        .sum();
}