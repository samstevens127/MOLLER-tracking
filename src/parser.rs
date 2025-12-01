use std::fs::{File,read_to_string};
use std::io::{BufRead, BufReader};

use crate::types::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config{
    data_file: DataFile,
}

#[derive(Debug,Deserialize)]
struct DataFile {
    data_path: String,
    filename: String,
}

pub fn parse_config() -> Result<Config, Box<dyn std::error::Error>>
{
        let contents = read_to_string("config.toml")?;
        let config: Config = toml::from_str(&contents)?;
    
        Ok(config)
}

fn next_event(reader: &mut BufReader<File>) -> Option<(u32,f64,f64,Event)> // returns eventnum and event
{
    let mut line = String::new();

    loop {
        line.clear();
        if reader.read_line(&mut line).ok()? == 0{
            return None;
        }

        let data: Vec<&str> = line.split_whitespace().collect();

        let event_num: u32 = match data[0].parse() {Ok(v) => v, Err(_) => break};
        let x: f64 = match data[1].parse::<f64>() {Ok(v) => v * 0.390625, Err(_) => break};
        let y: f64 = match data[2].parse::<f64>() {Ok(v) => v * 0.390625, Err(_) => break};
        let _x_charge: f64  = match data[3].parse() {Ok(v) => v, Err(_) => break};
        let _y_charge: f64  = match data[4].parse() {Ok(v) => v, Err(_) => break};
        let hadc: u16      = match data[5].parse() {Ok(v) => v, Err(_) => break};
        let ladc: u16      = match data[6].parse() {Ok(v) => v, Err(_) => break};
        let run_num: u16   = match data[7].parse() {Ok(v) => v, Err(_) => break};
        let hv: u16        = match data[8].parse() {Ok(v) => v, Err(_) => break};

        return Some((event_num,x, y, Event { hadc, ladc, hv, run_num}));
        
    }

    return None;
}


pub fn parse_data(config: &Config) -> (Vec<u32>, Vec<Vec<f64>>,Vec<Vec<f64>>, Vec<Event>)
{

        let filenames = vec![
        format!("{}/{}_x1y1.txt",config.data_file.data_path, config.data_file.filename),
        format!("{}/{}_x2y2.txt",config.data_file.data_path, config.data_file.filename),
        format!("{}/{}_x3y3.txt",config.data_file.data_path, config.data_file.filename)
    ];


    let mut x: Vec<Vec<f64>> = Vec::new();
    let mut y: Vec<Vec<f64>> = Vec::new();
    let mut adcs: Vec<Event> = Vec::new();
    let mut event_nums: Vec<u32> = Vec::new();

    let mut readers: Vec<_> = filenames
        .iter()
        .map(|f| BufReader::new(File::open(f).unwrap()))
        .collect();

    // Get the first event from each file
    let mut current: Vec<Option<(u32,f64,f64, Event)>> = readers
        .iter_mut()
        .map(|r| next_event(r))
        .collect();


    /* read input files and sort for events that hit all GEMs and populate HashMap events with them*/
    while current.iter().all(|c| c.is_some()){
        let event_num: Vec<u32> = current.iter().map(|c| c.as_ref().unwrap().0).collect();
        let adc: Vec<Event> = current.iter().map(|c| c.as_ref().unwrap().3).collect();
        let min_event = *event_num.iter().min().unwrap();
        let max_event = *event_num.iter().max().unwrap();

        if min_event == max_event {
            // Found common event
            let x_event: Vec<f64> = current.iter().map(|c| c.as_ref().unwrap().1.clone()).collect();
            let y_event: Vec<f64> = current.iter().map(|c| c.as_ref().unwrap().2.clone()).collect();

             
            event_nums.push(event_num[0]);
            adcs.push(adc[0]);
            x.push(x_event);
            y.push(y_event);

            // Advance all readers
            for (i, r) in readers.iter_mut().enumerate() {
                current[i] = next_event(r);
            }
        } else {
            // Advance readers pointing to the smallest event number
            for (i, c) in current.iter_mut().enumerate() {
                if c.as_ref().unwrap().0 == min_event {
                    *c = next_event(&mut readers[i]);
                }
            }
        }
    }
    println!("{} Events hit all 3 GEMs.", x.len());

    (event_nums, x, y, adcs)
}
