# Mabe

[中文](translations/中文/README.md) | [Русский](translations/Русский/README.md) | [Español](translations/Español/README.md) |
[Français](translations/Français/README.md) | [日本語](translations/日本語/README.md) | [한국어](translations/한국어/README.md)
| [Português](translations/Português/README.md) | [Italiano](translations/Italiano/README.md) |
[हिन्दी](translations/हिन्दी/README.md) | [العربية](translations/العربية/README.md)

-> Write an introduction.

## Use case

```rust
use mabe::Mabe;

#[derive(Debug, Mabe)]
pub enum ServerError {
    #[error("You are not authorized to access this resource.")]
    #[solution("Try using a different account.")]
    Unauthorized,

    #[error("Network failure.")]
    #[reason("Code {0}: {1}.")]
    NetworkFailure(u32, String),

    #[error("Connection lost.")]
    #[reason("{reason}")]
    #[solution("Retry in {retry_in} seconds.")]
    ConnectionLost { reason: String, retry_in: u32 }
}
```

## Details

If you try to use a field that does not exist in the variant, an error will be thrown:

```rust
use mabe::Mabe;

#[derive(Debug, Mabe)]
pub enum ServerError {
    #[error("You are not authorized to access this resource.")]
    #[solution("Try using a different account.")]
    Unauthorized,
}
```

If you ...

## Contributing

If you are a developer that wants to participate at improving the library, here are a couple of things you should know first...

This project follows the [Koseka Contribution Guidelines (Version 1.0)](https://koseka.org/contribution-guidelines/1.0) so make
sure to read this first before contributing to the project in any way.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, shall be
licensed as bellow, without any additional terms or conditions.

## License

Copyright 2024 Amon Rayfa.

This project is licensed under the [Apache License (Version 2.0)](LICENSE).
