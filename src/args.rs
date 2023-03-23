use clap::{self, Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    Create {
        /// The name of the server
        #[clap()]
        name: String,

        #[clap(short, long, default_value = "latest")]
        version: String,
    },

    Launch {
        #[clap()]
        name: String,
    },

    List,

    Remove {
        #[clap()]
        name: String,
    },

    Versions,
}
