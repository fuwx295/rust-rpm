use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// queryformat base/info/changelog/require/provid e
    #[arg(short, long)]
    pub query: char,

    /// query all packages
    #[arg(short, long)]
    pub all: bool,

    /// package name or file
    pub name: Option<Vec<String>>,
}
