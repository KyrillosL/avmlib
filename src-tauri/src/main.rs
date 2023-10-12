// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod constants;
use crate::constants::*;

mod dot_product;
use crate::dot_product::*;

/*
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/



struct Neuron {
    weights: Vec<Precision>,
    bias: Precision
}

struct Layer {
    neurons: [Neuron; N_NEURONS]
}

fn compute(layer: &Layer, input: &Vec<Precision>) -> Vec<Precision> {
    let mut output :Vec<Precision> = vec![];
    for n in 0..N_NEURONS {
        output.push(dot_product_simd(&layer.neurons[n].weights, input) + layer.neurons[n].bias);
    }
    return output;
}



fn main() {

    println!("Start");
    let inputs: Vec<Precision> = vec![1.0, 2.0, 3.0, 2.5];
    let neuron_1 = Neuron {weights: vec![0.2, 0.8, -0.5, 1.0], bias: 2.0 };
    let neuron_2 = Neuron {weights: vec![0.5, -0.91, 0.26, -0.5], bias: 3.0 };
    let neuron_3 = Neuron {weights: vec![-0.26, -0.27, 0.17, 0.87], bias: 0.5 };
    let layer1 = Layer { neurons : [neuron_1, neuron_2, neuron_3]};
    let result = compute(&layer1, &inputs);
    //println!("output : {;?}", result);
    println!("{:?}", result);


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
