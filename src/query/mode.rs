use crate::{cli::{Cli, Commands}, db::{installed_packages, find_package}};






pub struct QueryMode {
    pub mode: char,
}

impl QueryMode {
    pub fn query(cli: Cli) {
        let mut pkgs = Vec::new();
        let mut keys: Vec<String> = Vec::new();
        match cli.query {
            'a' =>  {
                pkgs = installed_packages().collect()
            },
            _ => {
            match &cli.command {
                Some(cmd) => {
                    match cmd {
                        Commands::Name(name) => {
                                keys.extend(name.name.clone());
                                for key in keys.iter() {
                                    pkgs = find_package(key.clone()).collect();
                                }
                            },
                        Commands::File(file) => (),
                        }
                    }
                    None => (),
                }
            }
        }

        if pkgs.len() == 0 {
            println!("package {:?} is not install", keys);
        } 
        for package in pkgs {
            println!("{}", package);
        }
    }
}