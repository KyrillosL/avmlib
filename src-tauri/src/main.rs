// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod constants;

use std::ops::Mul;
use avmlib::constants::*;
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


fn main() {

    println!("Start");

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




    /*
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
     */

}
