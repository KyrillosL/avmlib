// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
//#[feature(core_intrinsics)]
//use std::intrinsics::logf64;

mod constants;
mod sine;

use std::iter::zip;
use std::ops::Mul;
use avmlib::constants::*;
use avmlib::sine::*;
use avmlib::spiral::*;

use std::ops::DivAssign;
use approx::assert_ulps_eq;
use num::clamp;

//use avmlib::dot_product::*;
extern crate nalgebra as na;
use na::{U2, U3, Dyn, ArrayStorage, VecStorage, Matrix, Matrix3x4, RowVector4, Vector, Matrix1x4, DMatrix, Vector3, RowVector3, MatrixXx3, Matrix4x3, Vector4, RowDVector, RowOVector};
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

fn SoftMax(r: usize, c: usize, inputs: &DMatrix<Precision>) -> DMatrix<Precision>{
    assert_eq!(r, inputs.nrows());
    assert_eq!(c, inputs.ncols());
    //TODO : use Apply()
    //m.apply(|v| v.exp());
    let mut e = inputs.map(|v| v.exp());
    println!("{}", &e);
    let v4 = DMatrix::<Precision>::from_fn(r, c, |i, _| e.row(i).iter().sum());

    for j in 0..r {
        e.row_mut(j).div_assign(v4[j])
    }
    return e;
}

fn ReluActivation(inputs: &DMatrix<Precision>) -> DMatrix<Precision>{
    //Todo : use inputs.apply();
    return  inputs.map(ReluFunction);
}

fn ReluFunction(x: Precision) -> Precision{
    return if x <= 0.0 {
        0.0
    } else {
        x
    }
}

fn ClipLoss(inputs: &DMatrix<Precision>) -> DMatrix<Precision>{
    let r = inputs.map(|x| clamp(x, 1e-7, 1.0-1e-7));
    r
}

//take [[1,0,0], [0,1,0], [0,1,0]] as target -> One Hot
fn LossSparse(inputs: &DMatrix<Precision>, targets:  &DMatrix<Precision>) -> Precision {
    assert!(targets.shape().0 > 1, "targets.shape().0 = {}, should be > = {}, Wrong target shape, Use LossCategorical?", targets.shape().0, 1);
    assert_eq!(inputs.nrows(), targets.nrows());
    assert_eq!(inputs.ncols(), targets.ncols());
    println!("inputs {}", inputs);
    println!("targets {}", targets);
    let clipped = ClipLoss(inputs);
    let confidence_matrix = clipped.component_mul(targets);
    println!("Confidence {}", confidence_matrix);
    let sum_confidence_matrix= confidence_matrix.column_sum();
    println!("sum_confidence_matrix {}", sum_confidence_matrix);
    let neg_log_matrix = sum_confidence_matrix.map(|x| -x.ln());
    println!("neg_log_matrix {}", neg_log_matrix);
    let r = neg_log_matrix.mean();
    println!("r {}", r);
    return r;
}

//take [0,1,1] as target -> Categorical
fn LossCategorical(inputs: &DMatrix<Precision>, targets:  &DMatrix<Precision>) -> Precision {
    assert_eq!(targets.shape().0, 1, "Wrong target shape, Use LossSparse?");
    assert_eq!(inputs.nrows(), targets.len());
    println!("inputs {}", inputs);
    println!("targets {}", targets);
    let clipped = ClipLoss(inputs);
    let mut sum_confidence_matrix =  RowDVector::<Precision>::zeros(targets.len());
    for i in 0..inputs.nrows(){
        sum_confidence_matrix[i] = clipped[(i, targets[i as usize] as usize)];
    }
    println!("sum_confidence_matrix {}", sum_confidence_matrix);
    let neg_log_matrix = sum_confidence_matrix.map(|x| -x.ln());
    println!("neg_log_matrix {}", neg_log_matrix);
    let r = neg_log_matrix.mean();
    println!("r {}", r);
    return r;
}


impl DenseLayer{
    fn new(batch_size : usize, n_inputs: usize, n_neurons: usize) -> Self {
        Self {
            batch_size: batch_size,
            n_inputs : n_inputs,
            n_neurons : n_neurons,

            //TODO : Put this in tests !
            /*


            */
            weights: DMatrix::new_random(n_neurons, n_inputs),
            biases: RowDVector::zeros(n_neurons),
        }
    }
    fn forward(&self, inputs: &DMatrix<Precision>, weights: &DMatrix<Precision>, biases: &RowDVector<Precision>) -> DMatrix<Precision> {

        //Convert unique vector (x, y, z) to :
        //For a 4*3 input :
        /*
          ┌       ┐
          │ x y z │
          │ x y z │
          │ x y z │
          └       ┘
         */
        //TODO -> use apply for biases
        let mut bs = DMatrix::<Precision>::zeros(self.batch_size, self.n_neurons);   //(&[self.biases]);
        for i in 0..self.batch_size{
            bs.set_row(i,  &biases);
        }
        println!("bs{}", bs);

        println!("Weights{}", weights);
        let transposed = weights.transpose();
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

    //TODO -> Use compile time !!
    env::set_var("RUST_BACKTRACE", "1");
    println!("Start");

    //TODO : FOR NOW BATCH_SIZE == n_samples == 100 -> 2 batchs of 50 ?
    let n_samples_inputs = 2;
    let n_neurons_layer1 = 2;
    let n_neurons_layer2 = 3;
    let n_class = 3;
    let n_generated_samples_dataset = 12;

    let batch_size = n_generated_samples_dataset;

    //This will create n samples per class
    let (x, y, c) = create_data_spiral(n_generated_samples_dataset,n_class);
    let _  = visualize(&x, &y, &c);
    println!("X: {:?}", &x);
    println!("y: {:?}", &y);
    println!("c: {:?}", &c);

    let inputs = get_inputs(&x, &y, n_samples_inputs, batch_size);


    //n_neurons MUST BE == WITH input_batch_size

    //atm the batch size is linked to the number of neurons
    //The batch_size is just used for the biases to be added... Because nalgebra can add a vector to a matrix
    let layer1 = DenseLayer::new( batch_size, n_samples_inputs, n_neurons_layer1);
    let lf1 = layer1.forward(&inputs, &layer1.weights, &layer1.biases);
    let activated1 = ReluActivation(  &lf1);
    println!("{}", activated1);

    let layer2 = DenseLayer::new( batch_size, n_neurons_layer1, n_neurons_layer2);
    let lf2 = layer2.forward(&activated1, &layer2.weights, &layer2.biases);
    let softed_max = SoftMax(batch_size,n_neurons_layer2,&lf2);
    println!("{}", softed_max);

    /*
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
     */

    //println!("{}", Relu(2.0));

}


#[cfg(test)]
mod tests {
    use na::{DMatrix, RowDVector};
    use crate::{DenseLayer, LossSparse, LossCategorical};
    use approx::relative_eq;

    /*
    #[test]
    fn test_model() {

        let n_samples_inputs = 4;
        let n_neurons_layer1 = 3;
        let n_neurons_layer2 = 3;
        let batch_size = 3;

        let inputs = DMatrix::from_row_slice(batch_size,n_samples_inputs, &[
            1.0, 2.0, 3.0, 2.5,
            2.0, 5.0, -1.0, 2.0,
            -1.5, 2.7, 3.3, -0.8,
        ]);
        println!("inputs : {}", inputs);

        let weights = DMatrix::from_row_slice(n_neurons_layer1, n_samples_inputs,  &[
            0.2, 0.8, -0.5, 1.0,
            0.5, -0.91, 0.26, -0.5,
            -0.26, -0.27, 0.17, 0.87
        ]);
        let biases = RowDVector::from_vec(vec![2.0, 3.0, 0.5]);

        let weights2 = DMatrix::from_row_slice(batch_size, n_neurons_layer1,  &[
            0.1, -0.14, 0.5,
            -0.5, 0.12, -0.33,
            -0.44, 0.73, -0.13,
        ]);
        let biases2 = RowDVector::from_vec(vec![-1.0, 2.0, -0.5]);

        let layer1 = DenseLayer::new(batch_size, n_samples_inputs, n_neurons_layer1);
        let lf1 = layer1.forward(&inputs, &weights, &biases);

        let layer2 = DenseLayer::new(batch_size, n_neurons_layer1, n_neurons_layer1);
        let lf2 = layer1.forward(&lf1, &weights2, &biases2);

        let result = DMatrix::from_row_slice(n_neurons_layer1, n_neurons_layer2,  &[
            0.5031, -1.04185, -2.03875,
            0.2434, -2.7332, -5.7633,
            -0.99314, 1.41254, -0.35655,
        ]);

        relative_eq!(lf2, result, epsilon = f64::EPSILON);
    }
    */

    #[test]
    fn test_loss_sparse() {
        let outputs = DMatrix::from_row_slice(3,3, &[
            0.7, 0.1, 0.2,
            0.1, 0.5, 0.4,
            0.02, 0.9, 0.08,
        ]);
        let targets = DMatrix::from_row_slice(3,3, &[
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0
        ]);
        let r = LossSparse(&outputs, &targets);
        //assert_eq!(r, 0.38506088005216804, "{}", epsilon = f64::EPSILON);
        assert_eq!(r, 0.38506088005216804);
    }

    #[test]
    fn test_loss_categorical() {
        let outputs = DMatrix::from_row_slice(3,3, &[
            0.7, 0.1, 0.2,
            0.1, 0.5, 0.4,
            0.02, 0.9, 0.08,
        ]);
        let targets = DMatrix::from_row_slice(1,3, &[0.0, 1.0, 1.0]);
        let r = LossCategorical(&outputs, &targets);
        //assert_eq!(r, 0.38506088005216804, "{}", epsilon = f64::EPSILON);
        assert_eq!(r, 0.38506088005216804);
    }

    #[test]
    fn test_loss_ln() {
        let outputs = DMatrix::from_row_slice(1,3, &[
            0.0, 0.1, 0.2
        ]);
        let targets = DMatrix::from_row_slice(1,1, &[0.0]);
        let r = LossCategorical(&outputs, &targets);
        //assert_eq!(r, 0.38506088005216804, "{}", epsilon = f64::EPSILON);
        assert_eq!(r, 16.11809565095832);

        let outputs2 = DMatrix::from_row_slice(2,3, &[
            0.0, 0.1, 0.2,
            0.0, 0.1, 0.2
        ]);
        let targets2 = DMatrix::from_row_slice(2,3, &[
            1.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
        ]);
        let r2 = LossSparse(&outputs2, &targets2);
        //assert_eq!(r, 0.38506088005216804, "{}", epsilon = f64::EPSILON);
        assert_eq!(r2, 16.11809565095832);
    }
}