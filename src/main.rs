mod gem_residuals;
mod types;
mod angles;
mod parser;

use std::io::{BufWriter, Write};
use crate::types::Event;




#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [99]))]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // read config.toml
    let config = parser::parse_config()?;

    let (event_nums, x, y, events): (Vec<u32>, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Event>) = gem_residuals::align_gems(&config)?;
    println!["GEMs aligned!"];

    let z = vec![0.0,180.0,700.0];
    angles::calculate_angles(&config,&event_nums, &x, &y, &z, &events)?;

    Ok(())
}

