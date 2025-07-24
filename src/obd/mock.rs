use super::ObdReader;
use crate::data::GaugeData;

pub struct MockObd;

impl MockObd {
    pub fn new() -> Self {
        Self
    }
}

impl ObdReader for MockObd {
    fn read_data(&mut self) -> GaugeData {
        GaugeData {
            rpm: 3250,
            coolant_temp: 78.5,
            voltage: 13.8,
            speed: 65,
        }
    }
}
