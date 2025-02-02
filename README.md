<p align="center">
  <h1 align="center">Mabe</h1>
  <p align="center">Contributions, corrections, and requests can be made through GitHub, and the documentation is available on the platforms linked bellow.</p>
  <p align="center">Thank you for your interest in the project, enjoy your reading! 🚀</p>
</p>

<p align="center">
  <a href="https://github.com/AmonRayfa/mabe"><img alt="GitHub: created in" src="https://img.shields.io/github/created-at/AmonRayfa/mabe?logo=github&label=created%20in&color=red"/></a>
  <a href="https://github.com/AmonRayfa/mabe"><img alt="GitHub: last commit" src="https://img.shields.io/github/last-commit/AmonRayfa/mabe?display_timestamp=committer&logo=github&color=yellow"/></a>
  <a href="https://github.com/AmonRayfa/mabe"><img alt="GitHub: milestones" src="https://img.shields.io/github/milestones/all/AmonRayfa/mabe?logo=github&color=blue"/></a>
  <a href="https://github.com/AmonRayfa/mabe"><img alt="GitHub: CI/CD" src="https://img.shields.io/github/actions/workflow/status/AmonRayfa/mabe/ci-cd.yaml?branch=main&logo=github&label=CI%2FCD"/></a>
  <br/>
  <a href="https://crates.io/crates/mabe"><img alt="Crates.io: size" src="https://img.shields.io/crates/size/mabe?logo=rust&logoColor=black&color=black"/></a>
  <a href="https://crates.io/crates/mabe"><img alt="Crates.io: dependents" src="https://img.shields.io/crates/dependents/mabe?logo=rust&logoColor=black&color=black"/></a>
</p>

## Introduction

**Mabe** is a simple framework for creating debug-friendly error enums in Rust. Each variant in the enum can encapsulate an
error and a debug message, and errors are presented in a structured format, displaying the messages defined for the variant.
This allows for a more detailed and clear debugging process.

## Getting Started

In order to use **Mabe** in your project, you need to add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
mabe = "1"
```

You can now use the [`Mabe`](https://docs.rs/mabe/latest/mabe/derive.Mabe.html) derive macro and its attributes to create your
own error enums by following the examples below.

## Examples

Here is a simple example of how to create a debug-friendly error enum:

```rust
use mabe::Mabe;

#[derive(Mabe)]
pub enum ServerError {
    #[error("You are not authorized to access this resource.")]
    #[debug("Try using a different account.")]
    Unauthorized,
}

let error = ServerError::Unauthorized;
println!("{}", error);
```

```plaintext
Output:
[error] You are not authorized to access this resource.
[debug] Try using a different account.
```

You can also interpolate the values of variant fields in the error and debug messages as shown below:

```rust
use mabe::Mabe;

#[derive(Mabe)]
pub enum ServerError {
    // Interpolates the values of the 1st and 2nd field in the error message.
    #[error("Network failure. --> Code {0}: {1}.")]
    NetworkFailure(u32, String),

    // Interpolates the value of the `cause` field in the error message.
    #[error("Connection lost. --> {cause}.")]
    // Interpolates the value of the `retry_in` field in the debug message.
    #[debug("Retry in {retry_in} seconds.")]
    ConnectionLost { cause: String, retry_in: u32 }
}

let error1 = ServerError::NetworkFailure(404, "Not Found".to_string());
println!("{}", error1);

let error2 = ServerError::ConnectionLost { cause: "Server down".to_string(), retry_in: 10 };
println!("{}", error2);
```

```plaintext
Output:
[error] Network failure. --> Code 404: Not Found.

[error] Connection lost. --> Server down.
[debug] Retry in 10 seconds.
```

## Contributing

This project is open to contributions and suggestions, and any help or feedback is highly appreciated. There is no code of
conduct, but please be respectful and considerate when engaging with the community.

The project follows the [Koseka Project Guidelines](https://koseka.org/project-guidelines), which provide standardized rules and
recommendations for project development. Make sure to read these guidelines first before contributing to the project in any way.
Additionally, please refer to the [DEVELOPMENT](DEVELOPMENT.md) file for setup instructions and guidance on developing, testing,
and building the project.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, shall be
licensed as bellow, without any additional terms or conditions.

## License

Copyright 2024 Amon Rayfa.

This project is licensed under the [Apache License (Version 2.0)](LICENSE).
