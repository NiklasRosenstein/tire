use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
#[clap(verbatim_doc_comment)]
pub enum Cmd {
    /// Add dependencies.
    ///
    /// This command is analogous to the `uv add` command, but provides an additional `--auto` flag.
    Add {
        /// Parse all `*.py` files in your project, looking for imports that can be mapped to
        /// known Python packages. Note that this option must be specified first if any other
        /// options are being passed to `uv add` with `[PKGS]...`.
        ///
        /// Not yet implemented.
        #[arg(short, long)]
        auto: bool,

        /// One or more requirement specs that represent packages to add to the project, as well as
        /// any additional flags to pass along to `uv add`.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

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
        /// Arguments to pass to Uv. Requires at least one positional argument. The expected format
        /// is roughly: [UV_ARGS]... <TARGET> [TARGET_ARGS]...
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Type-check your code.
    Check {
        /// Files or directories to type-check recursively. Defaults to the current working
        /// directory.
        #[arg(name = "file")]
        files: Vec<String>,
    },

    /// Format your code.
    Fmt {
        /// Files or directories to format recursively. Defaults to the current working directory.
        #[arg(name = "file")]
        files: Vec<String>,

        /// Only check whether formatting would modify any files.
        #[arg(long)]
        check: bool,
    },

    /// Lint your code.
    Lint {
        /// Files or directories to lint recursively. Defaults to the current working directory.
        #[arg(name = "file")]
        files: Vec<String>,

        /// Automatically fix applicable lints.
        #[arg(long)]
        fix: bool,

        /// Enable potentially unsafe fixes.
        #[arg(long)]
        unsafe_fixes: bool,
    },

    /// Run tests.
    ///
    /// Uses `pytest` with `pytest-xdist` to run tests in the current working directory.
    ///
    /// Note that it is recommended to have `pytest` as a development dependency in your project
    /// so you get language server support when importing the `pytest` module.
    Test {
        /// Files or directories to test recursively. Defaults to the current working directory.
        #[arg(name = "file")]
        files: Vec<String>,

        /// Do not error when no tests are discovered.
        #[arg(long)]
        allow_no_tests: bool,

        /// Number of parallel tests to run. If not specified, it will be determined automatically.
        #[arg(long, short = 'j')]
        parallel: Option<i32>,

        /// Run only tests that contain the given substring. Same as `pytest -k`.
        #[arg(long)]
        filter: Option<String>,
    },
}

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
        Cmd::Test {
            files,
            allow_no_tests,
            parallel,
            filter,
        } => {
            tire::test::test(files, allow_no_tests, parallel, filter);
        }
    }
}
