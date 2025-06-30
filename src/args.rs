///
/// This file defines the command-line interface of Tire.
///
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
#[clap(verbatim_doc_comment)]
pub enum Cmd {
    /// Call a Python script, module, function or package.
    ///
    /// This command is analogous to the `uv run` command, but provides a bit more flexibility and
    /// shorter syntax. This command is parsed such that all options before the first positional
    /// argument are passed to Uv, and all subsequent arguments are passed to the call target.
    ///
    /// Examples:
    /// {n}
    /// $ tire run path/to/file.py{n}
    /// $ tire run module:func{n}
    /// $ tire run -m module{n}
    /// $ tire run @pkg{n}
    /// $ tire run --with pkg pkg-cmd2
    ///
    /// Differences to Uv:
    /// {n}
    /// - The `module:func` version runs the function with the `cyclopts` CLI framework.{n}
    /// - The `@pkg` version runs as `--with pkg pkg`.
    ///
    /// To see which additional arguments you can pass to `tire run` before the first positional
    /// argument, check the Uv documentation with `uv run --help`.
    Run {
        // Remaining arguments are passed to UV.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}
