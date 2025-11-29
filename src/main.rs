mod gem_residuals;
mod parser;
mod types;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use rayon::prelude::*;
use crate::types::*;
use crate::parser::*;
use crate::gem_residuals::align_gems;




#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [99]))]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // read config.toml
    let config = parser::parse_config()?;

    gem_residuals::align_gems(&config)?;

    Ok(())
}

