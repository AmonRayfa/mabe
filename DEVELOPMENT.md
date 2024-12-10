# Mabe Development

This file is primarily intended for developers who wish to fork the project and potentially contribute to it. It provides
instructions on setting up the development environment, linting and formatting the code, and testing and building the project.

## Table of Contents

- [Setting Up the Development Environment](#setting-up-the-development-environment)
- [Linting and Formatting the Code](#linting-and-formatting-the-code)
- [Testing and Building the Project](#testing-and-building-the-project)

## Setting Up the Development Environment

First, ensure that you have the latest version of **Rust** installed on your machine. You can install **Rust** by following the
instructions on the official [Rust website](https://www.rust-lang.org/tools/install).

Second, this project uses [**Trunk**](https://www.trunk.io) as an npm package for formatting and linting the code, and **pnpm**
as a package manager. So, make sure you have **node.js** and **pnpm** installed on your machine. You can install both of them
from the official websites: [node.js](https://nodejs.org) and [pnpm](https://pnpm.io/installation).

Next, clone the `mabe` repository to your local machine and install the development dependencies:

```sh
git clone https://github.com/AmonRayfa/mabe.git           # Clones the repository.
cd mabe                                                   # Moves into the project directory.
pnpm install                                              # Installs the development dependencies.
```

Since **Trunk** is used for formatting the code, it's best if you disable the _format on save_ option in your editor to avoid
potential conflicts with the project's formatting configurations.

If you are using [**Zed**](https://zed.dev), you can locally disable the _format on save_ option of your editor for this project
by adding the following line to the `.zed/settings.json` file at the root of the project directory:

```json
{
	"format_on_save": "off"
}
```

If you are using [**VSCode**](https://code.visualstudio.com), you can locally disable the _format on save_ option of your editor
for this project by adding the following line to the `.vscode/settings.json` file at the root of the project directory:

```json
{
	"editor.formatOnSave": false
}
```

As for the linting, the project comes with its own linters and configurations, so if you have your own linters installed with
custom configurations, you should make sure they don't conflict with the project's linters. You can check the list of linters
(and formatters) along with their configurations in the `.trunk/trunk.yaml` file and the `.trunk/configs/` directory.

If you have followed all the steps correctly, you should now have a working development environment for the project. If you
encounter any issues, feel free to open an issue on the project's [GitHub repository](https://github.com/AmonRayfa/mabe/issues).

## Linting and Formatting the Code

The linters and formatters work through git hooks, so they will run automatically when you commit changes. However, it's best to
also run them manually before committing changes to avoid failing the commit hook.

To make sure **Trunk** is managing the git hooks, you can run the following command:

```sh
pnpm trunk git-hooks sync
```

You can manually run the linters and formatters using the following commands:

```sh
pnpm trunk check                                          # Runs linters and formatters on all the changed files.
pnpm trunk check --all                                    # Runs linters and formatters on all the files in the repository.
```

You can manually format the code using the following commands:

```sh
pnpm trunk fmt                                            # Formats all the changed files.
pnpm trunk fmt --all                                      # Formats all the files in the repository.
```

If you want to know more about **Trunk**, you can check the [Trunk Documentation](https://docs.trunk.io).

## Testing and Building the Project

All tests are located in the `src/tests/` directory and can be run using the following commands:

```sh
cargo test                                                # Runs all the tests in the project.
cargo test -- --nocapture                                 # Runs all the tests in the project and displays their output.
cargo test --lib utils::helpers::tests                    # Runs the tests in the `tests` module of `src/utils/helpers/mod.rs`.
cargo test --test unit_variants                           # Runs the tests in `tests/unit_variants.rs`.
cargo test --features colorize                            # Runs all the tests in the project for the `colorize` feature.
cargo test --all-features                                 # Runs all the tests in the project for all the features.
```

You can build the project using the following commands:

```sh
cargo doc --open                                          # Generates the Rust documentation and opens it in the browser.
cargo build                                               # Builds the project in debug mode.
cargo build --release                                     # Builds the project in release mode.
cargo build --features colorize                           # Builds the project with the `colorize` feature.
cargo build --all-features                                # Builds the project with all the features.
```

If you want to know more about **Cargo**, you can check the [Cargo Documentation](https://doc.rust-lang.org/cargo).
