use super::ObdReader;
use crate::data::GaugeData;

pub struct MockObd {
    tick: u32,
}

impl MockObd {
    pub fn new() -> Self {
        Self { tick: 0 }
    }
}

impl ObdReader for MockObd {
    fn read_data(&mut self) -> GaugeData {
        self.tick += 1;

        let horse_power = 0 + ((self.tick * 10) % 200);
        let coolant_temp = 60.0 + ((self.tick % 300) as f32) * 0.1;
        let voltage = 13.5 + (((self.tick / 10) % 10) as f32 * 0.02);
        let engine_load = ((self.tick / 1) % 160) as u8;

        GaugeData {
            horse_power: horse_power as u16,
            coolant_temp,
            voltage,
            engine_load,
        }
    }
}
