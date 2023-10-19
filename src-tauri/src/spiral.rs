use rand::Rng;

pub fn create_data_spiral(samples: usize, classes: usize) -> (Vec<(f64, f64)>, Vec<u8>) {
    let mut X = Vec::with_capacity(samples * classes);
    let mut y = Vec::with_capacity(samples * classes);

    let mut rng = rand::thread_rng();

    for class_number in 0..classes {
        for sample in 0..samples {
            let r = sample as f64 / samples as f64;
            let t = (class_number * 4) as f64 + rng.gen::<f64>() * 0.2;

            let x_val = r * f64::sin(t * 2.5);
            let y_val = r * f64::cos(t * 2.5);

            X.push((x_val, y_val));
            y.push(class_number as u8);
        }
    }

    (X, y)
}