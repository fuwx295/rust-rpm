

use clap::Parser;
use rust_rpm::{cli, rpm::config, query};
fn main() {
    //println!("Hello, world!");
    let cli = cli::Cli::parse();
    config::read_file(None).unwrap(); 

    // println!("{}", cli.query);
    query::QueryMode::query(cli);
    
}
