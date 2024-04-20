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
    Name(Name),
}


#[derive(Args, Clone)]
pub struct Name {
    pub name: Vec<String>
}