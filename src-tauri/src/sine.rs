extern crate nalgebra as na;

pub fn create_data_na(samples: usize) -> (na::DMatrix<f64>, na::DMatrix<f64>) {
    let mut x = na::DMatrix::zeros(samples, 1);
    let mut y = na::DMatrix::zeros(samples, 1);

    for i in 0..samples {
        let x_val = i as f64 / samples as f64;
        let y_val = f64::sin(2.0 * std::f64::consts::PI * x_val);
        x[(i, 0)] = x_val;
        y[(i, 0)] = y_val;
    }

    (x, y)
}

pub fn create_data(samples: usize) -> (Vec<f64>, Vec<f64>) {
    let mut x = Vec::with_capacity(samples);
    let mut y = Vec::with_capacity(samples);

    for i in 0..samples {
        let x_val = i as f64 / samples as f64;
        let y_val = f64::sin(2.0 * std::f64::consts::PI * x_val);
        x.push(x_val);
        y.push(y_val);
    }

    (x, y)
}