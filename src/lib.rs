// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! [**Mabe**](https://github.com/AmonRayfa/mabe) is a simple library for creating debug-friendly errors in Rust. This is
//! achieved thanks to two _procedural macros_: the [`Error`](../mabe_derive/derive.Error.html) derive macro to create
//! structured error enums, and the [`main`] attribute macro that formats error chains into a clean, readable tree structure in
//! the terminal.
//!
//! # Cargo Features
//!
//! The following is a list of default and optional
//! [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/features.html#the-features-section) that can be enabled or
//! disabled in the `Cargo.toml` file.
//!
//! ### Default Features
//!
//! * **colorize**: Adds colors to the error chain in terminal. This feature only works with ANSI-compatible terminals.
//!
//! ### Optional Features
//!
//! * **structured**: Allows users to create error enums by using the [`Error`](../mabe_derive/derive.Error.html) derive
//!   macro and its `error` attribute.
//!
//! # Installation
//!
//! To use the **latest stable version** of the project, add the repository link targeting the `v1` branch to your `Cargo.toml`
//! file:
//!
//! ```toml
//! [dependencies]
//! mabe = { git = "https://github.com/AmonRayfa/mabe", branch = "v1" }
//! ```
//!
//! To use the **nightly version**, you can change the branch to `dev`:
//!
//! ```toml
//! [dependencies]
//! mabe = { git = "https://github.com/AmonRayfa/mabe", branch = "dev" }
//! ```
//!
//! You can now use the [`main`] attribute on your entry point, along with the re-exported
//! [`bail`](https://docs.rs/anyhow/latest/anyhow/macro.bail.html),
//! [`Context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html), and
//! [`Result`](https://docs.rs/anyhow/latest/anyhow/type.Result.html) items from the
//! [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) crate.
//!
//! If you enable the `structured` feature, you can use the [`Error`](../mabe_derive/derive.Error.html) derive macro to define
//! custom error enums with interpolated messages.
//!
//! # Usage
//!
//! The following example demonstrates how to define a structured error enum and how to chain errors together to produce a
//! detailed traceback.
//!
//! ### Define your Errors
//!
//! ```rust
//! // ./src/error.rs
//!
//! use mabe::Error; // Only available for the "structured" feature.
//!
//! #[derive(Error)]
//! pub enum ServerError {
//!     #[error("You are not authorized to access this resource. Try using a different account.")]
//!     Unauthorized,
//!
//!     // Interpolates the values of the 1st and 2nd field in the error message.
//!     #[error("Network failure\nCode {0}: {1}")]
//!     NetworkFailure(u32, String),
//!
//!     // Interpolates the value of the `cause` and `retry_in` fields in the error message.
//!     #[error("Connection lost --> {cause}. Retry in {retry_in} seconds.")]
//!     ConnectionLost { cause: String, retry_in: u32 }
//! }
//! ```
//!
//! ### Use in Main
//!
//! ```rust
//! // ./src/main.rs
//! # use mabe::Error;
//! #
//! # mod error {
//! #     use super::*;
//! #     #[derive(Error)]
//! #     pub enum ServerError {
//! #         #[error("You are not authorized to access this resource. Try using a different account.")]
//! #         Unauthorized,
//! #         #[error("Network failure\nCode {0}: {1}")]
//! #         NetworkFailure(u32, String),
//! #         #[error("Connection lost --> {cause}. Retry in {retry_in} seconds.")]
//! #         ConnectionLost { cause: String, retry_in: u32 }
//! #     }
//! # }
//!
//! use mabe::{ bail, Context, Result };
//! use crate::error::ServerError;
//!
//! // A dummy function that returns an error if the value is false.
//! fn dummy_function(value: bool) -> Result<()> {
//!     match value {
//!         true => Ok(()),
//!         false => bail!(ServerError::ConnectionLost {
//!             cause: "Server down".to_string(),
//!             retry_in: 10
//!         })
//!     }
//! }
//!
//! #[mabe::main]
//! fn main() -> Result<()> {
//!     // We wrap the errors using `.context()` to create a traceback.
//!     dummy_function(true)
//!         .context("Some other context error.")
//!         // You can also use structured errors as context.
//!         .context(ServerError::NetworkFailure(404, "Not Found".to_string()))
//!         .context("Some context error.")
//!         .context(ServerError::Unauthorized)
//! }
//! ```
//!
//! ### Output
//!
//! When running the code above (with `value` in `dummy_function()` set to `false`), the terminal output will be:
//!
//! ```plaintext
//! [X] You are not authorized to access this resource. Try using a different account.
//!  ├─ Some context error.
//!  ├─ Network failure
//!  │  Code 404: Not Found
//!  ├─ Some other context error.
//!  └─ Connection lost --> Server down. Retry in 10 seconds.
//! ```

#[doc(hidden)]
pub mod internal;

pub use anyhow::{Context, Result, bail};
pub use mabe_attr::main;

#[cfg(feature = "structured")]
pub use mabe_derive::Error;
