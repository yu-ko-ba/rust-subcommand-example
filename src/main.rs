use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    #[command(about = "about of hoge subcommand")]
    Hoge,
    #[command(about = "about of fuga subcommand")]
    Fuga,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let c = Cli::parse();
    match c.command {
        Commands::Hoge => {
            println!("hogee");
        }
        Commands::Fuga => {
            println!("fugaa");
        }
    }
}
