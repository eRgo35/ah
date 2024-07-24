use clap::{Command, Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Option<SubCommands>,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
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
struct PackageArg {
    package: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(SubCommands::Install(arg)) => println!("Installing package: {}", arg.package),
        Some(SubCommands::Upgrade {}) => todo!(),
        Some(SubCommands::Sync {}) => todo!(),
        Some(SubCommands::Remove(arg)) => println!("Removing package: {}", arg.package),
        Some(SubCommands::Find(arg)) => println!("Looking for package: {}", arg.package),
        None => println!("No subcommand was used"),
    }
}
