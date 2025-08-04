use serialport::SerialPort;
use std::io::{Result, Write};
use std::time::Duration;

use super::ObdReader;
use crate::data::GaugeData;

pub struct RealObd {
    port: Box<dyn SerialPort>,
}

impl RealObd {
    pub fn new() -> Result<Self> {
        let port = serialport::new("/dev/rfcomm0", 38400)
            .timeout(Duration::from_secs(2))
            .open()?;

        Ok(Self { port })
    }

    fn send_command(&mut self, cmd: &str) -> Result<String> {
        let full_cmd = format!("{}\r", cmd);

        self.port.clear(serialport::ClearBuffer::All)?;
        self.port.write_all(full_cmd.as_bytes())?;
        self.port.flush()?;

        let mut response = String::new();
        let mut buf = [0u8; 128];

        loop {
            match self.port.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let chunk = String::from_utf8_lossy(&buf[..n]);
                    response.push_str(&chunk);

                    if chunk.contains('>') {
                        break;
                    }
                }
                Ok(_) => continue,
                Err(_) => break,
            }
        }

        Ok(response.trim().to_string())
    }
}

impl ObdReader for RealObd {
    fn read_data(&mut self) -> GaugeData {
        fn parse_bytes(response: &str) -> Vec<u8> {
            response
                .split(|c| c == '\r' || c == '\n')
                .filter(|line| line.trim_start().starts_with("41"))
                .flat_map(|line| {
                    line.split_whitespace()
                        .filter_map(|b| u8::from_str_radix(b, 16).ok())
                        .collect::<Vec<u8>>()
                })
                .collect()
        }

        let rpm = self
            .send_command("010C")
            .ok()
            .and_then(|resp| {
                let bytes = parse_bytes(&resp);
                if bytes.len() >= 4 && bytes[0] == 0x41 && bytes[1] == 0x0C {
                    let a = bytes[2] as u16;
                    let b = bytes[3] as u16;
                    Some(((a * 256 + b) as f32) / 4.0)
                } else {
                    None
                }
            })
            .unwrap_or(0.0);

        let coolant_temp = self
            .send_command("0105")
            .ok()
            .and_then(|resp| {
                let bytes = parse_bytes(&resp);
                if bytes.len() >= 3 && bytes[0] == 0x41 && bytes[1] == 0x05 {
                    Some(bytes[2] as f32 - 40.0)
                } else {
                    None
                }
            })
            .unwrap_or(0.0);

        let voltage = self
            .send_command("0142")
            .ok()
            .and_then(|resp| {
                let bytes = parse_bytes(&resp);
                if bytes.len() >= 4 && bytes[0] == 0x41 && bytes[1] == 0x42 {
                    let a = bytes[2] as u16;
                    let b = bytes[3] as u16;
                    Some(((a * 256 + b) as f32) / 1000.0)
                } else {
                    None
                }
            })
            .unwrap_or(0.0);

        let engine_load = self
            .send_command("0104")
            .ok()
            .and_then(|resp| {
                let bytes = parse_bytes(&resp);
                if bytes.len() >= 3 && bytes[0] == 0x41 && bytes[1] == 0x04 {
                    Some(bytes[2])
                } else {
                    None
                }
            })
            .unwrap_or(0);

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
