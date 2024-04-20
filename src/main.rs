use rust_rpm::{config, Package, db::{find, find_package}, Index};



fn main() {
    //println!("Hello, world!");

    config::read_file(None).unwrap(); 
    let key = "git";
    let found_packages: Vec<Package> = find_package(key).collect();
    for package in found_packages {
        println!("{:#?}", package);
    }
}
