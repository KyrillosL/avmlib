// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod benchmarks;



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

type Precision = f64;
const SIZE_INPUT: usize = 4;
const N_NEURONS : usize = 3;

struct Neuron {
    weights: [Precision; SIZE_INPUT],
    bias: Precision
}

struct Layer {
    neurons: [Neuron; N_NEURONS]
}

fn Compute(layer: &Layer, input: &[Precision; SIZE_INPUT]) -> Precision {
    let mut output = 0.0;
    for n in 0..N_NEURONS {
        let mut output_tmp = layer.neurons[n].bias;
        for i in 0..SIZE_INPUT {
            output_tmp += layer.neurons[n].weights[i] * input[i];
        }
        output += output_tmp;
    }
    return output;
}



fn main() {

    println!("Start");
    let inputs: [Precision; SIZE_INPUT] = [1.0, 2.0, 3.0, 2.5];
    let neuron_1 = Neuron {weights: [0.2, 0.8, -0.5, 1.0], bias: 2.0 };
    let neuron_2 = Neuron {weights: [0.5, -0.91, 0.26, -0.5], bias: 3.0 };
    let neuron_3 = Neuron {weights: [-0.26, -0.27, 0.17, 0.87], bias: 0.5 };
    let layer1 = Layer { neurons : [neuron_1, neuron_2, neuron_3]};
    let result = Compute(&layer1, &inputs);
    println!("output : {}", result);


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

    benchmarks::compute_benchmarks();
}
