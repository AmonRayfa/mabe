// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! [**Mabe**](https://github.com/AmonRayfa/mabe) is a *procedural macro crate* that provides tools for creating simple and
//! well-structured error enums for easy debugging. Each variant in the enum can have an error, reason, and solution message.
//! This allows for a more detailed error handling and debugging process. Also, when an error is printed, the error, reason,
//! and solution messages are displayed in a structured and easy-to-read format.
//!
//! # Examples
//!
//! Simple example of using the [`Mabe`](https://docs.rs/mabe/latest/mabe/derive.Mabe.html) derive macro:
//!
//! ```
//! // Imports the `Colorize` trait only when the `colored` feature is enabled.
//! #[cfg(feature = "colored")] // Remove this line when using the crate in a real project.
//! use colored::Colorize;
//!
//! use mabe::Mabe;
//!
//! #[derive(Debug, Mabe)]
//! pub enum ServerError {
//!     #[error("You are not authorized to access this resource.")]
//!     #[reason("Your account does not have the required permissions.")]
//!     #[solution("Try using a different account.")]
//!     Unauthorized,
//! }
//!
//! fn main() {
//!     let error = ServerError::Unauthorized;
//!     println!("{}", error);
//! }
//! ```
//!
//! ```text
//! Output:
//! [error] You are not authorized to access this resource.
//! [reason] Your account does not have the required permissions.
//! [solution] Try using a different account.
//! ```
//!
//! You can also interpolate the values of variant fields in the error, reason, and solution messages as shown below:
//!
//! ```
//! // Imports the `Colorize` trait only when the `colored` feature is enabled.
//! #[cfg(feature = "colored")] // Remove this line when using the crate in a real project.
//! use colored::Colorize;
//!
//! use mabe::Mabe;
//!
//! #[derive(Debug, Mabe)]
//! pub enum ServerError {
//!     #[error("Network failure.")]
//!     // Interpolates the values of the 1st and 2nd field in the reason message.
//!     #[reason("Code {0}: {1}.")]
//!     NetworkFailure(u32, String),
//!
//!     #[error("Connection lost.")]
//!     // Interpolates the value of the `reason` field in the reason message.
//!     #[reason("{reason}")]
//!     // Interpolates the value of the `retry_in` field in the solution message.
//!     #[solution("Retry in {retry_in} seconds.")]
//!     ConnectionLost { reason: String, retry_in: u32 }
//! }
//!
//! fn main() {
//!     let error1 = ServerError::NetworkFailure(404, "Not Found".to_string());
//!     println!("{}", error1);
//!
//!     let error2 = ServerError::ConnectionLost { reason: "Server down".to_string(), retry_in: 10 };
//!     println!("{}", error2);
//! }
//! ```
//!
//! ```text
//! Output:
//! [error] Network failure.
//! [reason] Code 404: Not Found.
//! [solution]
//!
//! [error] Connection lost.
//! [reason] Server down.
//! [solution] Retry in 10 seconds.
//! ```
//!
//! # Features
//!
//! * **colored**: Adds colors to the prefixes of the error, reason, and solution messages (i.e. to `[error]`, `[reason]`, and `[solution]`) when they are printed. In order to use this feature, the [colored](https://crates.io/crates/colored) crate must be included in the dependencies of the project and the [`Colorize`](https://docs.rs/colored/latest/colored/trait.Colorize.html) trait must be imported where the error enums are defined.

extern crate proc_macro;
mod utils;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use utils::*;

/// Allows for the creation of simple and well-structured error enums for easy debugging by providing error, reason,
/// and solution attributes for each variant, and generating methods to retrieve the messages of these attributes.
/// Additionally, it generates an implementation for the [`Display`](std::fmt::Display) and [`Error`](std::error::Error) traits.
#[proc_macro_derive(Mabe, attributes(error, reason, solution))]
pub fn mabe_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut error_match_arms = Vec::<proc_macro2::TokenStream>::new();
    let mut reason_match_arms = Vec::<proc_macro2::TokenStream>::new();
    let mut solution_match_arms = Vec::<proc_macro2::TokenStream>::new();

    if let Data::Enum(enum_data) = &input.data {
        if enum_data.variants.is_empty() {
            panic!("The `Mabe` derive macro cannot be used on empty enums.");
        }

        // Iterates over all the variants of the enum to generate the appropriate match arms for each of them.
        for variant in &enum_data.variants {
            let variant_ident = &variant.ident;

            let (error_msg, error_args) = generic_format(get_attr_msg("error", variant));
            let (reason_msg, reason_args) = generic_format(get_attr_msg("reason", variant));
            let (solution_msg, solution_args) = generic_format(get_attr_msg("solution", variant));

            // Generates the match arms for the variant based on the type of fields it contains.
            match &variant.fields {
                Fields::Unit => {
                    let fields = Vec::<String>::new();
                    let (_, error_keyword_args) = pattern_map(&error_args, &fields, true);
                    let (_, reason_keyword_args) = pattern_map(&reason_args, &fields, true);
                    let (_, solution_keyword_args) = pattern_map(&solution_args, &fields, true);

                    error_match_arms.push(quote! {
                        Self::#variant_ident => format!(#error_msg, #(#error_keyword_args),*),
                    });
                    reason_match_arms.push(quote! {
                        Self::#variant_ident => format!(#reason_msg, #(#reason_keyword_args),*),
                    });
                    solution_match_arms.push(quote! {
                        Self::#variant_ident  => format!(#solution_msg, #(#solution_keyword_args),*),
                    });
                }
                Fields::Unnamed(fields) => {
                    let fields = (0..fields.unnamed.iter().len()).map(|i| i.to_string()).collect::<Vec<String>>();

                    for f in fields.iter() {
                        if !error_args.contains(f) && !reason_args.contains(f) && !solution_args.contains(f) {
                            panic!(
                            "The `{}` field of the `{}` variant is not used in the error, reason, or solution message. Make sure to include it in at least one of the messages.",
                            f, variant_ident
                        );
                        }
                    }

                    let (error_pattern_bindings, error_keyword_args) = pattern_map(&error_args, &fields, true);
                    let (reason_pattern_bindings, reason_keyword_args) = pattern_map(&reason_args, &fields, true);
                    let (solution_pattern_bindings, solution_keyword_args) = pattern_map(&solution_args, &fields, true);

                    error_match_arms.push(quote! {
                        Self::#variant_ident(#(#error_pattern_bindings),*) => format!(#error_msg, #(#error_keyword_args),*),
                    });
                    reason_match_arms.push(quote! {
                        Self::#variant_ident(#(#reason_pattern_bindings),*) => format!(#reason_msg, #(#reason_keyword_args),*),
                    });
                    solution_match_arms.push(quote! {
                        Self::#variant_ident(#(#solution_pattern_bindings),*) => format!(#solution_msg, #(#solution_keyword_args),*),
                    });
                }
                Fields::Named(fields) => {
                    let fields = fields.named
                    .iter()
                    .map(|f| {
                        f.ident
                            .clone()
                            .unwrap_or_else(|| panic!("Failed to retrieve the identifier of a named field in the `{}` variant. This error should not be possible, try reloading the window. If the problem persists, report the issue to the crate's [GitHub repository](https://github.com/AmonRayfa/mabe).", variant_ident))
                            .to_string()
                    })
                    .collect::<Vec<String>>();

                    for f in fields.iter() {
                        if !error_args.contains(f) && !reason_args.contains(f) && !solution_args.contains(f) {
                            panic!(
                            "The `{}` field of the `{}` variant is not used in the error, reason, or solution message. Make sure to include it in at least one of the messages.",
                            f, variant_ident
                        );
                        }
                    }

                    let (error_pattern_bindings, error_keyword_args) = pattern_map(&error_args, &fields, false);
                    let (reason_pattern_bindings, reason_keyword_args) = pattern_map(&reason_args, &fields, false);
                    let (solution_pattern_bindings, solution_keyword_args) = pattern_map(&solution_args, &fields, false);

                    error_match_arms.push(quote! {
                        Self::#variant_ident { #(#error_pattern_bindings),* } => format!(#error_msg, #(#error_keyword_args),*),
                    });
                    reason_match_arms.push(quote! {
                        Self::#variant_ident { #(#reason_pattern_bindings),* } => format!(#reason_msg, #(#reason_keyword_args),*),
                    });
                    solution_match_arms.push(quote! {
                        Self::#variant_ident { #(#solution_pattern_bindings),* } => format!(#solution_msg, #(#solution_keyword_args),*),
                    });
                }
            }
        }
    } else {
        panic!("The `Mabe` derive macro can only be used with enums.");
    }

    let enum_ident = &input.ident;

    #[cfg(feature = "colored")]
    let write_messages = quote! { write!(f, "\n{} {}\n{} {}\n{} {}", "[error]".red().bold(), self.error(), "[reason]".yellow().bold(), self.reason(), "[solution]".green().bold(), self.solution()) };

    #[cfg(not(feature = "colored"))]
    let write_messages =
        quote! { write!(f, "\n[error] {}\n[reason] {}\n[solution] {}", self.error(), self.reason(), self.solution()) };

    let implementations = quote! {
        impl #enum_ident {
            pub fn error(&self) -> String { match self { #(#error_match_arms)* } }

            pub fn reason(&self) -> String { match self { #(#reason_match_arms)* } }

            pub fn solution(&self) -> String { match self { #(#solution_match_arms)* } }
        }

        impl std::fmt::Display for #enum_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #write_messages
            }
        }

        impl std::error::Error for #enum_ident {}
    };

    #[cfg(debug_assertions)]
    write_implementations(&implementations, "./logs/generated_implementations.txt");

    TokenStream::from(implementations)
}
