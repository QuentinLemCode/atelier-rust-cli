use std::env;

mod greetings; //declare module
mod chifoumi;
use greetings::greets; // import function
use chifoumi::{play, Game};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).expect("no argument");
    if cmd == "greets" {
        greets(args.get(2).expect("bad args"));
    }
    if cmd == "chifoumi" {
        let a = Game::try_from(args.get(2).expect("bad args")).expect("toto");
        let b = Game::try_from(args.get(3).expect("bad args")).expect("toto");
        println!("{:#?}", play(a, b));
    }
}
