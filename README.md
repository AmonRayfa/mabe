<div align="center">
  <h1 align="center">Mabe</h1>
  <p align="center">
    Contributions, corrections, and requests can be made through GitHub, and the documentation is available <a href="https://mabe.readthedocs.io">here</a>.
  </p>
  <p align="center">Thank you for your interest in the project. Enjoy your reading! 🚀</p>
</div>

<div align="center">
  <a href="https://phased-versioning.koseka.net"><img src="https://img.shields.io/badge/Versioning-Phased-304CD3?style=flat&color=12398D" alt="Phased Versioning" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-Apache%202.0-723179?style=flat" alt="License" /></a>
  <br>
  <a href="https://github.com/AmonRayfa/mabe/releases"><img src="https://img.shields.io/github/v/tag/AmonRayfa/mabe?label=version&logo=github&color=579D52" alt="version" /></a>
  <a href="https://github.com/AmonRayfa/mabe"><img src="https://img.shields.io/github/created-at/AmonRayfa/mabe?logo=github&label=created&color=C9443C" alt="created" /></a>
  <a href="https://github.com/AmonRayfa/mabe/commits/main"><img src="https://img.shields.io/github/last-commit/AmonRayfa/mabe?display_timestamp=committer&logo=github&color=438240" alt="last commit" /></a>
  <a href="https://github.com/AmonRayfa/mabe/milestones"><img src="https://img.shields.io/github/milestones/all/AmonRayfa/mabe?logo=github&color=5288DF" alt="milestones" /></a>
  <a href="https://github.com/AmonRayfa/mabe/stargazers"><img src="https://img.shields.io/github/stars/AmonRayfa/mabe?style=flat&logo=github&color=DCB456" alt="stars" /></a>
  <br>
  <a href="Cargo.toml"><img src="https://img.shields.io/badge/Dependencies-5-black?style=flat&logo=rust&logoColor=black" alt="Dependencies" /></a>
  <a href="Cargo.toml"><img src="https://img.shields.io/badge/Size-10.2kB-black?style=flat&logo=rust&logoColor=black" alt="Size" /></a>
</div>

---

**Mabe** is an ergonomic, opinionated library for creating debug-friendly errors in Rust. It simplifies error handling by providing an [`Error`](https://mabe.readthedocs.io/en/stable/mabe_derive/derive.Error.html) derive macro for defining structured errors and a [`main`](https://mabe.readthedocs.io/en/stable/mabe/attr.main.html) attribute macro that formats error chains into a clean, readable tree structure in the terminal.

<h2><img height="20" alt="branches" src="./img/branches.svg">&nbsp;&nbsp;Branches</h2>

| Branch | Description                                                     |
| :----- | :-------------------------------------------------------------- |
| `v1`   | The latest production branch.                                   |
| `dev`  | The development branch; regularly merged into `v1` when stable. |

**Note for Contributors:** Please submit all feature requests and standard bug fixes to the **`dev`** branch.

<h2><img height="20" alt="installation" src="./img/installation.svg">&nbsp;&nbsp;Installation</h2>

To use the **latest stable version** of the project, add the repository link targeting the `v1` branch to your `Cargo.toml` file:

```toml
[dependencies]
mabe = { git = "https://github.com/AmonRayfa/mabe", branch = "v1" }
```

To use the **nightly version**, you can change the branch to `dev`:

```toml
[dependencies]
mabe = { git = "https://github.com/AmonRayfa/mabe", branch = "dev" }
```

You can now use the [`main`](https://mabe.readthedocs.io/en/stable/mabe/attr.main.html) attribute on your entry point, along with the re-exported [`bail`](https://docs.rs/anyhow/latest/anyhow/macro.bail.html), [`Context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html), and [`Result`](https://docs.rs/anyhow/latest/anyhow/type.Result.html) items from the [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) crate.

If you enable the `structured` feature, you can use the [`Error`](https://mabe.readthedocs.io/en/stable/mabe_derive/derive.Error.html) derive macro to define custom error enums with interpolated messages.

<h2><img height="20" alt="usage" src="./img/usage.svg">&nbsp;&nbsp;Usage</h2>

The following example demonstrates how to define a structured error enum and how to chain errors together to produce a detailed traceback.

### Define your Errors

```rust
// ./src/error.rs

use mabe::Error; // Only available for the "structured" feature.

#[derive(Error)]
pub enum ServerError {
    #[error("You are not authorized to access this resource. Try using a different account.")]
    Unauthorized,

    // Interpolates the values of the 1st and 2nd field in the error message.
    #[error("Network failure\nCode {0}: {1}")]
    NetworkFailure(u32, String),

    // Interpolates the value of the `cause` and `retry_in` fields in the error message.
    #[error("Connection lost --> {cause}. Retry in {retry_in} seconds.")]
    ConnectionLost { cause: String, retry_in: u32 }
}
```

### Use in Main

```rust
// ./src/main.rs

use mabe::{ bail, Context, Result };
use crate::error::ServerError;

// A dummy function that returns an error if the `value` is `false`.
fn dummy_function(value: bool) -> Result<()> {
    match value {
        true => Ok(()),
        false => bail!(ServerError::ConnectionLost {
            cause: "Server down".to_string(),
            retry_in: 10
        })
    }
}

#[mabe::main]
fn main() -> Result<()> {
    // We wrap the errors using `.context()` to create a traceback.
    dummy_function(false)
        .context("Some other context error.")
        // You can also use structured errors as context.
        .context(ServerError::NetworkFailure(404, "Not Found".to_string()))
        .context("Some context error.")
        .context(ServerError::Unauthorized)
}
```

### Output

When running the code above, the terminal output will be:

```plaintext
[X] You are not authorized to access this resource. Try using a different account.
 ├─ Some context error.
 ├─ Network failure
 │  Code 404: Not Found
 ├─ Some other context error.
 └─ Connection lost --> Server down. Retry in 10 seconds.
```

For further details on how to use the project, please refer to the [documentation](https://mabe.readthedocs.io).

<h2><img height="20" alt="security" src="./img/security.svg">&nbsp;&nbsp;Security</h2>

Vulnerabilities and sensitive information should not be reported via public GitHub issues. Please refer to the [Security Policy](SECURITY.md) for details on supported versions and instructions on how to responsibly disclose security concerns.

<h2><img height="20" alt="contributing" src="./img/contributing.svg">&nbsp;&nbsp;Contributing</h2>

This project is open to contributions and suggestions, and any help or feedback is highly appreciated. There is no code of conduct, but please be respectful and considerate when engaging with the community.

This project uses [Phased Versioning](https://phased-versioning.koseka.net), which defines the versioning, branching, and release rules, and commit messages follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification. So, make sure to read both first before contributing to the project in any way. Additionally, please refer to the [Contribution Guide](CONTRIBUTING.md) for setup instructions and guidance on how to contribute the project.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, shall be licensed as bellow, without any additional terms or conditions.

<h2><img height="20" alt="license" src="./img/license.svg">&nbsp;&nbsp;License</h2>

Copyright 2026 Amon Rayfa.

This project is licensed under the [Apache License (Version 2.0)](LICENSE).
