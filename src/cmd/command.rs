use std::env::Args;
use std::fs;

pub fn parse_command(mut _args: Args) {
    let file_name = _args.nth(1).expect("Please Input ClassFile Name");
    println!("{}", file_name)
}