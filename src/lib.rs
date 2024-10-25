// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! The `mabe` crate provides a procedural macro that generates code for enums annotated with the `Mabe` derive macro.
//!
//! # Example
//!

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Lit, Meta, NestedMeta};

/// Procedural macro that generates code for enums annotated with the `Mabe` derive macro.
/// This macro processes the input enum and generates methods to retrieve error, reason, and solution messages for each variant
/// along with implementations for the [`Display`](std::fmt::Display) and [`Error`](std::error::Error) traits.
#[proc_macro_derive(Mabe, attributes(error, reason, solution))]
pub fn mabe_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = &input.ident;

    let mut error_match_arms = Vec::new();
    let mut reason_match_arms = Vec::new();
    let mut solution_match_arms = Vec::new();

    if let Data::Enum(enum_data) = &input.data {
        // Iterates over all the variants of the enum to generate the appropriate match arms for each of them.
        for variant in &enum_data.variants {
            let variant_ident = &variant.ident;
            let mut error_msg = String::new();
            let mut reason_msg = String::new();
            let mut solution_msg = String::new();

            // Iterates over all the attributes of a variant to extract the error, reason, and solution messages.
            for attr in &variant.attrs {
                if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                    if let Some(attr_ident) = meta_list.path.get_ident() {
                        match attr_ident.to_string().as_str() {
                            "error" => {
                                if meta_list.nested.len() != 1 {
                                    panic!("The `#[error]` attribute must contain exactly one argument.");
                                }
                                if let NestedMeta::Lit(Lit::Str(lit_str)) = &meta_list.nested[0] {
                                    error_msg = lit_str.value();
                                } else {
                                    panic!("The `#[error]` attribute must contain a single string literal.");
                                }
                            }
                            "reason" => {
                                if meta_list.nested.len() != 1 {
                                    panic!("The `#[reason]` attribute must contain exactly one argument.");
                                }
                                if let NestedMeta::Lit(Lit::Str(lit_str)) = &meta_list.nested[0] {
                                    reason_msg = lit_str.value();
                                } else {
                                    panic!("The `#[reason]` attribute must contain a single string literal.");
                                }
                            }
                            "solution" => {
                                if meta_list.nested.len() != 1 {
                                    panic!("The `#[solution]` attribute must contain exactly one argument.");
                                }
                                if let NestedMeta::Lit(Lit::Str(lit_str)) = &meta_list.nested[0] {
                                    solution_msg = lit_str.value();
                                } else {
                                    panic!("The `#[solution]` attribute must contain a single string literal.");
                                }
                            }
                            _ => {}
                        }
                    }
                } else {
                    panic!("The `#[error]`, `#[reason]`, and `#[solution]` attributes must contain a single string literal.");
                }
            }

            // Generates match arms for each variant.
            error_match_arms.push(quote! {
                Self::#variant_ident => #error_msg,
            });
            reason_match_arms.push(quote! {
                Self::#variant_ident => #reason_msg,
            });
            solution_match_arms.push(quote! {
                Self::#variant_ident => #solution_msg,
            });
        }

        // Generates a default match arm in case the enum has no variants.
        error_match_arms.push(quote! { _ => String::new() });
        reason_match_arms.push(quote! { _ => String::new() });
        solution_match_arms.push(quote! { _ => String::new() });
    } else {
        panic!("The `Mabe` derive macro can only be used with enums.");
    }

    TokenStream::from(quote! {
        impl #enum_ident {
            fn error(&self) -> String { match self { #(#error_match_arms),* } }

            fn reason(&self) -> String { match self { #(#reason_match_arms),* } }

            fn solution(&self) -> String { match self { #(#solution_match_arms),* } }
        }

        impl std::fmt::Display for #enum_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[error] {}\n[reason] {}\n[solution] {}", self.error(), self.reason(), self.solution())
            }
        }

        impl std::error::Error for #enum_ident {}
    })
}
