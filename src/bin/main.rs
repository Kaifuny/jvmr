extern crate jvmr;

use std::env;
use jvmr::cmd::command;

fn main() {
    
    println!("JVMR - JVM implementation in Rust");

    command::parse_command(env::args());
}
