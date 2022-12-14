use CopeHarder::scan::Scan;
use clap::{Parser};

//use mcping::get_status;

fn main() {
    let cmd = Scan::parse();
    cmd.scan();
}
