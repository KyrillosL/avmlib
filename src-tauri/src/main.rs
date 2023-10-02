// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {

    println!("Start");
    /*
    let output = inputs[0] * weights[0] + inputs[1] * weights[1] + inputs[2] * weights[2] + bias;
    println!("output : {}", output);
    */

    type precision = f64;

    const SIZE : usize = 3;
    let inputs: [precision; SIZE] = [1.0, 2.0, 3.0];
    let weights: [precision; SIZE] = [0.2, 0.8, -0.5];
    let bias = 2.0;

    let mut output2 = bias;
    for i in 0..SIZE{
        output2 += inputs[i] * weights[i];
    }
    println!("output2 : {}", output2);

    let mut output3 = 0.0;
    for i in 0..SIZE{
        output3 += inputs[i] * weights[i];
    }
    output3 += bias;
    println!("output3 : {}", output3);


    /*
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
     */
}
