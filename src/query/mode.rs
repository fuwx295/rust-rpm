use crate::{
    cli::{Cli, Commands},
    rpm::db::{find_package, installed_packages},
};

pub struct QueryMode {}

impl QueryMode {
    pub fn query(cli: Cli) {
        let mut pkgs = Vec::new();
        let mut keys: Vec<String> = Vec::new();
        match cli.all {
            true => pkgs = installed_packages(cli.query).collect(),
            false => match &cli.command {
                Some(cmd) => match cmd {
                    Commands::Name(name) => {
                        keys.extend(name.name.clone());
                        for key in keys.iter() {
                            pkgs = find_package(key.clone(), cli.query).collect();
                        }
                    }
                    Commands::File(file) => (),
                },
                None => (),
            },
        }

        if pkgs.is_empty() {
            println!("package {:?} is not install", keys);
            return;
        }
        match &cli.query {
            'i' => {
                for package in pkgs {
                    println!("{:#?}", package);
                }
            }
            'b' => {
                for package in pkgs {
                    println!("{}", package);
                }
            }
            'c' => {
                for package in pkgs {
                    println!("{:#?}", package.changelog);
                }
            }
            'r' => {
                for package in pkgs {
                    if let Some(requires) = package.require {
                        requires.show();
                    }
                }
            }
            'p' => {
                for package in pkgs {
                    println!("{:#?}", package.provide);
                }
            }
            _ => {}
        }
    }
}
