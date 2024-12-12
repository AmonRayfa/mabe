// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! [**Mabe**](https://github.com/AmonRayfa/mabe) is a simple framework for creating debug-friendly error enums in Rust. Each
//! variant in the enum can include an error, cause, and debug message, and errors are displayed in a structured format,
//! showing the messages defined for the variant. This allows for a more detailed and clear debugging process.
//!
//! Functionally, this crate is a _procedural macro_ that provides a derive macro called
//! [`Mabe`](https://docs.rs/mabe/0.3.1/mabe/derive.Mabe.html), which is used to generate the debug-friendly error enums.
//!
//! # Examples
//!
//! Here is a simple example of how to create a debug-friendly error enum:
//!
//! ```
//! use mabe::Mabe;
//!
//! #[derive(Mabe)]
//! pub enum ServerError {
//!     #[error("You are not authorized to access this resource.")]
//!     #[cause("Your account does not have the required permissions.")]
//!     #[debug("Try using a different account.")]
//!     Unauthorized,
//! }
//!
//! let error = ServerError::Unauthorized;
//! println!("{}", error);
//! ```
//!
//! ```text
//! Output:
//! [error] You are not authorized to access this resource.
//! [cause] Your account does not have the required permissions.
//! [debug] Try using a different account.
//! ```
//!
//! You can also interpolate the values of variant fields in the error, cause, and debug messages as shown below:
//!
//! ```
//! use mabe::Mabe;
//!
//! #[derive(Mabe)]
//! pub enum ServerError {
//!     #[error("Network failure.")]
//!     // Interpolates the values of the 1st and 2nd field in the cause message.
//!     #[cause("Code {0}: {1}.")]
//!     NetworkFailure(u32, String),
//!
//!     #[error("Connection lost.")]
//!     // Interpolates the value of the `cause` field in the cause message.
//!     #[cause("{cause}")]
//!     // Interpolates the value of the `retry_in` field in the debug message.
//!     #[debug("Retry in {retry_in} seconds.")]
//!     ConnectionLost { cause: String, retry_in: u32 }
//! }
//!
//! let error1 = ServerError::NetworkFailure(404, "Not Found".to_string());
//! println!("{}", error1);
//!
//! let error2 = ServerError::ConnectionLost { cause: "Server down".to_string(), retry_in: 10 };
//! println!("{}", error2);
//! ```
//!
//! ```text
//! Output:
//! [error] Network failure.
//! [cause] Code 404: Not Found.
//!
//! [error] Connection lost.
//! [cause] Server down.
//! [debug] Retry in 10 seconds.
//! ```
//!
//! # Cargo Features
//!
//! The following is a list of
//! [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/features.html#the-features-section) that can be enabled or
//! disabled in the `Cargo.toml` file:
//!
//! * **colorize**: Adds colors to the prefixes of the error, cause, and debug messages (i.e. to `[error]`, `[cause]`, and
//!   `[debug]`) when they are printed. This feature only works with ANSI-compatible terminals.

extern crate proc_macro;
mod api;
mod error;
use api::mabe;

#[cfg(debug_assertions)]
mod utils;

/// The derive macro that generates debug-friendly error enums by providing an `error`, `cause`, and `debug` attribute for each
/// variant of the enum, which can be used to define the error, cause, and debug messages respectively. The macro also provides
/// support for the [`Debug`](std::fmt::Debug), [`Display`](std::fmt::Display), and [`Error`](std::error::Error) traits.
#[proc_macro_derive(Mabe, attributes(error, cause, debug))]
pub fn mabe_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mabe(input)
}
