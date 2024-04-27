use crate::{cli::{Cli, Commands}, rpm::db::{installed_packages, find_package}};

pub struct QueryMode {
}

impl QueryMode {
    pub fn query(cli: Cli) {
        let mut pkgs = Vec::new();
        let mut keys: Vec<String> = Vec::new();
        match cli.query {
            'a' =>  {
                pkgs = installed_packages(cli.query).collect()
            },
            _ => {
            match &cli.command {
                Some(cmd) => {
                    match cmd {
                        Commands::Name(name) => {
                                keys.extend(name.name.clone());
                                for key in keys.iter() {
                                    pkgs = find_package(key.clone(), cli.query).collect();
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
            return;
        } 
        match &cli.query {
            'i' => {
                for package in pkgs {
                    println!("{:#?}", package);
                }
            }
            _ => {
                for package in pkgs {
                    println!("{}", package);
                }
            }
        }
        
    }
}