use std::fs::{File,read_to_string};
use std::io::{BufRead, BufReader};

use serde::Deserialize;

#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub hadc: u16,
    pub ladc: u16,
    pub hv: u16,
    pub run_num: u16,
}

#[derive(Debug, Deserialize)]
pub struct Config{
    pub residuals: Residuals,
    pub angles: Angles,
}

#[derive(Debug,Deserialize)]
pub struct Residuals {
    pub exec : bool,
    pub outpath: String,
    pub run_num: String,
    pub z: Vec<f64>,
    inpath: String,
    infile: String,
}

#[derive(Debug,Deserialize)]
pub struct Angles {
    pub exec : bool,
    pub outpath: String,
    pub run_num: String
}

pub fn parse_config() -> Result<Config, Box<dyn std::error::Error>>
{
        let contents = read_to_string("config.toml")?;
        let config: Config = toml::from_str(&contents)?;
    
        Ok(config)
}

pub fn parse_align(config: &Config) -> Result<(Vec<u32>, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<f64>, Vec<Event>), Box<dyn std::error::Error>> {
    let mut line = String::new();

    let file = format!("{}/x_y_corrected_{}.txt", config.residuals.outpath, config.residuals.run_num);

    println!["Calculating angles from '{}'", file];

    let mut reader = BufReader::new(File::open(file).unwrap());

    let mut event_nums: Vec<u32> = Vec::new();

    let mut x: Vec<Vec<f64>> = Vec::new();
    let mut y: Vec<Vec<f64>> = Vec::new();
    let mut z: Vec<f64> = vec![0.0, 0.0, 0.0];

    let mut adc: Vec<Event> = Vec::new();

    loop {
        line.clear();
        reader.read_line(&mut line)?;

        let mut x_events: Vec<f64> = Vec::with_capacity(3);
        let mut y_events: Vec<f64> = Vec::with_capacity(3);

        let data: Vec<&str> = line.split_whitespace().collect();

        let event_num: u32 = match data[0].parse() {Ok(v) => v, Err(_) => break};
        let x_1: f64 = match data[1].parse::<f64>() {Ok(v) => v, Err(_) => break};
        let y_1: f64 = match data[2].parse::<f64>() {Ok(v) => v, Err(_) => break};
        let z_1: f64 = match data[2].parse::<f64>() {Ok(v) => v, Err(_) => break};
        let x_2: f64 = match data[1].parse::<f64>() {Ok(v) => v, Err(_) => break};
        let y_2: f64 = match data[2].parse::<f64>() {Ok(v) => v, Err(_) => break};
        let z_2: f64 = match data[2].parse::<f64>() {Ok(v) => v, Err(_) => break};
        let x_3: f64 = match data[1].parse::<f64>() {Ok(v) => v, Err(_) => break};
        let y_3: f64 = match data[2].parse::<f64>() {Ok(v) => v, Err(_) => break};
        let z_3: f64 = match data[2].parse::<f64>() {Ok(v) => v, Err(_) => break};
        let hadc: u16      = match data[5].parse() {Ok(v) => v, Err(_) => break};
        let ladc: u16      = match data[6].parse() {Ok(v) => v, Err(_) => break};
        let run_num: u16   = match data[7].parse() {Ok(v) => v, Err(_) => break};
        let hv: u16        = match data[8].parse() {Ok(v) => v, Err(_) => break};

        x_events.push(x_1);
        x_events.push(x_2);
        x_events.push(x_3);
        x.push(x_events);

        y_events.push(y_1);
        y_events.push(y_2);
        y_events.push(y_3);
        y.push(y_events);

        z = vec![z_1,z_2,z_3];

        event_nums.push(event_num);
        adc.push(Event{ hadc, ladc, hv, run_num })
    }

    Ok((event_nums, x, y, z, adc))
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
        format!("{}/output_file_run_{}_x1y1.txt",config.residuals.inpath, config.residuals.infile),
        format!("{}/output_file_run_{}_x2y2.txt",config.residuals.inpath, config.residuals.infile),
        format!("{}/output_file_run_{}_x3y3.txt",config.residuals.inpath, config.residuals.infile)
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
