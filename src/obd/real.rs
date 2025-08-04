use std::io::{BufReader, Result, Write};
use std::time::Duration;

use serialport::SerialPort;

use super::ObdReader;
use crate::data::GaugeData;

pub struct RealObd {
    port: Box<dyn SerialPort>,
    reader: BufReader<Box<dyn SerialPort>>,
}

impl RealObd {
    pub fn new() -> Result<Self> {
        let port = serialport::new("/dev/rfcomm0", 38400)
            .timeout(Duration::from_secs(2))
            .open()?;

        let reader = BufReader::new(port.try_clone()?);

        eprintln!("[RealObd::new] Opened serial port /dev/rfcomm0 at 38400 baud");

        Ok(Self { port, reader })
    }

    fn send_command(&mut self, cmd: &str) -> Result<String> {
        let full_cmd = format!("{}\r", cmd);
        eprintln!("[send_command] Sending: {:?}", full_cmd.trim_end());

        self.port.write_all(full_cmd.as_bytes())?;
        self.port.flush()?;

        let mut response = String::new();
        loop {
            let mut buf = [0u8; 128];
            match self.port.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let chunk = String::from_utf8_lossy(&buf[..n]);
                    eprintln!("[send_command] Received chunk: {:?}", chunk);
                    response.push_str(&chunk);
                    if response.contains('>') {
                        eprintln!("[send_command] Prompt '>' detected, end of response");
                        break;
                    }
                }
                Ok(_) => {
                    eprintln!("[send_command] No more data received");
                    break;
                }
                Err(e) => {
                    eprintln!("[send_command] Read error: {}", e);
                    break;
                }
            }
        }
        let trimmed = response.trim().to_string();
        eprintln!("[send_command] Full response: {:?}", trimmed);
        Ok(trimmed)
    }
}

impl ObdReader for RealObd {
    fn read_data(&mut self) -> GaugeData {
        fn parse_bytes(response: &str) -> Vec<u8> {
            eprintln!("[parse_bytes] Parsing response:\n{}", response);
            let bytes = response
                .lines()
                .find(|line| line.trim_start().starts_with("41"))
                .map(|line| {
                    line.split_whitespace()
                        .filter_map(|b| u8::from_str_radix(b, 16).ok())
                        .collect()
                })
                .unwrap_or_else(|| {
                    eprintln!("[parse_bytes] No valid '41' line found, returning empty vec");
                    Vec::new()
                });
            eprintln!("[parse_bytes] Parsed bytes: {:?}", bytes);
            bytes
        }

        let rpm = self
            .send_command("010C")
            .ok()
            .and_then(|resp| {
                let bytes = parse_bytes(&resp);
                if bytes.len() >= 4 && bytes[0] == 0x41 && bytes[1] == 0x0C {
                    let a = bytes[2] as u16;
                    let b = bytes[3] as u16;
                    let rpm_val = ((a * 256 + b) as f32) / 4.0;
                    eprintln!("[read_data] RPM: {}", rpm_val);
                    Some(rpm_val)
                } else {
                    eprintln!("[read_data] RPM response invalid or incomplete");
                    None
                }
            })
            .unwrap_or_else(|| {
                eprintln!("[read_data] RPM command failed, defaulting to 0");
                0.0
            });

        let coolant_temp = self
            .send_command("0105")
            .ok()
            .and_then(|resp| {
                let bytes = parse_bytes(&resp);
                if bytes.len() >= 3 && bytes[0] == 0x41 && bytes[1] == 0x05 {
                    let temp = bytes[2] as f32 - 40.0;
                    eprintln!("[read_data] Coolant Temp: {}", temp);
                    Some(temp)
                } else {
                    eprintln!("[read_data] Coolant temp response invalid or incomplete");
                    None
                }
            })
            .unwrap_or_else(|| {
                eprintln!("[read_data] Coolant temp command failed, defaulting to 0");
                0.0
            });

        let voltage = self
            .send_command("0142")
            .ok()
            .and_then(|resp| {
                let bytes = parse_bytes(&resp);
                if bytes.len() >= 3 && bytes[0] == 0x41 && bytes[1] == 0x42 {
                    let volt = bytes[2] as f32 * 0.1;
                    eprintln!("[read_data] Voltage: {}", volt);
                    Some(volt)
                } else {
                    eprintln!("[read_data] Voltage response invalid or incomplete");
                    None
                }
            })
            .unwrap_or_else(|| {
                eprintln!("[read_data] Voltage command failed, defaulting to 0");
                0.0
            });

        let engine_load = self
            .send_command("0104")
            .ok()
            .and_then(|resp| {
                let bytes = parse_bytes(&resp);
                if bytes.len() >= 3 && bytes[0] == 0x41 && bytes[1] == 0x04 {
                    eprintln!("[read_data] Engine Load: {}", bytes[2]);
                    Some(bytes[2])
                } else {
                    eprintln!("[read_data] Engine load response invalid or incomplete");
                    None
                }
            })
            .unwrap_or_else(|| {
                eprintln!("[read_data] Engine load command failed, defaulting to 0");
                0
            });

        let oil_temp_est = estimate_oil_temp(coolant_temp, rpm, engine_load);
        eprintln!("[read_data] Estimated Oil Temp: {}", oil_temp_est);

        GaugeData {
            oil_temp_est,
            coolant_temp,
            voltage,
            engine_load,
        }
    }
}

fn estimate_oil_temp(coolant_temp: f32, rpm: f32, engine_load: u8) -> f32 {
    let mut offset = 10.0;

    if rpm > 6000.0 {
        offset += 10.0;
    } else if rpm > 4000.0 {
        offset += 5.0;
    }

    let load_factor = (engine_load as f32) / 255.0;
    offset += load_factor * 5.0;

    offset = offset.clamp(5.0, 25.0);

    coolant_temp + offset
}
