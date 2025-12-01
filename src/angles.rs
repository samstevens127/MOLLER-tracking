use nalgebra as na;
use na::matrix;
use crate::parser::Config;
use std::f64::consts::PI;

fn get_eigenvector(x: &Vec<f64>, y: &Vec<f64>, z: &Vec<f64>) -> nalgebra::RowVector3<f64>{
        let x_mat = matrix![
            x[0], y[0], z[0];
            x[1], y[1], z[1];
            x[2], y[2], z[2]
        ];

        let svd_x = x_mat.svd(true, true);

        let v = svd_x.v_t.as_ref().unwrap().row(0).into_owned();

        v
}


pub fn calculate_angles(config: &Config,
                        event_nums: &Vec<u32>, x_events: &Vec<Vec<f64>>, y_events: &Vec<Vec<f64>>, z: &Vec<f64>
                        )-> Result<(), Box<dyn std::error::Error>>{
    for i in 0..x_events.len() {
        let x = &x_events[i];
        let y = &y_events[i];

        let eigenvector: nalgebra::RowVector3<f64> = get_eigenvector(&x,&y,&z);

        let angle = eigenvector[0].atan2(eigenvector[2]) * 180.0 / PI;
        let angle_x = eigenvector[0].acos() * 180.0 / PI; 
        let angle_y = eigenvector[1].acos() * 180.0 / PI;
    }

    Ok(())
}
