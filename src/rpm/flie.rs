use rpm::{PackageMetadata, DependencyFlags};

use crate::rpm::package::buildtime;

fn symbol(d: DependencyFlags) -> Option<String> {
        match d {
            _ if d.contains(DependencyFlags::LE) => Some(format!("<=")),
            _ if d.contains(DependencyFlags::GE) => Some(format!(">=")),
            _ if d.contains(DependencyFlags::LESS) => Some(format!("<")),
            _ if d.contains(DependencyFlags::GREATER) => Some(format!(">")),
            _ if d.contains(DependencyFlags::EQUAL) => Some(format!("=")),
            _ => None,
        }
} 


pub fn show(meta: PackageMetadata, query: char) {
    match query {
        'b' => {
            match meta.get_arch() {
                        Ok(arch) => {
                            println!("{}-{}-{}.{}", meta.get_name().unwrap(), meta.get_version().unwrap(), meta.get_release().unwrap(), arch);
                        }
                        _ => {
                         println!("{}-{}-{}", meta.get_name().unwrap(), meta.get_version().unwrap(), meta.get_release().unwrap());  
                        }
                    }
                },
        'i' => {
            println!("Name: {}", meta.get_name().unwrap());
            println!("Epoch: {}", meta.get_epoch().unwrap_or(0));
            println!("Version: {}", meta.get_version().unwrap());
            println!("Release: {}", meta.get_release().unwrap());
            println!("Architecture: {}", meta.get_arch().unwrap_or("None"));
            println!("Install Date: (not installed)");
            println!("Group: {}", meta.get_group().unwrap());
            println!("Size: {}", meta.get_installed_size().unwrap());
            println!("License: {}", meta.get_license().unwrap());
            println!("Source RPM: {}", meta.get_source_rpm().unwrap());
            println!("Build Date: {}", buildtime(meta.get_build_time().unwrap_or(0) as i32));
            println!("Build Host: {}", meta.get_build_host().unwrap());
            println!("Vendor: {}", meta.get_vendor().unwrap_or("None"));
            println!("URL: {}", meta.get_url().unwrap_or("None"));
            println!("Summary: {}", meta.get_summary().unwrap());
            println!("Description: \n{}", meta.get_description().unwrap());
                
            },
        'r' => {
                    for r in meta.get_requires().unwrap() {
                        match symbol(r.flags) {
                            Some(f) => {println!("{} {} {}", r.name, f, r.version);},
                            _ => {println!("{}", r.name);}
                        }
                    }
                },
                'p' => {
                    for p in meta.get_provides().unwrap() {
                        match symbol(p.flags) {
                            Some(f) => println!("{} {} {}", p.name, f, p.version),
                            _ => println!("{}", p.name)
                        }
                    }
                },
                'c' => {
                    for cl in meta.get_changelog_entries().unwrap() {
                        println!("* {} {}\n{}\n", buildtime(cl.timestamp as i32), cl.name, cl.description);
                    }
                },
                _ => {}
            }
}