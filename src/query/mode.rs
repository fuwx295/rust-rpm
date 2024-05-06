use crate::{
    cli::{Cli, Commands},
    rpm::db::{find_package, installed_packages},
};

use rpm;

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
                    Commands::File(file) => {
                        for f in file.files.iter() {
                            let p = rpm::Package::open(f).unwrap();
                            match cli.query {
                                'b' => {

                                    println!("{}-{}-{}.{}", p.metadata.get_name().unwrap(), p.metadata.get_version().unwrap(), p.metadata.get_release().unwrap(), p.metadata.get_arch().unwrap());
                                },
                                'i' => {},
                                'c' => {
                                    for cl in p.metadata.get_changelog_entries().unwrap() {
                                        println!("{}\n{}\n", cl.name, cl.description);
                                    }
                                },
                                'r' => {
                                    for r in p.metadata.get_requires().unwrap() {
                                        println!("{}", r.name);
                                    }
                                },
                                'p' => {
                                    for pv in p.metadata.get_provides().unwrap() {
                                        println!("{}", pv.name);
                                    }
                                }
                                _ => (),
                            }
                        }

                    return;
                    },
                },
                None => (),
            },
        }

        if pkgs.is_empty() {
            println!("package {:?} is not install", keys);
            return;
        }
        for pkg in pkgs {
            pkg.show(cli.query);
        }
    }
}
