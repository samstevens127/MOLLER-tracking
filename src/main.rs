mod gem_residuals;
mod angles;
mod parser;

use crate::parser::Event;

#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [99]))]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // read config.toml
    let config = parser::parse_config()?;

    let run_align: bool = config.residuals.exec;
    let run_angles: bool= config.angles.exec;
    

    if !(run_align || run_angles ) {
        panic!["Nothing to run! Set 'exec = true' for one of the fields in config.toml"];
    }

    if run_align{
        println!["Aligning GEMs..."];
        let (event_nums, x, y, events): (Vec<u32>, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Event>) = gem_residuals::align_gems(&config)?;

        if run_angles {
            println!["Calculating angles..."];
            let z = vec![0.0,180.0,700.0];
            angles::calculate_angles(&config,&event_nums, &x, &y, &z, &events)?;
        }
        return Ok(());
    }
    
    println!["Calculating angles..."];
    let (event_nums, x, y, z, events): (Vec<u32>, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<f64>, Vec<Event>) = parser::parse_align(&config)?;

    angles::calculate_angles(&config,&event_nums, &x, &y, &z, &events)?;
    println!["Done angles"];

    Ok(())
}

