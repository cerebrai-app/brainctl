use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "brainctl",
    about = "brainctl command line interface",
    disable_version_flag = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print the brainctl version
    Version,
}

fn version_string() -> String {
    format!("brainctl {}", env!("CARGO_PKG_VERSION"))
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Version => println!("{}", version_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_string_has_name_and_version() {
        assert_eq!(
            version_string(),
            concat!("brainctl ", env!("CARGO_PKG_VERSION"))
        );
    }
}
