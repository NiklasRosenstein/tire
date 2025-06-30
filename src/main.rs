use clap::Parser;
use tire::args::{Args, Cmd};

fn main() {
    let args = Args::parse();
    match args.cmd {
        Cmd::Add { args: pkgs, auto } => {
            tire::add::add(pkgs, auto);
        }
        Cmd::Check { files } => {
            tire::check::check(files);
        }
        Cmd::Fmt { files, check } => {
            tire::fmt::fmt(files, check);
        }
        Cmd::Lint {
            files,
            fix,
            unsafe_fixes,
        } => {
            tire::lint::lint(files, fix, unsafe_fixes);
        }
        Cmd::Run { args } => {
            tire::run::run(args);
        }
    }
}
