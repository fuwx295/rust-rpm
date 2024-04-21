
use rust_rpm::{config, Package, db::{find, find_package}, Index, cli::{Cli, Commands}};

use clap::Parser;
fn main() {
    //println!("Hello, world!");
    let cli = Cli::parse();
    config::read_file(None).unwrap(); 

    let mut keys = Vec::new();
    match &cli.command {
        Commands::Name(name) => {
            keys.extend(name.name.clone())
        },
        Commands::File(file) => (),
    }

    for key in keys {
        let found_packages: Vec<Package> = find_package(key.clone()).collect();
        if found_packages.len() == 0 {
            println!("package {} is not install", key);
        } 
        for package in found_packages {
            println!("{:#?}", package);
        }
    }
    
}
