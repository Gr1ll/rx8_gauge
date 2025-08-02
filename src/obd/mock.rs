use crate::{data::GaugeData, obd::ObdReader};

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

        let rpm = 1000.0 + ((self.tick * 300) % 6000) as f32;

        let coolant_temp = 60.0 + ((self.tick % 300) as f32) * 0.1;
        let voltage = 13.5 + (((self.tick / 10) % 10) as f32 * 0.02);
        let engine_load = ((self.tick / 1) % 160) as u8;

        let oil_temp_est = estimate_oil_temp(coolant_temp, rpm, engine_load);

        GaugeData {
            oil_temp_est,
            coolant_temp,
            voltage,
            engine_load,
        }
    }
}

fn estimate_oil_temp(coolant_temp: f32, rpm: f32, engine_load: u8) -> f32 {
    let mut offset = 15.0;

    if rpm > 6000.0 {
        offset += 10.0;
    } else if rpm > 4000.0 {
        offset += 5.0;
    }

    let load_factor = (engine_load as f32) / 255.0;
    offset += load_factor * 5.0;

    offset = offset.clamp(10.0, 35.0);

    coolant_temp + offset
}
