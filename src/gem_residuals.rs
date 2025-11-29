use crate::types::*;
use crate::parser::{Config,parse_data};
use nalgebra as na;
use std::collections::HashMap;
use na::{Vector3,matrix};

const GEM: usize = 1; // GEM TO SHIFT

#[cfg_attr(feature = "hotpath", hotpath::measure)]
fn gradient(data: &mut Vec<Vec<f64>>) -> f64 {
    let epsilon: f64 = 1e-6;

    // Apply perturbations sequentially
    for row in data.iter_mut() {
        row[GEM] += epsilon;
    }

    let chi2p = chi2_fit_1d(&data);

    for row in data.iter_mut() {
        row[GEM] -= 2.0 * epsilon;
    }

    // Compute chi2 sequentially
    let chi2m = chi2_fit_1d(&data);

    for row in data.iter_mut() {
        row[GEM] += epsilon;
    }

    // Finite difference gradient
    (chi2p - chi2m) / (2.0 * epsilon)
}

fn collect_errors(data: &Vec<Vec<f64>>) -> Vec<Vec<f64>>{

    let mut errors: Vec<Vec<f64>> = Vec::with_capacity(data.len());
    for (j,datas )in data.iter().enumerate(){
        
        let x: Vec<f64> = (0..3).map(|i| {
            let x1 = datas[(i + 1) % 3];
            let x2 = datas[(i + 2) % 3];
            let x3 = datas[i];

            let z1 = Z[(i + 1) % 3];
            let z2 = Z[(i + 2) % 3];
            let z3 = Z[i];

            let slope = (x1 - x2) / (z1 - z2);
            let intercept = x1 - z1 * slope;
            (slope * z3 + intercept) - x3
        }).collect();

        errors[j] = x;
    }
     
    errors
}


// TODO rewrite this for 1D
fn chi2_fit_1d(data: &Vec<Vec<f64>>) -> f64 {
    let n_pts: usize = 3;
    let mut total_chi_sqr: f64 = 0.0;
    let z_vec = Vector3::new(Z[0],Z[1],Z[2]);

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

            total_chi_sqr += (Z[i] - pred_x).powi(2) / sigma.powi(2);
        }
    }

    total_chi_sqr /= data.len() as f64;

    total_chi_sqr


}
// returns shift in X and Y
fn gradient_descent(data: &mut Vec<Vec<f64>>) -> (f64, u64) 
{
    let learning_rate: f64 = 5e-5;
    let tol: f64 = 1e-3;
    let num_itr: u64 = 4000;
    let mut grad: f64 = 99999.9;
    let mut shift: f64  = 0.0;
    let mut counter: u64 = 1;

    while grad.abs() > tol  && num_itr > counter
    {
        
        grad = gradient(data); 
            if counter % 10 == 0 {
        let chi2 = chi2_fit_1d(data);
        println!(
            "Iter {:4}: grad=({:+.6e}), shift=({:+.4}), χ²=({:.6})",
            counter, grad, shift, chi2
        );
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

pub fn align_gems(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let (event_nums, mut x_vals, mut y_vals) = parse_data(&config);

    println!("Data and config successfully parsed! Beginning data correction:");

    let (output_x, output_y) = rayon::join(
        || gradient_descent(&mut x_vals),
        || gradient_descent(&mut y_vals),
    );
    let shift_x = output_x.0;
    let counter_x = output_x.1;

    let shift_y = output_y.0;
    let counter_y = output_y.1;
    println!("GEM 2 shifted {} mm in X, {} mm in Y, after {} iterations.", shift_x, shift_y, counter_y);

    Ok(())

}
