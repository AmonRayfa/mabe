# Mabe Development

This file is mainly intended for developers who want to contribute to the [Mabe](README.md) project. It provides information on
how to set up the development environment, run tests, and build the project, as well as potential new features and improvements.

The project follows the [Koseka Contribution Guidelines (Version 1.0)](https://koseka.org/contribution-guidelines/1.0) which
provides standardized rules and guidelines for contributing to projects, so make sure to read this first before contributing to
the project in any way.

Remember that unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the project by
you, shall be licensed under the [Apache-2.0 License](LICENSE), without any additional terms or conditions.

## Table of Contents

- [Setting Up the Development Environment](#setting-up-the-development-environment)
- [Running Tests](#running-tests)
- [Building the Project](#building-the-project)
- [Potential New Features and Improvements](#potential-new-features-and-improvements)

## Setting Up the Development Environment

First, ensure that you have the latest version of **Rust** installed on your machine. You can install **Rust** by following the
instructions on the official [Rust website](https://www.rust-lang.org/tools/install).

Second, this project uses [Trunk](https://www.trunk.io) as an npm package for formatting and linting the code, and **pnpm** as a
package manager. So, make sure you have **node.js** and **pnpm** installed on your machine. You can install both of them from
the official websites: [node.js](https://nodejs.org) and [pnpm](https://pnpm.io/installation).

Next, clone the **mabe** repository to your local machine and navigate to the project directory:

```sh
git clone https://github.com/AmonRayfa/mabe.git
cd mabe
```

Then, install the required dependencies using the following command:

```sh
pnpm install
cargo build
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

The linters and formatters work through git hooks, so they will run automatically when you commit changes. If you want more
information on how they work, you can check the [Trunk documentation](https://docs.trunk.io).

If you have followed all the steps correctly, you should now have a working development environment for the project. If you
encounter any issues, feel free to open an issue on the project's [GitHub repository](https://github.com/AmonRayfa/mabe/issues).

## Running Tests

All tests are located in the `src/tests/` directory and can be run using the following command:

```sh
cargo test
```

This command will run all the tests in the project and display the results in the terminal. If you want to run a specific test
or a specific set of tests, run the following command:

```sh
cargo test <test_name_1> <test_name_2> ...
```

If you want to run the tests and display the output of the `println!` macros in the tests, you can use the following command:

```sh
cargo test -- --nocapture
```

If you want to run the tests for the **colored** feature, you can use the following command:

```sh
cargo test --features colored
```

If you want to run the tests for all the features, you can use the following command:

```sh
cargo test --all-features
```

You can also combine these commands to run more specific tests. For example, if you want to run the tests for the **colored**
feature and display the output of the `println!` macros, you can use the following command:

```sh
cargo test --features colored -- --nocapture
```

## Building the Project

You can build the project using the following command:

```sh
cargo build
```

If you want to build the project in release mode, you can use the following command:

```sh
cargo build --release
```

If you want to build the project with the **colored** feature, you can use the following command:

```sh
cargo build --features colored
```

If you want to build the project with all the features, you can use the following command:

```sh
cargo build --all-features
```

You can also combine these commands to build the project with more specific configurations. For example, if you want to build
the project with the **colored** feature in release mode, you can use the following command:

```sh
cargo build --features colored --release
```

## Potential New Features and Improvements

There are several features and improvements that can be added to the Mabe project in the future. Some of these include:

[ ] Finding a way to make it so that the **colored** feature can be used without the need for the user to add the [colored](https://crates.io/crates/colored)
crate to their project and to import the [`Colorize`](https://docs.rs/colored/2.1.0/colored/trait.Colorize.html) trait where their
error enums are defined.

[ ] Creating macros that can be used on other error enums that were not defined by the user in order to retrieve the error, reason,
and solution messages.
