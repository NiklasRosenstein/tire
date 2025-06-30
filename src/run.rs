//! Implements the behaviour of the `tire run` command.

use crate::utils::{run_command_or_exit, string_vec};

pub fn run(args: Vec<String>) {
    // Extract options until the first positional argument to pass to `uv run`.
    // TODO: Should we assume options with the `--` prefix to always consume an additional arg?
    let mut uv_args = Vec::new();
    let mut target_args = Vec::new();
    let mut target: Option<String> = None;
    for arg in args {
        if target.is_none() {
            if !arg.starts_with("-") {
                target = Some(arg);
            } else {
                uv_args.push(arg);
            }
        } else {
            target_args.push(arg);
        }
    }

    if target.is_none() {
        eprintln!("Missing positional argument");
        std::process::exit(1);
    }
    let target = target.unwrap();

    eprintln!("uv_args={uv_args:?}");
    eprintln!("target={target:?}");
    eprintln!("target_args={target_args:?}");

    let mut uv_command: Vec<String>;

    // If the target starts with `@`, it references a package name and the command is similar
    // to using `uvx`.
    if target.starts_with("@") {
        uv_command = string_vec!["uv", "run", "--with", target.strip_prefix("@").unwrap()];
        uv_command.extend(uv_args);
        uv_command.push(String::from(target.strip_prefix("@").unwrap()));
    }
    // If the target contains a colon, it is a function call in a module. We wrap it into a
    // `cyclopts.App` for advanced argument parsing and help formatting.
    else if target.contains(":") {
        let module = target.split(':').next().unwrap();
        let func = target.split(':').nth(1).unwrap();
        let code = format!(
            "import sys, cyclopts, {module}; \
            app = cyclopts.App(name='{module}:{func}', version_flags=[]); \
            app.default({module}.{func}); \
            app();"
        );
        uv_command = string_vec![
            "uv",
            "run",
            "--with",
            "cyclopts>=3.0.0,<4.0.0",
            "python",
            "-c",
            code.as_str()
        ];
    }
    // Otherwise we pass it to UV directly.
    else {
        uv_command = string_vec!["uv", "run"];
    }

    // Append the arguments for the called target.
    uv_command.extend(target_args);

    eprintln!("uv_command={uv_command:?}");

    // Invoke the command.
    run_command_or_exit(uv_command)
}
