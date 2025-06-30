use clap::Parser;
use tire::args::{Args, Cmd};

fn main() {
    let args = Args::parse();
    match args.cmd {
        Cmd::Check { files } => {
            tire::check::check(files);
        }
        Cmd::Add { args: pkgs, auto } => {
            tire::add::add(pkgs, auto);
        }
        Cmd::Run { args } => {
            tire::run::run(args);
        }
    }
}
