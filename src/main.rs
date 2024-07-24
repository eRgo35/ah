use clap::Parser;
use cli::{PackageList, Query};
use colored::Colorize;

mod cli;
mod packages;
mod file;

fn main() {
    let cli = cli::Cli::parse();

    let result = match cli.command {
        Some(cli::Commands::Install(PackageList { packages })) => packages::install(packages),
        Some(cli::Commands::Upgrade { noconfirm }) => packages::upgrade(noconfirm),
        Some(cli::Commands::Sync { noconfirm }) => packages::sync(noconfirm),
        Some(cli::Commands::Remove(PackageList { packages })) => packages::remove(packages),
        Some(cli::Commands::Find(Query { query })) => packages::find(query),
        None => packages::rebuild(true),
    };

    if let Err(err) = result {
        eprintln!("{} {}", "::".bold().red(), err.to_string().bold());
        std::process::exit(1);
    }
}
