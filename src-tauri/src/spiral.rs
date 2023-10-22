use rand::Rng;
use itertools::izip;
extern crate plotters;
use plotters::prelude::*;

pub fn create_data_spiral(samples: usize, classes: usize) -> (Vec<f64>, Vec<f64>, Vec<u32>) /* -> (Vec<(f64, f64)>, Vec<u8>)*/{

    let n = samples / classes;
    //println!("{}", n);
    assert_eq!(samples % classes, 0, "The number of samples must be a multiple of the number of class");

    let mut x = vec![0.0; samples];
    let mut y = vec![0.0; samples];
    let mut c = vec![0; samples];

    for j in 0..classes {
        //Range : 0->33, 33->66, 66->99
        let ix = n * j..n * (j+1);

        //Map
        let r = (0..n).map(|n2| n2 as f64 / n as f64);

        let mut rng = rand::thread_rng();

        let t = (0..samples)
            .map(|n2| {
                let t = j as f64 * 4.0 + rng.gen::<f64>() * 0.2;
                t + n2 as f64 * 4.0 / n as f64
            });

        for (i, (r_val, t_val)) in r.zip(t).enumerate() {
            let index = ix.start + i;
            x[index] = r_val * f64::sin(t_val);
            y[index] = r_val * f64::cos(t_val);
            c[index] = j as u32;
        }
    }


    /*
    let mut x = vec![0.0; samples*classes];
    let mut y = vec![0.0; samples*classes];
    let mut c = vec![0; samples*classes];

    for j in 0..classes {
        let ix = samples * j..samples * (j+1);
        let r = (0..samples).map(|n| n as f64 / samples as f64);
        let mut rng = rand::thread_rng();
        let t = (0..samples)
            .map(|n| {
                let t = j as f64 * 4.0 + rng.gen::<f64>() * 0.2;
                t + n as f64 * 4.0 / samples as f64
            });

        for (i, (r_val, t_val)) in r.zip(t).enumerate() {
            let index = ix.start + i;
            x[index] = r_val * f64::sin(t_val);
            y[index] = r_val * f64::cos(t_val);
            c[index] = j as u32;
        }
    }
    */
    /*
    println!("x{:?}", x);
    println!("y{:?}", y);
    println!("c{:?}", c);

    println!("{}", x.len());
    println!("{}", y.len());
    println!("{}", c.len());
    */
    (x, y, c)


}


pub fn visualize(x: &Vec<f64>, y: &Vec<f64>, c: &Vec<u32>) -> Result<(), Box<dyn std::error::Error>> {
    // Visualize the data
    let root =
        BitMapBackend::new("plots/scatter.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_min = -1.0;
    let x_max = 1.0;
    let y_min = -1.0;
    let y_max = 1.0;

    let ok : Vec<(&f64, &f64, &u32)> = izip!(x, y, c).collect();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_ranged(x_min..x_max, y_min..y_max)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart.draw_series(ok.iter().map(|(&a,&b, &c)| Circle::new((a,b), 3, plotters::style::RGBColor(
        if c as u8 == 0 { 255 } else {0},
        if c as u8 == 1 { 255 } else {0},
        if c as u8 == 2 { 255 } else {0})
    )));

    Ok(())
}

