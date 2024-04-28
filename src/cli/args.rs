use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// queryformat base/info/changelog/require
    #[arg(short, long)]
    pub query: char,

    /// query all packages
    #[arg(short, long)]
    pub all: bool,

    /// query package by name or local
    #[command(subcommand)]
    pub command: Option<Commands>,
}
#[derive(Subcommand)]
pub enum Commands {
    /// package name
    Name(PackageName),
    /// package filepath
    File(FileName),
}

#[derive(Args, Clone)]
pub struct PackageName {
    pub name: Vec<String>,
}

#[derive(Args, Clone)]
pub struct FileName {
    pub files: Vec<String>,
}
