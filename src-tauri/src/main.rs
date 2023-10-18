// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod constants;
use avmlib::constants::*;
//use avmlib::dot_product::*;
extern crate nalgebra as na;
use na::{U2, U3, Dyn, ArrayStorage, VecStorage, Matrix, Matrix3x4, RowVector4, Vector, Matrix1x4, DMatrix, Vector3, RowVector3, MatrixXx3, Matrix4x3, Vector4};
/*
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/


struct DenseLayer {
    weights: Matrix3x4<Precision>,
    biases: RowVector3<Precision>
}

impl Default for DenseLayer {
    fn default() -> Self {
        Self {

            weights : Matrix3x4::new_random(),
            biases : RowVector3::zeros(),

            //TODO : Put this in tests !
            /*
            weights : Matrix3x4::new(
                    0.2, 0.8, -0.5, 1.0,
                    0.5, -0.91, 0.26, -0.5,
                    -0.26, -0.27, 0.17, 0.87
            ),
            biases : RowVector3::new(2.0, 3.0, 0.5),
             */

        }
    }
}
impl DenseLayer{
    fn forward(&self, inputs: &Matrix3x4<Precision>) -> Matrix3x4<Precision> {

        //Convert unique vector (x, y, y) to :
        /*
          ┌       ┐
          │ x y z │
          │ x y z │
          │ x y z │
          │ x y z │
          └       ┘
         */
        let bs = MatrixXx3::from_rows(&[self.biases, self.biases, self.biases]);

        println!("bs{}", bs);
        println!("Weights{}", self.weights);
        let transposed = self.weights.transpose();
        println!("Transposed{}", transposed);

        let result = inputs * &transposed + &bs;
        //let result = &mul + &bs;

        println!("Result{}", result);




        let w = Matrix3x4::zeros();
        return w;
    }
}


fn main() {

    println!("Start");

    let inputs = Matrix3x4::from_rows(&[
        RowVector4::new(1.0, 2.0, 3.0, 2.5),
        RowVector4::new(2.0, 5.0, -1.0, 2.0),
        RowVector4::new(-1.5, 2.7, 3.3, -0.8),
    ]);

    println!("inputs : {}", inputs);


    let layer1 = DenseLayer::default();
    layer1.forward(&inputs);




    /*
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
     */

}
