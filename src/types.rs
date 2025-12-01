
pub static Z: &[f64]  = &[0.0,180.0,700.0];

#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub hadc: u16,
    pub ladc: u16,
    pub hv: u16,
    pub run_num: u16,
}



