# _Tire_ (WIP)

[Uv]: https://astral.sh/docs/uv

Tire provides a single interface for the most common workflows in a typical, modern Python project. It is built on the
fantastic [Uv] and provides an ergonomic development experience, covering type checking, testing, linting and
formatting, all the while keeping configuration minimal with modern and sane defaults.

## Features

- Out-of-the-box best practices for your Python project with strict settings
- Keep your `pyproject.toml` lean by using remote configuration profiles
- (planned) Supports Uv workspaces
- (planned) Auto-discover dependencies from imports
- (planned) Editor configuration support

## Installation

Tire is written in Rust and can be installed with Cargo.

```console
$ cargo install tire
```

## Usage

Check your project's typing with `tire check`:

```console
$ tire check
Daemon started
Success: no issues found in 2 source files
```

Lint your code by running `tire lint`:

```console
$ tire lint
All checks passed!
```

Format your code with `tire fmt` (incl. organized imports):

```console
$ tire fmt
2 files left unchanged          # from `ruff format`
All checks passed!              # from `ruff lint --fix --select I`
```

Run tests with `tire test` (incl. parallel by default and with doctests):

```console
$ tire test
============================= test session starts ==============================
platform darwin -- Python 3.11.13, pytest-8.4.1, pluggy-1.6.0
rootdir: /var/folders/m1/tnpq610n5dv3nzmt_wmq4x040000gp/T/.tmpRh6vdD
configfile: pyproject.toml
plugins: xdist-3.7.0
12 workers [1 item]       
...
```

Run a script (alias for `uv run`):

```console
$ tire run main.py
```

Or invoke a function call (wrapped with [cyclopts](https://github.com/BrianPugh/cyclopts)):

```console
$ tire run hello:main --help
Usage: main:main [ARGS] [OPTIONS]

Say hello.

╭─ Commands ───────────────────────────────────────────────────────────────────╮
│ --help -h  Display this message and exit.                                    │
╰──────────────────────────────────────────────────────────────────────────────╯
╭─ Parameters ─────────────────────────────────────────────────────────────────╮
│ *  NAME --name  [required]                                                   │
╰──────────────────────────────────────────────────────────────────────────────╯
```

Run tasks defined under `[tool.tire.tasks.*]`:

```console
$ tire run start
[ tire ] run $ tire run server:main ...
```

## Configuration

Tire can be used directly with any Uv-compatible Python project. If you want to use another instead of the latest
default configuration profile, you can override it in your `pyproject.toml`:

```toml
# pyproject.toml
[tool.tire]
profile = "https://public.acme.org/tire-profile.v1.toml"
```

## Profiles

A profile is a partial `pyproject.toml` configuration that Tire combines with your project's `pyproject.toml` to
provide preferred default values for the tools it invokes.

The `default` profile is embedded in Tire and provides a good starting point for most Python projects, but it is also
very opinionated by the Tire developers. Custom profiles can be stored remotely and configured in the
`tool.tire.profile` option to use instead. Remote profiles are cached locally, so you can continue to use Tire when
going offline.

Settings in the `pyproject.toml` take precedence over settings configured in a profile, allowing you to still customize
specific settings while also benefitting from a centralized and common configuration profile.

## Development

If you have [Mise](https://mise.jdx.dev/), simply run

```console
$ mise install
$ mise run setup
```

This will install the required development tools and install the pre-commit hook.
