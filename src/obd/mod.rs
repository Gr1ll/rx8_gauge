pub mod mock;

use crate::data::GaugeData;

pub trait ObdReader {
    fn read_data(&mut self) -> GaugeData;
}

pub fn init_obd() -> impl ObdReader {
    mock::MockObd::new()
}
