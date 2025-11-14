mod calculation;
mod parser;
mod types;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use rayon::prelude::*;
use crate::types::*;
use crate::parser::*;
use crate::calculation::{gradient_descent,collect_errors};


#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [99]))]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // read config.toml
    let config = parse_config()?;

    let (event_nums, mut x_vals, mut y_vals) = parse_data(&config);
    let mut errors_x: Vec<Vec<f64>> = Vec::new();
    let mut errors_y: Vec<Vec<f64>> = Vec::new();

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

    let outfile = File::create("corrected_x_y.txt")?;
    let mut writer = BufWriter::new(outfile);

    collect_errors(&x_vals,&mut errors_x);
    collect_errors(&y_vals,&mut errors_y);
    
    for i in 0..x_vals.len() {
        writeln!(writer, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", event_nums[i], errors_x[i][0], errors_y[i][0], Z[0], errors_x[i][1], errors_y[i][1], Z[1], errors_x[i][2], errors_y[i][2], Z[2])?;
    }

    Ok(())
}

