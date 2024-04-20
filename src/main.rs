use rust_rpm::{config, Package, db::{find, find_package}, Index};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pub name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    //println!("Hello, world!");
    let args = Args::parse();
    config::read_file(None).unwrap(); 
    let key = args.name;
    let found_packages: Vec<Package> = find_package(key).collect();
    for package in found_packages {
        println!("{}", package);
    }
}
