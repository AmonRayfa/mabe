// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

mod api;
mod error;
use api::mabe;
use proc_macro::TokenStream;

#[cfg(debug_assertions)]
mod utils;

/// The derive macro that creates the debug-friendly error enums. It provides an `error` attribute which can be used to define
/// the error message each variant of the enum. The macro also automatically generates implementations for the
/// [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html),
/// [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html), and
/// [`Error`](https://doc.rust-lang.org/std/error/trait.Error.html) traits. Note that the macro also generates public `debug()`
/// and `error()` methods on the enum, so these method names are reserved and cannot be defined manually on the same enum.
#[proc_macro_derive(Error, attributes(error))]
pub fn mabe_derive_macro(input: TokenStream) -> TokenStream {
    mabe(input)
}
