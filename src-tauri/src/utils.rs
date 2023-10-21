use ndarray::s;
use ndarray::Array;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

let random_samples: Vec<(f32, f32)> = {
let x_y_axes = Array::random((2, 5), Uniform::new(0., 1.));
let x_axis: Vec<f32> = x_y_axes.slice(s![0, ..]).to_vec();
let y_axis: Vec<f32> = x_y_axes.slice(s![0, ..]).to_vec();
x_axis.into_iter().zip(y_axis).collect()
};