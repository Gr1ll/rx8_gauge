#[derive(Debug, Clone)]
pub struct GaugeData {
    pub rpm: u16,
    pub coolant_temp: f32,
    pub voltage: f32,
    pub speed: u8,
}
