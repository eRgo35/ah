use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<SubCommands>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    #[command(alias = "i")]
    Install(PackageArg),
    #[command(alias = "u")]
    Upgrade {},
    #[command(alias = "s")]
    Sync {},
    #[command(alias = "r")]
    Remove(PackageArg),
    #[command(alias = "f")]
    Find(PackageArg),
}

#[derive(Args, Debug)]
pub struct PackageArg {
    pub package: String,
}