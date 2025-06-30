mod args;
mod run;

use crate::args::{Args, Cmd};
use clap::Parser;

fn main() {
    let args = Args::parse();
    match args.cmd {
        Cmd::Run { args } => {
            crate::run::run(args);
        }
    }
}
