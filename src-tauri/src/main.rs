// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod constants;
use avmlib::constants::*;
use avmlib::dot_product::*;
extern crate nalgebra as na;
use na::{U2, U3, Dynamic, ArrayStorage, VecStorage, Matrix, Matrix3x4, RowVector4, Vector, Matrix1x4, DMatrix, Vector3, RowVector3, MatrixXx3};
/*
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/



struct Neuron {
    //weights: Vec<Precision>,
    weights : RowVector4<Precision>,
    bias: Precision
}

struct Layer {
    neurons: [Neuron; N_NEURONS]
}

impl Layer /* for Layer*/ {
    fn compute(&self, inputs: &Matrix3x4<Precision>) -> Matrix3x4<Precision> {
        let mut w = Matrix3x4::zeros();
        //let mut b = Matrix3x4::zeros();
        let mut b = RowVector3::new(0.0,0.0,0.0); //Not implemented yet
        for i in 0..N_NEURONS{
            w.set_row(i, &self.neurons[i].weights);
            b[i] = self.neurons[i].bias;
        }
        println!("bias{}", b);
        //let bna = Matrix1x4::from_vec(b);
        //let mut bna = DMatrix::from_vec(1, 4, b);

        let bs = MatrixXx3::from_rows(&[b, b, b]);
        println!("bs{}", bs);

        println!("Weights{}", w);
        let transposed = w.transpose();
        println!("Transposed{}", transposed);

        let result = inputs * &transposed + bs;
        //let result = &mul + &bs;

        println!("Result{}", result);
        return w;
    }
}

fn compute(layer: &Layer, input: &Vec<Precision>) -> Vec<Precision> {
    /*
    let use_native: bool = input.len() < 8;
    if use_native {
        println!("Warning, using naive_native_rust, the len of the input vector is < 8");
    }
    let mut output :Vec<Precision> = vec![];
    for n in 0..N_NEURONS {
        if use_native{
            output.push(naive_native_rust(&layer.neurons[n].weights, input) + layer.neurons[n].bias);
        }
        else {
            output.push(dot_product_simd(&layer.neurons[n].weights, input) + layer.neurons[n].bias);
        }

    }
    return output;
     */
    return vec![1.0, 2.0];
}



fn main() {

    println!("Start");
    //let inputs: Vec<4, Precision> = vec![1.0, 2.0, 3.0, 2.5];

    let inputs = Matrix3x4::from_rows(&[
        RowVector4::new(1.0, 2.0, 3.0, 2.5),
        RowVector4::new(2.0, 5.0, -1.0, 2.0),
        RowVector4::new(-1.5, 2.7, 3.3, -0.8),
    ]);

    println!("inputs : {}", inputs);
/*
    let weights = Matrix3x4::from_rows(&[
        RowVector4::new(1.0, 2.0, 3.0, 2.5),
        RowVector4::new(2.0, 5.0, -1.0, 2.0),
        RowVector4::new(-1.5, 2.7, 3.3, -0.8),
    ]);
*/
    //let r = mat1.dot(&mat2);



    /*
    //let neuron_1 = Neuron {weights: vec![0.2, 0.8, -0.5, 1.0], bias: 2.0 };
    //let neuron_2 = Neuron {weights: vec![0.5, -0.91, 0.26, -0.5], bias: 3.0 };
    //let neuron_3 = Neuron {weights: vec![-0.26, -0.27, 0.17, 0.87], bias: 0.5 };
*/
    let neuron_1 = Neuron {weights: RowVector4::new(0.2, 0.8, -0.5, 1.0), bias: 2.0 };
    let neuron_2 = Neuron {weights: RowVector4::new(0.5, -0.91, 0.26, -0.5), bias: 3.0 };
    let neuron_3 = Neuron {weights: RowVector4::new(-0.26, -0.27, 0.17, 0.87), bias: 0.5 };

    let layer1 = Layer { neurons : [neuron_1, neuron_2, neuron_3]};
    layer1.compute(&inputs);
    //let result = compute(&layer1, &inputs);
    //println!("output : {;?}", result);
    //println!("{:?}", result);




/*
    let weights: [Precision; SIZE_INPUT] = [0.2, 0.8, -0.5];
    let bias = 2.0;

    let mut output = bias;
    for i in 0..SIZE_INPUT {
        output += inputs[i] * weights[i];
    }
    println!("output2 : {}", output);
*/


    /*
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
     */

}
