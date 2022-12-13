use clap::{Parser, arg, command};
//use mcping::get_status;

fn main() {
    let mut cmd = CopeHarder::scan::Scan::parse();
    cmd.scan();
}
