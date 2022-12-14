use CopeHarder::scan::Scan;
use clap::{Parser, arg, command};

//use mcping::get_status;

fn main() {
    let mut cmd = Scan::parse();
    cmd.scan();
}
