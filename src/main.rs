mod add;
mod args;
mod run;

use crate::args::{Args, Cmd};
use clap::Parser;

fn main() {
    let args = Args::parse();
    match args.cmd {
        Cmd::Add { args: pkgs, auto } => {
            crate::add::add(pkgs, auto);
        }
        Cmd::Run { args } => {
            crate::run::run(args);
        }
    }
}
