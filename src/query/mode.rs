use crate::{
    cli::Cli,
    rpm::{db::{find_package, installed_packages}, flie::show},
};

use rpm;

pub struct QueryMode {}

impl QueryMode {
    pub fn query(cli: Cli) {
        let mut pkgs = Vec::new();
        let mut keys: Vec<String> = Vec::new();
        let mut file_pkgs = Vec::new();
        match cli.all {
            true => pkgs = installed_packages(cli.query).collect(),
            false => match &cli.name {
                Some(names) => {
                    for name in names.iter() {
                        match name.ends_with(".rpm") {
                            true => {
                                let p = rpm::Package::open(name).unwrap();
                                file_pkgs.push(p);    
                            }
                            false => {
                                keys.push(name.clone());
                                for key in keys.iter() {
                                    pkgs = find_package(key, cli.query).collect();
                                }
                            }
                        }
                    }
                },
                None => (),
            },
        }

        if pkgs.is_empty() && file_pkgs.is_empty() {
            println!("package {:?} is not install", keys);
            return;
        }
        for pkg in pkgs {
            pkg.show(cli.query);
        }
        for pkg in file_pkgs {
            show(pkg.metadata, cli.query);
        }
    }
}
