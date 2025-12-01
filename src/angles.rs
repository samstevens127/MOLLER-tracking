use nalgebra as na;
use na::matrix;
use crate::parser::Config;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::Event;

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
                        event_nums: &Vec<u32>, x_events: &Vec<Vec<f64>>, y_events: &Vec<Vec<f64>>, z: &Vec<f64>, events: &Vec<Event>
                        )-> Result<(), Box<dyn std::error::Error>>{
    let mut angles = Vec::with_capacity(event_nums.len());
    let mut angles_x = Vec::with_capacity(event_nums.len());
    let mut angles_y = Vec::with_capacity(event_nums.len());

    for i in 0..x_events.len() {
        let x = &x_events[i];
        let y = &y_events[i];

        let eigenvector: nalgebra::RowVector3<f64> = get_eigenvector(&x,&y,&z);

        let angle = eigenvector[0].atan2(eigenvector[2]) * 180.0 / PI;
        let angle_x = eigenvector[0].acos() * 180.0 / PI; 
        let angle_y = eigenvector[1].acos() * 180.0 / PI;

        angles.push(angle);
        angles_x.push(angle_x);
        angles_y.push(angle_y);
    }

    let angle_file = File::create(format!("{}/angles_{}.txt", config.angles.outpath, config.angles.run_num))?;
    let mut angle_writer = BufWriter::new(angle_file);

    for i in 0..event_nums.len() {
        writeln![angle_writer, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t", event_nums[i], angles[i], angles_x[i], angles_y[i],events[i].hadc,events[i].ladc,events[i].hv,events[i].run_num]?;
    }
        writeln![angle_writer, "event_num\tpolar_angle\tx_angle\ty_angle\thadc\tladc\thv\trun_num"]?;

    Ok(())
}
