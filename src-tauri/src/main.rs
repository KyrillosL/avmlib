// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
mod constants;
mod sine;

use std::iter::zip;
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


fn ReluActivation(inputs: &DMatrix<Precision>) -> DMatrix<Precision>{
    return  inputs.map(ReluFunction);
}

fn ReluFunction(x: Precision) -> Precision{
    return if x <= 0.0 {
        0.0
    } else {
        x
    }
}


impl DenseLayer{
    fn new(batch_size : usize, n_inputs: usize, n_neurons: usize) -> Self {
        Self {
            batch_size: batch_size,
            n_inputs : n_inputs,
            n_neurons : n_neurons,

            //TODO : Put this in tests !
            /*
                        let inputs = DMatrix::from_row_slice(batch_size,n_samples_inputs, &[
                        1.0, 2.0, 3.0, 2.5,
                        2.0, 5.0, -1.0, 2.0,
                        -1.5, 2.7, 3.3, -0.8,
                        ]);
                        println!("inputs : {}", inputs);
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
    env::set_var("RUST_BACKTRACE", "1");
    println!("Start");


    //TODO : FOR NOW BATCH_SIZE == n_samples == 100 -> 2 batchs of 50 ?
    let n_samples_inputs = 2;
    let n_neurons = 2;
    let n_class = 3;
    let n_generated_samples_dataset = 12;

    let batch_size = n_generated_samples_dataset;

    //This will create n samples per class
    let (x, y, c) = create_data_spiral(n_generated_samples_dataset,n_class);
    let _  = visualize(&x, &y, &c);
    println!("X: {:?}", &x);
    println!("y: {:?}", &y);
    println!("c: {:?}", &c);



    //let zipped : Vec<(&f64, &f64)> = x.iter().zip(y.iter()).collect();
    let vecofvec : Vec<&Vec<f64>> = vec![&x, &y];
    assert_eq!(vecofvec.len(), n_samples_inputs, "The number of sample must be the same as the number of element in the vector");

    println!("VecOfvec{:?}", vecofvec);


    let mut inputs = DMatrix::<Precision>::zeros(n_samples_inputs, batch_size);
    for i in 0..n_samples_inputs{
        //TODO : DONT COPY !
        let row: RowDVector<Precision> =  RowDVector::from_vec( vecofvec[i].to_vec());
        let (shapex, shapey) = row.shape();
        //println!("{}, {}", shapex, shapey);
        inputs.set_row(i, &row);
    }
    inputs = inputs.transpose();
    println!("Inputs{}", inputs);

    //n_neurons MUST BE == WITH input_batch_size

    //atm the batch size is linked to the number of neurons
    let layer1 = DenseLayer::new( batch_size, n_samples_inputs, n_neurons); //3 rows, 4 cols
    let lf1 = layer1.forward(&inputs);
    let activated = ReluActivation(&lf1);
    println!("{}", activated);


    /*
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
     */

    //println!("{}", Relu(2.0));

}