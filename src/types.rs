
pub static Z: &[f64]  = &[0.0,180.0,700.0];

#[derive(Debug, Clone)]
pub struct Event {
    pub hadc: u16,
    pub ladc: u16,
    pub hv: u16,
    pub run_num: u16,
}

#[derive(Debug)]
pub struct Error {
    pub x: Vec<f64>,
    pub y: Vec<f64>
}


