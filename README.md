# Mabe

[**Mabe**](https://crates.io/crates/mabe) is a _Rust procedural macro crate_ that provides tools for creating simple and
well-structured error enums for easy debugging. Each variant in the enum can have an error, reason, and solution message. This
allows for a more detailed error handling and debugging process. Also, when an error is printed, the error, reason, and solution
messages are displayed in a structured and readable format.

## Getting Started

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
mabe = "0.3"
```

You can now use the `Mabe` derive macro and its attributes to create your own error enums as shown in the example below:

```rust
use mabe::Mabe;

#[derive(Debug, Mabe)]
pub enum ServerError {
    #[error("You are not authorized to access this resource.")]
    #[reason("Your account does not have the required permissions.")]
    #[solution("Try using a different account.")]
    Unauthorized,
}

fn main() {
    let error = ServerError::Unauthorized;
    println!("{}", error);
}
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

#[derive(Debug, Mabe)]
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

fn main() {
    let error1 = ServerError::NetworkFailure(404, "Not Found".to_string());
    println!("{}", error1);

    let error2 = ServerError::ConnectionLost { reason: "Server down".to_string(), retry_in: 10 };
    println!("{}", error2);
}
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

The project follows the [Koseka Contribution Guidelines (Version 1.0)](https://koseka.org/contribution-guidelines/1.0) which
provides standardized rules and guidelines for contributing to projects, so make sure to read this first before contributing to
the project in any way. Additionally, you can also read the [DEVELOPMENT.md](DEVELOPMENT.md) file for more information on how
the project is structured and what you can do to help.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, shall be
licensed as bellow, without any additional terms or conditions.

## License

Copyright 2024 Amon Rayfa.

This project is licensed under the [Apache License (Version 2.0)](LICENSE).
