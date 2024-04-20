#[macro_use]
pub mod error;

/// RPM configuration (i.e. rpmrc)
pub mod config;

/// RPM database access
pub mod db;

/// Internal functionality not to be exposed outside of this crate
mod internal;

/// Macros are RPM's configuration system
pub mod macro_context;

/// RPM packages
pub mod package;

pub use self::{db::Index, error::Error, macro_context::MacroContext, package::Package};

pub mod cli;