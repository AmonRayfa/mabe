// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! The `mabe` crate provides a procedural macro that generates code for enums annotated with the `Mabe` derive macro.
//!
//! # Example
//!

extern crate proc_macro;
mod utils;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta, NestedMeta};
use utils::*;

/// Procedural macro that generates code for enums annotated with the `Mabe` derive macro.
/// This macro processes the input enum and generates methods to retrieve error, reason, and solution messages for each variant
/// along with an implementation for the [`Display`](std::fmt::Display) and [`Error`](std::error::Error) traits.
#[proc_macro_derive(Mabe, attributes(error, reason, solution))]
pub fn mabe_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = &input.ident;

    let mut error_match_arms = Vec::new();
    let mut reason_match_arms = Vec::new();
    let mut solution_match_arms = Vec::new();

    if let Data::Enum(enum_data) = &input.data {
        if enum_data.variants.is_empty() {
            panic!("The `Mabe` derive macro cannot be used on empty enums.");
        }

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
                                    panic!(
                                        "Expected 1 argument for the `#[error]` attribute of the `{}` variant, found {}.",
                                        variant_ident,
                                        meta_list.nested.len()
                                    );
                                }
                                if let NestedMeta::Lit(Lit::Str(lit_str)) = &meta_list.nested[0] {
                                    error_msg = lit_str.value();
                                } else {
                                    panic!(
                                        "Expected a `&str` argument for the `#[error]` attribute of the `{}` variant.",
                                        variant_ident
                                    );
                                }
                            }
                            "reason" => {
                                if meta_list.nested.len() != 1 {
                                    panic!(
                                        "Expected 1 argument for the `#[reason]` attribute of the `{}` variant, found {}.",
                                        variant_ident,
                                        meta_list.nested.len()
                                    );
                                }
                                if let NestedMeta::Lit(Lit::Str(lit_str)) = &meta_list.nested[0] {
                                    reason_msg = lit_str.value();
                                } else {
                                    panic!(
                                        "Expected a `&str` argument for the `#[reason]` attribute of the `{}` variant.",
                                        variant_ident
                                    );
                                }
                            }
                            "solution" => {
                                if meta_list.nested.len() != 1 {
                                    panic!(
                                        "Expected 1 argument for the `#[solution]` attribute of the `{}` variant, found {}.",
                                        variant_ident,
                                        meta_list.nested.len()
                                    );
                                }
                                if let NestedMeta::Lit(Lit::Str(lit_str)) = &meta_list.nested[0] {
                                    solution_msg = lit_str.value();
                                } else {
                                    panic!(
                                        "Expected a `&str` argument for the `#[solution]` attribute of the `{}` variant.",
                                        variant_ident
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                } else {
                    panic!(
                        "Failed to parse the attributes of the `{}` variant, make sure the attributes are correctly formatted.",
                        variant_ident
                    );
                }
            }

            let (error_msg, error_args) = generic_format(error_msg);
            let (reason_msg, reason_args) = generic_format(reason_msg);
            let (solution_msg, solution_args) = generic_format(solution_msg);

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

    let implementations = quote! {
        impl #enum_ident {
            pub fn error(&self) -> String { match self { #(#error_match_arms)* } }

            pub fn reason(&self) -> String { match self { #(#reason_match_arms)* } }

            pub fn solution(&self) -> String { match self { #(#solution_match_arms)* } }
        }

        impl std::fmt::Display for #enum_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[error] {}\n[reason] {}\n[solution] {}", self.error(), self.reason(), self.solution())
            }
        }

        impl std::error::Error for #enum_ident {}
    };

    #[cfg(debug_assertions)]
    write_implementations(&implementations, "./logs/generated_implementations.txt");

    TokenStream::from(implementations)
}
