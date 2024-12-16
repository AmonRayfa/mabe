// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! [**Mabe**](https://github.com/AmonRayfa/mabe) is a simple framework for creating debug-friendly error enums in Rust. Each
//! variant in the enum can encapsulate an error and a debug message, and errors are presented in a structured format,
//! displaying the messages defined for the variant. This allows for a more detailed and clear debugging process.
//!
//! Functionally, this crate is a _procedural macro_ that provides a derive macro called
//! [`Mabe`](https://docs.rs/mabe/latest/mabe/derive.Mabe.html), which is used to generate the debug-friendly error enums.
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
//! [debug] Try using a different account.
//! ```
//!
//! You can also interpolate the values of variant fields in the error and debug messages as shown below:
//!
//! ```
//! use mabe::Mabe;
//!
//! #[derive(Mabe)]
//! pub enum ServerError {
//!     // Interpolates the values of the 1st and 2nd field in the error message.
//!     #[error("Network failure. --> Code: {0}: {1}.")]
//!     NetworkFailure(u32, String),
//!
//!     // Interpolates the value of the `cause` field in the error message.
//!     #[error("Connection lost. --> {cause}.")]
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
//! [error] Network failure. --> Code: 404: Not Found.
//!
//! [error] Connection lost --> Server down.
//! [debug] Retry in 10 seconds.
//! ```
//!
//! # Cargo Features
//!
//! The following is a list of
//! [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/features.html#the-features-section) that can be enabled or
//! disabled in the `Cargo.toml` file:
//!
//! * **colorize**: Adds colors to the prefixes of the error and debug messages (i.e. to `[error]` and `[debug]`) when they are
//!   printed. This feature only works with ANSI-compatible terminals.

extern crate proc_macro;
mod api;
mod error;
use api::mabe;

#[cfg(debug_assertions)]
mod utils;

/// The derive macro that creates the debug-friendly error enums. It provides an `error` and a `debug` attribute for each
/// variant of the enum, which can be used to define the error and debug messages respectively. The macro also automatically
/// generates implementations for the [`Debug`](std::fmt::Debug), [`Display`](std::fmt::Display), and
/// [`Error`](std::error::Error) traits.
#[proc_macro_derive(Mabe, attributes(error, debug))]
pub fn mabe_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mabe(input)
}
