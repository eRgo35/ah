use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "ah",
    author = "Michał Czyż",
    version = "0.1.0",
    about = "A declarative package manager for Arch Linux",
    long_about = "Arch Helper is a declarative package management tool for Arch Linux. It leverages paru or other package managers for seamless integration."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(alias = "i", about = "Install packages")]
    Install(PackageList),

    #[command(alias = "u", about = "Upgrade packages")]
    Upgrade {
        #[arg(help = "Don't prompt for confirmation", default_value_t = false)]
        noconfirm: bool,
    },

    #[command(alias = "s", about = "Synchronize packages")]
    Sync {
        #[arg(help = "Don't prompt for confirmation", default_value_t = false)]
        noconfirm: bool,
    },

    #[command(alias = "r", about = "Remove packages")]
    Remove(PackageList),

    #[command(alias = "f", about = "Find packages")]
    Find(Query),
}

#[derive(Args)]
pub struct PackageList {
    #[arg(help = "Name(s) of the package(s), separated by spaces")]
    pub packages: Vec<String>,
}

#[derive(Args)]
pub struct Query {
    #[arg(help = "Search term for finding packages")]
    pub query: Vec<String>,
}
