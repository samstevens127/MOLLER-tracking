use std::fs::File;
use std::io::{self, BufWriter, Write};
use crate::parser::{Config,parse_data};
use nalgebra as na;
use na::{Vector3,matrix};
use crate::Event;

const GEM: usize = 1; // GEM TO SHIFT


// gradient with midpoint method
fn gradient(data: &mut Vec<Vec<f64>>, z: &Vec<f64>) -> f64 {
    let epsilon: f64 = 1e-6;

    for row in data.iter_mut() {
        row[GEM] += epsilon;
    }

    let chi2p = chi2_fit(&data, &z);

    for row in data.iter_mut() {
        row[GEM] -= 2.0 * epsilon;
    }

    let chi2m = chi2_fit(&data, &z);

    for row in data.iter_mut() {
        row[GEM] += epsilon;
    }

    (chi2p - chi2m) / (2.0 * epsilon)
}

//collect residuals
fn collect_errors(data: &Vec<Vec<f64>>, z: &Vec<f64>) -> Vec<Vec<f64>>{

    let mut errors: Vec<Vec<f64>> = Vec::with_capacity(data.len());
    for datas in data.iter(){
        
        let x: Vec<f64> = (0..3).map(|i| {
            let x1 = datas[(i + 1) % 3];
            let x2 = datas[(i + 2) % 3];
            let x3 = datas[i];

            let z1 = z[(i + 1) % 3];
            let z2 = z[(i + 2) % 3];
            let z3 = z[i];

            let slope = (x1 - x2) / (z1 - z2);
            let intercept = x1 - z1 * slope;
            (slope * z3 + intercept) - x3
        }).collect();

        errors.push(x);
    }
     
    errors
}


// return average chi2 value of fit for each event
fn chi2_fit(data: &Vec<Vec<f64>>, z: &Vec<f64>) -> f64 {
    let n_pts: usize = 3;
    let mut total_chi_sqr: f64 = 0.0;
    let z_vec = Vector3::new(z[0],z[1],z[2]);

    for event in data {
        let x_mat = matrix![
            event[0], 1.0;
            event[1], 1.0;
            event[2], 1.0
        ];

        let svd_x = x_mat.svd(true, true);

        let sln_x = svd_x.solve(&z_vec, 1e-10).expect("SVD solve failed for x");

        let slope_x = sln_x[0];
        let intercept_x = sln_x[1];

        let sigma: f64 = 1.0;

        for i in 0..n_pts {
            let pred_x = slope_x * event[i] + intercept_x;

            total_chi_sqr += (z[i] - pred_x).powi(2) / sigma.powi(2);
        }
    }

    total_chi_sqr /= data.len() as f64;

    total_chi_sqr


}
// returns shift in X and Y
// TODO: take num_itr and learning_rate as parameters
fn gradient_descent(data: &mut Vec<Vec<f64>>, z: &Vec<f64>) -> (f64, u64) 
{
    let learning_rate: f64 = 5e-5;
    let tol: f64 = 1e-3;
    let num_itr: u64 = 4000;
    let mut grad: f64 = 99999.9;
    let mut shift: f64  = 0.0;
    let mut counter: u64 = 1;

    while grad.abs() > tol  && num_itr > counter
    {
        
        grad = gradient(data, &z); 
            if counter % 10 == 0 {
        let chi2 = chi2_fit(data, &z);
        print!(
            "\rIter {:4}: grad=({:+.6e}), shift=({:+.4}), χ²=({:.6})",
            counter, grad, shift, chi2
        );
        io::stdout().flush().unwrap();
    }

        for  event in data.iter_mut()
        {
            event[GEM] -= learning_rate * grad;
        }


        shift -= learning_rate * grad;
        counter += 1;
    }
    (shift,counter)
}

// TODO make it take z vector
pub fn align_gems(config: &Config) -> Result<(Vec<u32>, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Event>), Box<dyn std::error::Error>> {

    let (event_nums, mut x_vals, mut y_vals, events) = parse_data(&config);

    let z: &Vec<f64> = &config.residuals.z;

    println!("Data and config successfully parsed! Beginning data correction:");

    let (output_x, output_y) = rayon::join(
        || gradient_descent(&mut x_vals, &z),
        || gradient_descent(&mut y_vals, &z),
    );
    let shift_x = output_x.0;
    let counter_x = output_x.1;

    let shift_y = output_y.0;
    let counter_y = output_y.1;
    io::stdout().flush().unwrap();
    println!("\rGEM 2 shifted {:3} mm in X, {:3} mm in Y, after {} iterations.", shift_x, shift_y, counter_y + counter_x);

    let (x_err, y_err) = rayon::join(
        || collect_errors(&x_vals, &z),
        || collect_errors(&y_vals, &z),
    );


    let f_residuals = format!("{}/x_y_residuals_{}.txt",config.residuals.outpath, config.residuals.run_num);
    let f_corrected = format!("{}/x_y_corrected_{}.txt",config.residuals.outpath, config.residuals.run_num);
    let residuals = File::create(f_residuals)?;
    let corrected = File::create(f_corrected)?;

    let mut residuals_writer = BufWriter::new(residuals);
    let mut corrected_writer = BufWriter::new(corrected);

    for i in 0..event_nums.len() {
        writeln!(residuals_writer, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t", event_nums[i], x_err[i][0],y_err[i][0],z[0], x_err[i][1],y_err[i][1],z[1], x_err[i][2],y_err[i][2],z[2])?;
        writeln!(corrected_writer, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t", event_nums[i], x_vals[i][0],y_vals[i][0],z[0], x_vals[i][1],y_vals[i][1],z[1], x_vals[i][2],y_vals[i][2],z[2], events[i].hadc,events[i].ladc,events[i].hv,events[i].run_num)?;
    }
    writeln!(residuals_writer, "event_nums\tx_err_1\ty_err_1\tz_1\tx_err_2\ty_err_2\tz_2\tx_err_3\ty_err_3\tz_3\t")?;
    //writeln!(corrected_writer, "event_nums\tx1\ty1\tz1\tx2\ty2\tz2\tx3\ty3\tz3\thadc\tladc\thv\trun_num")?;
    Ok((event_nums, x_vals, y_vals, events))
}
