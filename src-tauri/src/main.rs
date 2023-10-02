// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn first_neuron(){
    let inputs : [f64; 3]  = [1.0, 2.0, 3.0];
    let weights : [f64; 3]  = [0.2, 0.8, -0.5];
    let bias : f64 = 2.0;



}
fn main() {


    println!("test!");

/*
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
 */
}
