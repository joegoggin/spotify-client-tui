use clap::Parser;
use core::clap::Args;

mod core;

pub fn run() {
    let args = Args::parse();

    println!("{:#?}", args);
}
