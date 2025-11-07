mod calculation;
mod parser;
mod types;

use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader,BufWriter, Write};
use crate::types::*;
use crate::parser::*;
use crate::calculation::{gradient_descent,collect_errors};



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = parse_config()?;

    let mut events = parse_data(&config)?;
    let mut errors: HashMap<u32, Error> = HashMap::new();

    println!("Data and config successfully parsed! Beginning data correction:");

    let output = gradient_descent(&mut events);
    let shift = output.0;
    let counter = output.1;
    println!("GEM 2 shifted {} mm in X, {} mm in Y, after {} iterations.", shift[0], shift[1], counter);

    let outfile = File::create("corrected_x_y.txt")?;
    let mut writer = BufWriter::new(outfile);

    collect_errors(&events,&mut errors);
    
    for (event_num, elem) in errors{
        writeln!(writer, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", event_num, elem.x[0], elem.y[0], Z[0], elem.x[1], elem.y[1], Z[1], elem.x[2], elem.y[2], Z[2]);
    }

   
    Ok(())
}

