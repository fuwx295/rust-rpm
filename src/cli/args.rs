use clap::{Parser, Args, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}
#[derive(Subcommand)]
pub enum Commands {
    Name(PackageName),
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