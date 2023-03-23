mod actions;
mod args;
mod utils;

use clap::Parser;

use actions::{create, launch, list, remove, versions};
use args::{Action, Args};

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Create { name, version } => create(&name, &version),
        Action::Launch { name } => launch(&name),
        Action::List => list(),
        Action::Remove { name } => remove(&name),
        Action::Versions => versions(),
    };
}
