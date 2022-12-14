use mcping::{get_status, Response};
use std::time::Duration;
use std:: net::Ipv4Addr;
use rayon::{prelude::*, ThreadPoolBuilder};
use clap::{Parser, arg, command};

use crate::IPHex;

#[derive(Parser, Debug)]
#[command(name = "CopeHarder", version = "1.0", long_about = None)]
#[command(about = "Scans the interwebs for minecraft servers")]
#[allow(non_snake_case)]
pub struct Scan {
    #[arg(short, long, default_value_t = Ipv4Addr::new(82,174,150,98))]
    pub startIp: Ipv4Addr,
    #[arg(short, long, default_value_t = Ipv4Addr::new(255,255,255,255))]
    pub endIp: Ipv4Addr,
    #[arg(short, long, default_value_t = 50)]
    pub nWorker: u32,
    #[arg(short, long)]
    pub addPorts: Option<Vec<u16>>
}

impl Scan {
    pub fn scan(&self) {
        let start_hex = self.startIp.as_hex_number();
        let end_hex = self.endIp.as_hex_number();

        let jobs = 0..(end_hex - start_hex);
        //let pool = ThreadPoolBuilder::new().num_threads(self.nWorker.try_into().unwrap()).build().unwrap();
        rayon::ThreadPoolBuilder::new().num_threads(self.nWorker.try_into().unwrap()).build_global().unwrap();

        jobs.into_par_iter().for_each(|i| self.do_check(i));

        //jobs.for_each(|i| pool.install(|| self.do_check(i)));
    }
    
    fn do_check(&self, job: u32) {
        let hex = self.startIp.as_hex_number() + job;
        let new_ip: Ipv4Addr = Ipv4Addr::from(hex); 

        //let (latency, response) = get_status(&new_ip.to_string(), Duration::from_millis(5000));

        match get_status(&new_ip.to_string(), Duration::from_millis(5000)) {
            Ok((latency, response)) => {
                println!("{}|{}: {} {:?} {:?}", job, new_ip.to_string(), latency, response.version.name, response.players.online);
            },
            Err(_) => {
                //println!("{}|{}: No worky", job, new_ip.to_string());
            },
        }
    }
}
