// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod constants;
mod sine;

use std::ops::Mul;
use avmlib::constants::*;
use avmlib::sine::*;
use avmlib::spiral::*;

//use avmlib::dot_product::*;
extern crate nalgebra as na;
use na::{U2, U3, Dyn, ArrayStorage, VecStorage, Matrix, Matrix3x4, RowVector4, Vector, Matrix1x4, DMatrix, Vector3, RowVector3, MatrixXx3, Matrix4x3, Vector4, RowDVector};
/*
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/


struct DenseLayer {
    batch_size : usize,
    n_inputs : usize,
    n_neurons: usize,
    weights: DMatrix<Precision>,
    biases: RowDVector<Precision>
}

/*
impl Default for DenseLayer {
    fn default() -> Self {
        Self {
            weights : Matrix3x4::new_random(),
            biases : RowVector3::zeros(),
        }
    }
}
*/


impl DenseLayer{
    fn new(batch_size : usize, n_inputs: usize, n_neurons: usize) -> Self {
        Self {
            batch_size: batch_size,
            n_inputs : n_inputs,
            n_neurons : n_neurons,

            //TODO : Put this in tests !
/*
            weights : DMatrix::from_row_slice(n_neurons, n_inputs,  &[
                0.2, 0.8, -0.5, 1.0,
                0.5, -0.91, 0.26, -0.5,
                -0.26, -0.27, 0.17, 0.87
            ]),
            biases : RowDVector::from_vec(vec![2.0, 3.0, 0.5]),
*/
            weights: DMatrix::new_random(n_neurons, n_inputs),
            biases: RowDVector::zeros(n_neurons),
        }
    }
    fn forward(&self, inputs: &DMatrix<Precision>) -> DMatrix<Precision> {

        //Convert unique vector (x, y, z) to :
        //For a 4*3 input :
        /*
          ┌       ┐
          │ x y z │
          │ x y z │
          │ x y z │
          └       ┘
         */
        let mut bs = DMatrix::<Precision>::zeros(self.batch_size, self.n_neurons);   //(&[self.biases]);
        for i in 0..self.batch_size{
            bs.set_row(i, &self.biases);
        }
        println!("bs{}", bs);

        println!("Weights{}", self.weights);
        let transposed = self.weights.transpose();
        println!("Transposed{}", transposed);
        let mult = inputs * &transposed;
        println!("mult{}", mult);

        //println!("bs{}", self.biases);
        let result = &mult + &bs;

        println!("Result{}", result);
        return result;
    }
}

/*
fn main() {

    println!("Start");

    let samples = 1000;
    let (x, y) = create_data(samples);
    println!("X: {:?}", x);
    println!("y: {:?}", y);



    let n_samples_inputs = 4;
    let n_neurons = 2;
    let batch_size = 3;

    //n_neurons MUST BE == WITH input_batch_size
    //TODO : put in tests
    let inputs = DMatrix::from_row_slice(batch_size,n_samples_inputs, &[
        1.0, 2.0, 3.0, 2.5,
        2.0, 5.0, -1.0, 2.0,
        -1.5, 2.7, 3.3, -0.8,
    ]);

    println!("inputs : {}", inputs);

    //atm the batch size is linked to the number of neurons
    let layer1 = DenseLayer::new( batch_size, n_samples_inputs, n_neurons); //3 rows, 4 cols
    layer1.forward(&inputs);
    */
    /*
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
     */

//}

use plotters::prelude::*;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use ndarray_rand::rand_distr::Normal;
use rand::distributions::Distribution;
/*
fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("Start");

    let samples = 1000;
    let (x, y) = create_data_spiral(samples, 3);
    println!("X: {:?}", x);
    println!("y: {:?}", y);



    //let sine_v:  Vec<(&f64, &f64)>  = x.iter().zip(y.iter()).take(1000).collect();
    //println!("{:?}", sine_v);

    let root = BitMapBackend::new("plots/0.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;


    let sd = 0.13;

    let random_points: Vec<(f64, f64)> = {
        let norm_dist = Normal::new(0.5, sd).unwrap();
        let mut x_rand = XorShiftRng::from_seed(*b"MyFragileSeed123");
        let mut y_rand = XorShiftRng::from_seed(*b"MyFragileSeed321");
        let x_iter = norm_dist.sample_iter(&mut x_rand);
        let y_iter = norm_dist.sample_iter(&mut y_rand);
        x_iter.zip(y_iter).take(5000).collect()
    };


    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;
    scatter_ctx.draw_series(
        x
            .iter()
            //.map(|(&x, &y)| Circle::new((x, y), 2, GREEN.filled())), random
            .map(|(x, y)| Circle::new((*x, *y), 2, GREEN.filled())),

    )?;

    root.present()?;

    Ok(())
}
*/

extern crate plotters;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const N: usize = 100; // number of points per class
    const D: usize = 2; // dimensionality
    const K: usize = 3; // number of classes

    let mut x = vec![0.0; N * K * D];
    let mut y = vec![0; N * K];

    for j in 0..K {
        let ix = N * j..N * (j + 1);
        let r = (0..N).map(|n| n as f64 / N as f64);
        let mut rng = rand::thread_rng();
        let t = (0..N)
            .map(|n| {
                let t = j as f64 * 4.0 + rng.gen::<f64>() * 0.2;
                t + n as f64 * 4.0 / N as f64
            });

        for (i, (r_val, t_val)) in r.zip(t).enumerate() {
            let index = ix.start + i;
            x[index * D] = r_val * f64::sin(t_val);
            x[index * D + 1] = r_val * f64::cos(t_val);
            y[index] = j as u32;
        }
    }
    println!("x{:?}", x);
    println!("y{:?}", y);

    println!("{}", x.len());
    println!("{}", y.len());


    // Visualize the data
    let root = BitMapBackend::new("plots/scatter.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_min = x.iter().step_by(2).copied().fold(f64::INFINITY, f64::min);
    let x_max = x.iter().step_by(2).copied().fold(f64::NEG_INFINITY, f64::max);
    let y_min = x.iter().skip(1).step_by(2).copied().fold(f64::INFINITY, f64::min);
    let y_max = x.iter().skip(1).step_by(2).copied().fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_ranged(x_min..x_max, y_min..y_max)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart.draw_series(
        x.iter()
            .step_by(2)
            .zip(x.iter().skip(1).step_by(2))
            .zip(y.iter())
            .map(|((x, y), label)| {
                return Circle::new((*x, *y), 5,
                                   plotters::style::RGBColor(
                                       if *label as u8 == 0 { 255 } else {0},
                                       if *label as u8 == 1 { 255 } else {0},
                                       if *label as u8 == 2 { 255 } else {0})
                );
            }),
    )?;

    Ok(())
}
