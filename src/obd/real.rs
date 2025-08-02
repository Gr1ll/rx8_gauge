use std::io::{BufRead, BufReader, Result, Write};
use std::net::TcpStream;
use std::time::Duration;

use super::ObdReader;
use crate::data::GaugeData;

pub struct RealObd {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
}

impl RealObd {
    pub fn new(ip: &str) -> Result<Self> {
        let stream = TcpStream::connect(format!("{}:35000", ip))?;
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        stream.set_write_timeout(Some(Duration::from_secs(2)))?;

        let reader = BufReader::new(stream.try_clone()?);

        Ok(Self { stream, reader })
    }

    fn send_command(&mut self, cmd: &str) -> Result<String> {
        let full_cmd = format!("{}\r", cmd);
        self.stream.write_all(full_cmd.as_bytes())?;
        self.stream.flush()?;

        let mut response = String::new();
        self.reader.read_line(&mut response)?;

        Ok(response.trim().to_string())
    }
}

impl ObdReader for RealObd {
    fn read_data(&mut self) -> GaugeData {
        fn parse_bytes(response: &str) -> Vec<u8> {
            response
                .split_whitespace()
                .filter_map(|b| u8::from_str_radix(b, 16).ok())
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
                if bytes.len() >= 3 && bytes[0] == 0x41 && bytes[1] == 0x42 {
                    Some(bytes[2] as f32 * 0.1)
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
