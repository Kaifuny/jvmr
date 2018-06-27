extern crate jvmr;

use std::env;
use jvmr::cmd::command;

fn main() {
    command::parse_command(env::args());
}
