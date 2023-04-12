mod chifoumi;
mod greetings; //declare module
use chifoumi::{play, Game};
use greetings::greets; // import function

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    author = "Quentin Lem Code",
    version = "1.0.0",
    about = "THE CLI",
    long_about = None
)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Chifoumi {
        #[clap(short, long, value_parser)]
        a: String,
        #[clap(short, long, value_parser)]
        b: String,
    },
    Greetings {
        #[clap(short, long, value_parser)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Chifoumi { a, b } => {
            let a = Game::try_from(a).expect("toto");
            let b = Game::try_from(b).expect("toto");
            println!("{:#?}", play(a, b));
        }
        Commands::Greetings { name } => {
            greets(name);
        }
    }
}
