# Mabe

Mabe is a **Rust** library that provides a _derive macro_ (`Mabe`) for creating simple and well-structured error enums for easy
debugging. Each variant in the enum can have an error, reason, and solution message. This allows for a more detailed error
handling and debugging process. Also, when an error is printed, the error, reason, and solution messages are displayed in a
structured and colorized format for better readability.

## Getting Started

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
mabe = "0.2"
```

You can now use the `Mabe` derive macro and its attributes to create your own error enums as shown in the example below:

```rust
use mabe::Mabe;

#[derive(Debug, Mabe)]
pub enum SystemError {
    #[error("Failed to build the system.")]
    #[reason("The error occurred because ...")]
    #[solution("Try doing ...")]
    BuildFailure,
}
```

You can also interpolate the values of variant fields in the error, reason, and solution messages as shown below:

```rust
use mabe::Mabe;

#[derive(Debug, Mabe)]
pub enum ServerError {
    #[error("You are not authorized to access this resource.")]
    #[solution("Try using a different account.")]
    Unauthorized,

    #[error("Network failure.")]
    // Interpolating the values of the 1st and 2nd field in the reason message.
    #[reason("Code {0}: {1}.")]
    NetworkFailure(u32, String),

    #[error("Connection lost.")]
    // Interpolating the value of the `reason` field in the reason message.
    #[reason("{reason}")]
    // Interpolating the value of the `retry_in` field in the solution message.
    #[solution("Retry in {retry_in} seconds.")]
    ConnectionLost { reason: String, retry_in: u32 }
}
```

The `Mabe` derive macro is quite resilient, as a compile error will only occur if one of the following rules is violated:

1. The element on which `Mabe` is used must be an enum.
2. The enum must have at least one variant.
3. Each attribute must have exactly one argument of type `&str` (string literal).
4. Each variant field must be interpolated in at least one of the attribute messages of the variant.

## Contributing

This project is open to contributions and suggestions, and any help or feedback is highly appreciated. There is no code of
conduct, but please be respectful and considerate when engaging with the community.

The project follows the [Koseka Contribution Guidelines (Version 1.0)](https://koseka.org/contribution-guidelines/1.0) which
provides standardized rules and guidelines for contributing to projects, so make sure to read this first before contributing to
the project in any way. Additionally, you can also read the [DEVELOPMENT.md](DEVELOPMENT.md) file for more information on how
the project is structured and what you can do to help.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, shall be
licensed as bellow, without any additional terms or conditions.

## License

Copyright 2024 Amon Rayfa.

This project is licensed under the [Apache License (Version 2.0)](LICENSE).
