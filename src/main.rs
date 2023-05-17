mod action;

use action::Action;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Test { name, count } => {
            for _ in 0..count {
                println!("Hello {}!", name)
            }
        },
        Action::Hoge { fuga } => {
            println!("Hoge {}", fuga)
        }
    }
}
