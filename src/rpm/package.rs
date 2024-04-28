use std::convert::TryFrom;
use std::{fmt, time};

/// RPM packages
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
    pub requirenevrs: Option<Vec<String>>,
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
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nevra())
    }
}
