use clap::Parser;

mod cli;
mod packages;

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        Some(cli::SubCommands::Install(arg)) => packages::install(&arg.package),
        Some(cli::SubCommands::Upgrade {}) => packages::upgrade(),
        Some(cli::SubCommands::Sync {}) => packages::sync(),
        Some(cli::SubCommands::Remove(arg)) => packages::remove(&arg.package),
        Some(cli::SubCommands::Find(arg)) => packages::find(&arg.package),
        None => packages::sync(),
    }
}
