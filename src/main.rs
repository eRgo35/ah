use ah_pkg::{cli, packages};
use clap::Parser;
use colored::Colorize;

fn main() {
    let cli = cli::Cli::parse();

    let result = match cli.command {
        Some(cli::Commands::Install(cli::PackageList { packages })) => packages::install(packages),
        Some(cli::Commands::Upgrade { noconfirm }) => packages::upgrade(noconfirm),
        Some(cli::Commands::Sync { noconfirm }) => packages::sync(noconfirm),
        Some(cli::Commands::Remove(cli::PackageList { packages })) => packages::remove(packages),
        Some(cli::Commands::Find(cli::Query { query })) => packages::find(query),
        Some(cli::Commands::ChooseInstall(cli::Query { query })) => packages::choose_install(query),
        None => packages::full_upgrade(true),
    };

    if let Err(err) = result {
        eprintln!("{} {}", "::".bold().red(), err.to_string().bold());
        std::process::exit(1);
    }
}
