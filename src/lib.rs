pub mod scan;

use std::net::Ipv4Addr;
use mcping::Response;

trait IPHex {
    fn as_hex_number(&self) -> u32;
}

impl IPHex for Ipv4Addr {
    fn as_hex_number(&self) -> u32 {
        self.octets().iter().enumerate().fold(0u32, |bef, cur| (bef | (*cur.1 as u32) << (24 - (8 * cur.0))))
    }
}
