# _Tire_ (WIP)

[Uv]: https://astral.sh/docs/uv

Tire is a tiny wrapper built on [Uv] that provides an ergonomic development experience in Python projects, covering
type checking with Mypy or Ty, linting and formatting with Ruff and testing with Pytest.

## Goal

Tire aims to provides a single, consistent and defragmented interface for the most common tasks in Python
development and simplify the assurance of consistent best practices across your Python projects.

## Features

- Out-of-the-box best practices for your Python project with strict settings
- Keep your `pyproject.toml` lean by using remote configuration profiles (inflating possible)
- Supports Uv workspaces
- (planned) Auto-discover dependencies from imports
- (planned) Editor configuration support

## Installation

Tire is written in Rust and can be installed with Cargo.

```console
$ cargo install tire
```

## Usage

The `tire` CLI comes with a set of sub-commands that cover the vast majority of commands run while working on Python
projects, such as running Python scripts or entrypoints, running tests, linting, formatting and type-checking.
Almost all commands delegate to Uv in one way or another.

* `tire add` is like `uv add`, with an additional `--auto` option
* `tire run` is mostly equivalent to `uv run`.
* `tire install` is mostly like `uv sync`
* `tire check` runs Mypy.
* `tire fmt` runs Ruff to format your code, organizes imports and `pyproject.toml`.
* `tire lint` runs Ruff to lint your code.
* `tire test` runs Pytest.
* `tire pyproject update` updates your `pyproject.toml`, optionally inflating it with the configuration Tire otherwise
injects dynamically.
* `tire pyproject diff` shows the difference between your current `pyproject.toml` and the one Tire would update it to.
* `tire task` runs a task defined in the `[tool.tire.task]` section of your `pyproject.toml`. 

## Configuration

Tire can be used directly with any Uv-compatible Python project. If you want to use another instead of the latest
default configuration profile, you can override it in your `pyproject.toml`:

```toml
# pyproject.toml
[tool.tire]
profile = "https://public.acme.org/tire-profile.v1.toml"
```

If you prefer Tire to apply the profile to your `pyproject.toml` instead of injecting the configuration dynamically,
run

```console
$ tire pyproject update --inflate
```

> This will set the `tool.tire.inflated=true` to let Tire remember to not inject the configuration from the profile
> again. You can use `tire pyproject diff` command to check if Tire would update your configuration from the profile
> again.

## Profiles

A profile is a partial `pyproject.toml` configuration that Tire combines with your project's `pyproject.toml` to
provide preferred default values for the tools it invokes. A profile's configuration may also be permanently applied to
your `pyproject.toml` by running `tire pyproject update --inflate`.

The `default` profile is embedded in Tire and provides a good starting point for most Python projects, but it is also
very opinionated by the Tire developers. Custom profiles can be stored remotely and configured in the
`tool.tire.profile` option to use instead. Remote profiles are cached locally, so you can continue to use Tire when
going offline.

## Development

If you have [Mise](https://mise.jdx.dev/), simply run

```console
$ mise install
$ mise run setup
```

This will install the required development tools and install the pre-commit hook.
