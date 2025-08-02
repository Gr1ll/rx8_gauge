#[cfg(not(all(feature = "pi", not(debug_assertions))))]
pub mod mock;
#[cfg(all(feature = "pi", not(debug_assertions)))]
pub mod real;

use crate::data::GaugeData;

pub trait ObdReader {
    fn read_data(&mut self) -> GaugeData;
}

#[cfg(all(feature = "pi", not(debug_assertions)))]
pub fn init_obd() -> impl ObdReader {
    real::RealObd::new("ip").expect("Failed to connect to OBD-II WiFi adapter")
}

#[cfg(not(all(feature = "pi", not(debug_assertions))))]
pub fn init_obd() -> impl ObdReader {
    mock::MockObd::new()
}
