#[derive(Debug, Clone)]
pub struct GaugeData {
    pub horse_power: u16,
    pub coolant_temp: f32,
    pub voltage: f32,
    pub engine_load: u8,
}
