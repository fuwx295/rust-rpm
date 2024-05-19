use std::convert::TryFrom;
use std::{fmt, time};

use chrono::{DateTime, Utc, TimeZone};

use super::internal::tag::DependencyFlag;

pub fn buildtime(time: i32) -> String {
    let datetime: DateTime<Utc> = Utc.timestamp_opt(time as i64, 0).unwrap();
    format!("{}", datetime.format("%a %b %d %Y"))
}



#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Require {
    pub requirename: Option<Vec<String>>,
    pub requireflags: Option<Vec<DependencyFlag>>,
    pub requireversion: Option<Vec<String>>,
}

impl Require {
    pub fn show(&self) {
        if let (Some(names), Some(flags), Some(versions)) = (
            &self.requirename,
            &self.requireflags,
            &self.requireversion) {
            for i in 0..names.len() {
                match flags[i].symbol() {
                    None => println!("{}", names[i]),
                    Some(flag) => println!("{} {} {}", names[i], flag, versions[i]),
                }
            }
        } else {
            println!("No Require");
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Changelog {
    pub changelognames: Option<Vec<String>>,
    pub changelogtimes: Option<Vec<i32>>,
    pub changelogtexts: Option<Vec<String>>,
}

impl Changelog {
    pub fn show(&self) {
        if let (Some(names), Some(times), Some(texts)) = (
            &self.changelognames,
            &self.changelogtimes,
            &self.changelogtexts) {
            for i in 0..names.len() {
                println!("* {} {}\n{}\n", buildtime(times[i]), names[i], texts[i]);
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Provide {
    pub providenames: Option<Vec<String>>,
    pub provideflags: Option<Vec<DependencyFlag>>,
    pub provideverions: Option<Vec<String>>,
}

impl Provide {
    pub fn show(&self) {
        if let (Some(names), Some(flags), Some(versions)) = (
            &self.providenames,
            &self.provideflags,
            &self.provideverions) {
            for i in 0..names.len() {
                match flags[i].symbol() {
                    None => println!("{} {}", names[i], versions[i]),
                    Some(flag) => println!("{} {} {}", names[i], flag, versions[i]),
                }
            }
        } else {
            println!("No Require");
        }
    }
}


/// RPM packages
#[derive(Default, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Package {
    pub name: String,
    pub epoch: Option<i32>,
    pub version: String,
    pub release: String,
    pub arch: Option<String>,
    pub installtime: i32,
    pub group: String,
    pub size: i64,
    pub license: String,
    pub signature: Option<String>,
    pub sourcerpm: String,
    pub buildtime: i32,
    pub buildhost: String,
    pub relocations: Option<String>,
    pub packager: Option<String>,
    pub vendor: Option<String>,
    pub url: Option<String>,
    pub bugurl: Option<String>,
    pub summary: String,
    pub description: String,
    pub require: Option<Require>,
    pub changelog: Option<Changelog>,
    pub provide: Option<Provide>,
}

impl Package {
    /// Name of the package
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Epoch of the package
    pub fn epoch(&self) -> Option<i32> {
        self.epoch
    }

    /// Version of the package
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Release of the package
    pub fn release(&self) -> &str {
        &self.release
    }

    /// Arch of the package
    pub fn arch(&self) -> Option<&str> {
        self.arch.as_deref()
    }

    /// EVR (epoch, version, release) of the package
    pub fn evr(&self) -> String {
        if let Some(epoch) = &self.epoch {
            format!("{}:{}-{}", epoch, self.version, self.release)
        } else {
            format!("{}-{}", self.version, self.release)
        }
    }

    /// NEVRA (name, epoch, version, release, arch) of the package
    pub fn nevra(&self) -> String {
        if let Some(arch) = &self.arch {
            format!("{}-{}.{}", self.name, self.evr(), arch)
        } else {
            format!("{}-{}", self.name, self.evr())
        }
    }

    /// License of the package
    pub fn license(&self) -> &str {
        &self.license
    }

    /// Succinct description of the package
    pub fn summary(&self) -> &str {
        &self.summary
    }

    /// Longer description of the package
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Buildtime of the package
    pub fn buildtime(&self) -> time::SystemTime {
        let buildtime = u64::try_from(self.buildtime).expect("negative build time");
        time::SystemTime::UNIX_EPOCH + time::Duration::new(buildtime, 0)
    }
    fn show_info(&self) {
        println!("Name: {}", self.name);
        println!("Epoch: {}", self.epoch.unwrap_or(0));
        println!("Version: {}", self.version);
        println!("Release: {}", self.release);
        println!("Architecture: {}", self.arch.as_deref().unwrap_or("None"));
        println!("Install Date: {}", buildtime(self.installtime));
        println!("Group: {}", self.group);
        println!("Size: {}", self.size);
        println!("License: {}", self.license);
        println!("Signature: {}", self.signature.as_deref().unwrap_or("None"));
        println!("Source RPM: {}", self.sourcerpm);
        println!("Build Date: {}", buildtime(self.buildtime));
        println!("Build Host: {}", self.buildhost);
        println!("Relocations: {}", self.relocations.as_deref().unwrap_or("None"));
        println!("Packager: {}", self.packager.as_deref().unwrap_or("None"));
        println!("Vendor: {}", self.vendor.as_deref().unwrap_or("None"));
        println!("URL: {}", self.url.as_deref().unwrap_or("None"));
        println!("Summary: {}", self.summary);
        println!("Description: \n{}", self.description);
    }

    pub fn show(&self, c: char) {
        match c {
            'b' => println!("{}", self.nevra()),
            'i' => self.show_info(),
            'r' => self.require.as_ref().unwrap().show(),
            'c' => self.changelog.as_ref().unwrap().show(),
            'p' => self.provide.as_ref().unwrap().show(),
            _ => {}
        }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nevra())
    }
}