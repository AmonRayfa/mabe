# Mabe

[**Mabe**](https://crates.io/crates/mabe) is a simple framework for creating debug-friendly error enums in Rust. Each variant in
the enum can include an error, reason, and solution message, and errors are displayed in a structured format, showing the
messages defined for the variant. This allows for a more detailed and clear debugging process.

## Table of Contents

- [Getting Started](#getting-started)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

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
    #[reason("Your account does not have the required permissions.")]
    #[solution("Try using a different account.")]
    Unauthorized,
}

let error = ServerError::Unauthorized;
println!("{}", error);
```

```plaintext
Output:
[error] You are not authorized to access this resource.
[reason] Your account does not have the required permissions.
[solution] Try using a different account.
```

You can also interpolate the values of variant fields in the error, reason, and solution messages as shown below:

```rust
use mabe::Mabe;

#[derive(Mabe)]
pub enum ServerError {
    #[error("Network failure.")]
    // Interpolates the values of the 1st and 2nd field in the reason message.
    #[reason("Code {0}: {1}.")]
    NetworkFailure(u32, String),

    #[error("Connection lost.")]
    // Interpolates the value of the `reason` field in the reason message.
    #[reason("{reason}")]
    // Interpolates the value of the `retry_in` field in the solution message.
    #[solution("Retry in {retry_in} seconds.")]
    ConnectionLost { reason: String, retry_in: u32 }
}

let error1 = ServerError::NetworkFailure(404, "Not Found".to_string());
println!("{}", error1);

let error2 = ServerError::ConnectionLost { reason: "Server down".to_string(), retry_in: 10 };
println!("{}", error2);
```

```plaintext
Output:
[error] Network failure.
[reason] Code 404: Not Found.
[solution]

[error] Connection lost.
[reason] Server down.
[solution] Retry in 10 seconds.
```

## Contributing

This project is open to contributions and suggestions, and any help or feedback is highly appreciated. There is no code of
conduct, but please be respectful and considerate when engaging with the community.

The project follows the [Koseka Contribution Guidelines](https://koseka.org/contribution-guidelines) which provides standardized
rules and guidelines for contributing to projects, so make sure to read this first before contributing to the project in any
way. Additionally, please refer to the [DEVELOPMENT](DEVELOPMENT.md) file for setup instructions and guidance on developing,
testing, and building the project.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, shall be
licensed as bellow, without any additional terms or conditions.

## License

Copyright 2024 Amon Rayfa.

This project is licensed under the [Apache License (Version 2.0)](LICENSE).
