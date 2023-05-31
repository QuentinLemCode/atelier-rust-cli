mod chifoumi;
mod greetings;
mod search;
use chifoumi::{play, Game};
use greetings::greets;
use search::search;
use cli_table::{print_stdout, Table};


use clap::{Parser, Subcommand};
use rand::{seq::SliceRandom, thread_rng};

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
        b: Option<String>,
    },
    Greetings {
        #[clap(short, long, value_parser)]
        name: String,
    },
    Search {
        #[clap(short, long, value_parser)]
        query: String,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Chifoumi { a, b } => {
            let player_a = Game::try_from(a).expect("toto");
            let player_b;
            match b {
                None => {
                    player_b = *[Game::Paper, Game::Rock, Game::Scissor].choose(&mut thread_rng()).expect("toto");
                },
                Some(b_arg) => {
                    player_b = Game::try_from(b_arg).expect("toto");
                }
            }
            println!("{:#?}", play(player_a, player_b));
        },
        Commands::Greetings { name } => {
            greets(name);
        },
        Commands::Search { query } => {
            let result = search(query);
            let table = result.expect("toto").hits.table();
            print_stdout(table).expect("toto");
        }
    }
}
