use std::net::Ipv4Addr;

pub mod scan;

trait IPHex {
    fn as_hex_number(&self) -> u64;
}

impl IPHex for Ipv4Addr {
    fn as_hex_number(&self) -> u64 {
        self::scan::Scan
    }
}