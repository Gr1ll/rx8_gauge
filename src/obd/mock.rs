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

        let rpm = 1000 + ((self.tick * 50) % 7000);
        let coolant_temp = 60.0 + ((self.tick % 300) as f32) * 0.1;
        let voltage = 13.5 + (((self.tick / 10) % 10) as f32 * 0.02);
        let speed = ((self.tick / 5) % 200) as u8;

        GaugeData {
            rpm: rpm as u16,
            coolant_temp,
            voltage,
            speed,
        }
    }
}
