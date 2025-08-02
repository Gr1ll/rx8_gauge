#[derive(Debug, Clone)]
pub struct GaugeData {
    pub oil_temp_est: f32,
    pub coolant_temp: f32,
    pub voltage: f32,
    pub engine_load: u8,
}
