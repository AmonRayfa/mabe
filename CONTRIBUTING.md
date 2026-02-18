# Contribution Guide

This file is primarily intended for developers who wish to fork the project and potentially contribute to it. This project adheres to the [Koseka Standard](https://koseka.net/book/standard), which provides standardized versioning and contribution rules. So, make sure to read it first before contributing to the project in any way.

## Project Structure

Here are the main directories and files in the project:

```plaintext
.
├── docs/
├── mabe_attr/
│   ├── src/
│   │   └── lib.rs
│   └── Cargo.toml
├── mabe_derive/
│   ├── src/
│   │   ├── api/
│   │   ├── error/
│   │   ├── utils/
│   │   └── lib.rs
│   ├── tests/
│   └── Cargo.toml
├── src/
│   ├── internal.rs
│   └── lib.rs
├── Cargo.toml
└── package.json
```

The project is organized as a Rust workspace centered around `src/`, which serves as the primary library entry point and re-exports functionality from the internal macro crates (and the [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) crate). The `mabe_attr` directory contains the logic for the opaque `#[mabe::main]` attribute, while `mabe_derive` houses the implementation for the structured `#[derive(Error)]` macro. Additionally, the `package.json` file manages the development environment, containing configurations for linting and formatting via Trunk, as well as scripts for generating the API reference.

See the [API reference](https://mabe.readthedocs.io/en/latest/) for a more detailed overview of the project structure.

## Setting Up the Development Environment

First, ensure that you have the latest version of **Rust** installed on your machine. You can install **Rust** by following the instructions on the official [Rust website](https://www.rust-lang.org/tools/install).

Second, this project uses [**Trunk**](https://www.trunk.io) as an npm package for formatting and linting the code, and **npm** as a package manager. So, make sure you have **node.js** and **npm** installed on your machine. You can install both of them from the official [node.js](https://nodejs.org) website.

Next, clone the `mabe` repository to your local machine and install the development dependencies:

```sh
git clone https://github.com/AmonRayfa/mabe.git             # Clones the repository.
cd mabe                                                     # Moves into the project directory.
npm install                                                 # Installs the development dependencies.
```

Since **Trunk** is used for formatting the code, it's best if you disable the _format on save_ option in your editor to avoid potential conflicts with the project's formatting configurations.

If you are using [**Zed**](https://zed.dev), you can locally disable the _format on save_ option of your editor for this project by adding the following line to the `.zed/settings.json` file at the root of the project directory:

```json
{
  "format_on_save": "off"
}
```

If you are using [**VSCode**](https://code.visualstudio.com), you can locally disable the _format on save_ option of your editor for this project by adding the following line to the `.vscode/settings.json` file at the root of the project directory:

```json
{
  "editor.formatOnSave": false
}
```

As for the linting, the project comes with its own linters and configurations, so if you have your own linters installed with custom configurations, you should make sure they don't conflict with the project's linters. You can check the list of linters (and formatters) along with their configurations in the `.trunk/trunk.yaml` file and the `.trunk/configs/` directory.

If you have followed all the steps correctly, you should now have a working development environment for the project. If you encounter any issues, feel free to open an issue on the project's [GitHub repository](https://github.com/AmonRayfa/mabe/issues).

## Linting and Formatting the Code

The linters and formatters work through git hooks, so they will run automatically when you commit changes. However, it's best to also run them manually before committing changes to avoid failing the commit hook.

To make sure **Trunk** is managing the git hooks, you can run the following command:

```sh
npm run trunk git-hooks sync
```

You can manually run the linters and formatters using the following commands:

```sh
npm run check                                               # Runs linters and formatters on all the changed files.
npm run check --all                                         # Runs linters and formatters on all the files in the repository.
```

You can manually format the code using the following commands:

```sh
npm run fmt                                                 # Formats all the changed files.
npm run fmt --all                                           # Formats all the files in the repository.
```

If you want to know more about **Trunk**, you can check the [Trunk documentation](https://docs.trunk.io).

## Testing and Building the Project

Currently, there are only tests for `mabe_derive`, but here are some general test commands:

```sh
cargo test --workspace                                      # Runs all the tests in the project.
cargo test -p mabe_derive                                   # Runs all the tests in `mabe_derive`.
cargo test -p mabe_derive --test unit_variants              # Runs the tests in `mabe_derive/tests/unit_variants.rs`.
cargo test --workspace --features structured                # Runs all the tests in the project for the `structured` feature.
cargo test --workspace --all-features                       # Runs all the tests in the project for all the features.
```

You can build the project using the following commands:

```sh
cargo build                                                 # Builds the project in debug mode.
cargo build --release                                       # Builds the project in release mode.
cargo build --features structured                           # Builds the project with the `structured` feature.
cargo build --all-features                                  # Builds the project with all the features.
```

You can generate the documentation using the following command:

```sh
npm run doc                                                 # Generates the documentation at `docs/gen/`.
npm run doc:open                                            # Generates the documentation and opens it in the browser.
```

If you want to know more about **Cargo**, you can check the [Cargo documentation](https://doc.rust-lang.org/cargo).

## License

Copyright 2026 Amon Rayfa.

This project is licensed under the [Apache License (Version 2.0)](LICENSE).
