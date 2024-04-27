use clap::{Parser, Args, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// query base/all/info/changelog/require
    #[arg(short, long)]
    pub query: char,

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
    pub name: Vec<String>
}

#[derive(Args, Clone)]
pub struct FileName {
    pub files: Vec<String>
}