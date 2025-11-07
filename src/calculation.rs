use crate::types::*;

use nalgebra as na;
use std::collections::HashMap;
use na::{Vector3,matrix};

const GEM: usize = 1; // GEM TO SHIFT

fn gradient_2d(data: &HashMap<u32, Vec<Event>>) -> Vec<f64> {
    let epsilon = 1e-6;

    // Perturb x 
    let mut plus_x = data.clone();
    let mut minus_x = data.clone();
    for events in plus_x.values_mut() {
        events[GEM].x += epsilon;
    }
    for events in minus_x.values_mut() {
        events[GEM].x -= epsilon;
    }

    // Perturb y 
    let mut plus_y = data.clone();
    let mut minus_y = data.clone();
    for events in plus_y.values_mut() {
        events[GEM].y += epsilon;
    }
    for events in minus_y.values_mut() {
        events[GEM].y -= epsilon;
    }

    let chi2p_x = chi2_fit(&plus_x);
    let chi2m_x = chi2_fit(&minus_x);
    let chi2p_y = chi2_fit(&plus_y);
    let chi2m_y = chi2_fit(&minus_y);

    vec![
        (chi2p_x[0] - chi2m_x[0]) / (2.0 * epsilon),
        (chi2p_y[1] - chi2m_y[1]) / (2.0 * epsilon),
    ]
}

pub fn collect_errors(data: &HashMap<u32, Vec<Event>>, errors: &mut HashMap<u32, Error>) {

    for (event_num, datas) in data{
        
        let x: Vec<f64> = (0..3).map(|i| {
            let x1 = datas[(i + 1) % 3].x;
            let x2 = datas[(i + 2) % 3].x;
            let x3 = datas[i].x;

            let z1 = Z[(i + 1) % 3];
            let z2 = Z[(i + 2) % 3];
            let z3 = Z[i];

            let slope = (x1 - x2) / (z1 - z2);
            let intercept = x1 - z1 * slope;
            (slope * z3 + intercept) - x3
        }).collect();

        let y: Vec<f64> = (0..3).map(|i| {
            let y1 = datas[(i + 1) % 3].y;
            let y2 = datas[(i + 2) % 3].y;
            let y3 = datas[i].y;

            let z1 = Z[(i + 1) % 3];
            let z2 = Z[(i + 2) % 3];
            let z3 = Z[i];

            let slope = (y1 - y2) / (z1 - z2);
            let intercept = y1 - z1 * slope;
            (slope * z3 + intercept) - y3
        }).collect();
        errors.insert(*event_num, Error{x, y});
    }
     
}


fn chi2_fit(data: &HashMap<u32, Vec<Event>>) -> Vec<f64> {
    let n_pts: usize = 3;
    let mut total_chi_sqr: Vec<f64> = vec![0.0, 0.0];
    let z_vec = Vector3::new(Z[0],Z[1],Z[2]);

    for (_, event) in data {
        let x_mat = matrix![
            event[0].x, 1.0;
            event[1].x, 1.0;
            event[2].x, 1.0
        ];
        let y_mat = matrix![
            event[0].y, 1.0;
            event[1].y, 1.0;
            event[2].y, 1.0
        ];


        let svd_x = x_mat.svd(true, true);
        let svd_y = y_mat.svd(true, true);

        let sln_x = svd_x.solve(&z_vec, 1e-10).expect("SVD solve failed for x");
        let sln_y = svd_y.solve(&z_vec, 1e-10).expect("SVD solve failed for y");

        let slope_x = sln_x[0];
        let intercept_x = sln_x[1];

        let slope_y = sln_y[0];
        let intercept_y = sln_y[1];

        let sigma: f64 = 1.0;

        for i in 0..n_pts {
            let pred_x = slope_x * event[i].x + intercept_x;
            let pred_y = slope_y * event[i].y + intercept_y;

            total_chi_sqr[0] += (Z[i] - pred_x).powi(2) / sigma.powi(2);
            total_chi_sqr[1] += (Z[i] - pred_y).powi(2) / sigma.powi(2);
        }
    }

    total_chi_sqr[0] /= data.len() as f64;
    total_chi_sqr[1] /= data.len() as f64;


    total_chi_sqr
}

// returns shift in X and Y
pub fn gradient_descent(data: &mut HashMap<u32, Vec<Event>>) -> (Vec<f64>, u64) 
{
    let learning_rate: f64 = 5e-5;
    let tol: f64 = 1e-3;
    let num_itr: u64 = 4000;
    let mut grad: Vec<f64> = vec![99999.9,99999.9];
    let mut shift: Vec<f64>  = vec![0.0,0.0];
    let mut counter: u64 = 1;

    while (grad[0].abs() > tol || grad[1].abs() > tol) && num_itr > counter
    {
        
        grad = gradient_2d(data); 
            if counter % 10 == 0 {
        let chi2 = chi2_fit(data);
        println!(
            "Iter {:4}: grad=({:+.6e}, {:+.6e}), shift=({:+.4}, {:+.4}), χ²=({:.6}, {:.6})",
            counter, grad[0], grad[1], shift[0], shift[1], chi2[0], chi2[1]
        );
    }

        for (_ , events) in &mut *data
        {
            events[GEM].x -= learning_rate * grad[0];
            events[GEM].y -= learning_rate * grad[1];
        }


        shift[0] -= learning_rate * grad[0];
        shift[1] -= learning_rate * grad[1];
        counter += 1;
    }
    (shift,counter)
}
