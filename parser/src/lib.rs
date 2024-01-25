
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

#[allow(dead_code)]
#[derive(Debug)]
struct VpnData {
    host_name: String,
    ip: String,
    score: String,
    ping: String,
    speed: String,
    country_long: String,
    country_short: String,
    num_sessions: String,
    uptime: String,
    total_users: String,
    total_traffic: String,
    log_type: String,
    operator: String,
    message: String,
    // config_base64: String,
}

impl VpnData {
    fn new(fields: &[&str]) -> Self {
        Self {
            host_name: fields.get(0).map_or_default().to_string(),
            ip: fields.get(1).map_or_default().to_string(),
            score: fields.get(2).map_or_default().to_string(),
            ping: fields.get(3).map_or_default().to_string(),
            speed: fields.get(4).map_or_default().to_string(),
            country_long: fields.get(5).map_or_default().to_string(),
            country_short: fields.get(6).map_or_default().to_string(),
            num_sessions: fields.get(7).map_or_default().to_string(),
            uptime: fields.get(8).map_or_default().to_string(),
            total_users: fields.get(9).map_or_default().to_string(),
            total_traffic: fields.get(10).map_or_default().to_string(),
            log_type: fields.get(11).map_or_default().to_string(),
            operator: fields.get(12).map_or_default().to_string(),
            message: fields.get(13).map_or_default().to_string(),
            // config_base64: fields.get(14).map_or_default().to_string(),
        }
    }
}

trait OptionOrDefault<T> {
    fn map_or_default(self) -> T;
}

impl<T> OptionOrDefault<T> for Option<&T>
where
    T: Default + Clone,
{
    fn map_or_default(self) -> T {
        self.map_or_else(|| T::default(), |s| s.clone())
    }
}

pub fn data_iter(reader: &mut BufReader<File>, output_writer: &mut Box<dyn Write>) {
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => continue,
        };

        let fields: Vec<&str> = line.split(',').collect();

        let vpn_info = VpnData::new(&fields);

        writeln!(output_writer, "{:?}", vpn_info).unwrap();
    }
}
