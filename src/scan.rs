use mcping::get_status;
use std::time::Duration;
use std:: net::Ipv4Addr;
use clap::{Parser, arg, command};

#[derive(Parser, Debug)]
#[command(name = "CopeHarder", version = "1.0", long_about = None)]
#[command(about = "Scans the interwebs for minecraft servers")]
#[allow(non_snake_case)]
pub struct Scan {
    #[arg(short, long, default_value_t = Ipv4Addr::new(82,176,3,214))]
    pub startIp: Ipv4Addr,
    #[arg(short, long)]
    pub addPorts: Option<Vec<u16>>
}

impl Scan {
    pub fn scan(&self) {
        let mut current_ip: Ipv4Addr = self.startIp;
        let ip = current_ip.octets().iter().nth(0).unwrap() << 24 | current_ip.octets().iter().nth(0).unwrap() << 16 | current_ip.octets().iter().nth(0).unwrap() << 8 | current_ip.octets().iter().nth(0).unwrap();
        
        let (latency, response) = get_status(&current_ip.to_string(), Duration::from_millis(5000)).unwrap();
        println!("{} {:?}", latency, response.version.name);
    }
}
