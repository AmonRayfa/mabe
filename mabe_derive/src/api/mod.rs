// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

mod helpers;
use crate::error::api::Error;
use helpers::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[cfg(debug_assertions)]
use crate::utils::debug::log_macro_output;

/// The core function that generates the raw implementations (i.e. a
/// [`TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html)) for the
/// [`Error`](https://mabe.readthedocs.io/en/stable/mabe_derive/derive.Error.html) derive macro. Invalid inputs are reported as
/// spanned compile errors instead of panics, so they read like native compiler diagnostics and point at the offending item.
pub(super) fn mabe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand(&input) {
        Ok(implementations) => TokenStream::from(implementations),
        Err(error) => TokenStream::from(error.to_compile_error()),
    }
}

/// The fallible expansion logic behind the [`mabe`] entry point.
fn expand(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let enum_ident = &input.ident;

    let mut debug_match_arms = Vec::<proc_macro2::TokenStream>::new();
    let mut error_match_arms = Vec::<proc_macro2::TokenStream>::new();

    if let Data::Enum(enum_data) = &input.data {
        if enum_data.variants.is_empty() {
            return Err(syn::Error::new_spanned(enum_ident, Error::EmptyEnum));
        }

        // Iterates over all the variants of the enum to generate the appropriate match arms for each of them.
        for variant in &enum_data.variants {
            let variant_ident = &variant.ident;

            let mut debug_msg = format!("{}::{}", enum_ident, variant_ident);
            let (error_msg, error_args) = format_msg(get_msg("error", variant)?);

            // Generates the match arms for the variant based on the type of fields it contains.
            match &variant.fields {
                Fields::Unit => {
                    let fields = Vec::<String>::new();

                    let (_, error_keyword_args) = map_args(&error_args, &fields, true);

                    debug_match_arms.push(quote! {
                        Self::#variant_ident => format!(#debug_msg),
                    });
                    error_match_arms.push(quote! {
                        Self::#variant_ident => format!(#error_msg, #(#error_keyword_args),*),
                    });
                }
                Fields::Unnamed(fields) => {
                    let fields = (0..fields.unnamed.iter().len()).map(|i| i.to_string()).collect::<Vec<String>>();
                    debug_msg.push('(');

                    for (i, f) in fields.iter().enumerate() {
                        if !error_args.contains(f) {
                            return Err(syn::Error::new_spanned(variant_ident, Error::UnusedVariantField(variant_ident, f)));
                        }

                        if i == fields.len() - 1 {
                            debug_msg.push_str(&format!("{{{}}}", f));
                        } else {
                            debug_msg.push_str(&format!("{{{}}}, ", f));
                        }
                    }

                    debug_msg.push(')');
                    let (debug_msg, debug_args) = format_msg(&debug_msg);

                    let (debug_pattern_bindings, debug_keyword_args) = map_args(&debug_args, &fields, true);
                    let (error_pattern_bindings, error_keyword_args) = map_args(&error_args, &fields, true);

                    debug_match_arms.push(quote! {
                        Self::#variant_ident(#(#debug_pattern_bindings),*) => format!(#debug_msg, #(#debug_keyword_args),*),
                    });
                    error_match_arms.push(quote! {
                        Self::#variant_ident(#(#error_pattern_bindings),*) => format!(#error_msg, #(#error_keyword_args),*),
                    });
                }
                Fields::Named(named_fields) => {
                    let mut fields = Vec::<String>::new();

                    for f in &named_fields.named {
                        match &f.ident {
                            Some(field_ident) => fields.push(field_ident.to_string()),
                            None => {
                                return Err(syn::Error::new_spanned(variant_ident, Error::IdentRetrievalFailed(variant_ident)));
                            }
                        }
                    }

                    debug_msg.push_str(" {{ ");

                    for (i, f) in fields.iter().enumerate() {
                        if !error_args.contains(f) {
                            return Err(syn::Error::new_spanned(variant_ident, Error::UnusedVariantField(variant_ident, f)));
                        }

                        if i == fields.len() - 1 {
                            debug_msg.push_str(&format!("{}: {{{}}} ", f, f));
                        } else {
                            debug_msg.push_str(&format!("{}: {{{}}}, ", f, f));
                        }
                    }

                    debug_msg.push_str("}}");
                    let (debug_msg, debug_args) = format_msg(&debug_msg);

                    let (debug_pattern_bindings, debug_keyword_args) = map_args(&debug_args, &fields, false);
                    let (error_pattern_bindings, error_keyword_args) = map_args(&error_args, &fields, false);

                    debug_match_arms.push(quote! {
                        Self::#variant_ident { #(#debug_pattern_bindings),* } => format!(#debug_msg, #(#debug_keyword_args),*),
                    });
                    error_match_arms.push(quote! {
                        Self::#variant_ident { #(#error_pattern_bindings),* } => format!(#error_msg, #(#error_keyword_args),*),
                    });
                }
            }
        }
    } else {
        return Err(syn::Error::new_spanned(enum_ident, Error::NotAnEnum));
    }

    let write_debug = quote! { write!(f, "{}", self.debug()) };
    let write_display = quote! { write!(f, "{}", self.error()) };

    let implementations = quote! {
        impl #enum_ident {
            pub fn debug(&self) -> String { match self { #(#debug_match_arms)* } }

            pub fn error(&self) -> String { match self { #(#error_match_arms)* } }
        }

        impl std::fmt::Debug for #enum_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #write_debug
            }
        }

        impl std::fmt::Display for #enum_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #write_display
            }
        }

        impl std::error::Error for #enum_ident {}
    };

    #[cfg(debug_assertions)]
    log_macro_output(&implementations, "./mabe_output.log");

    Ok(implementations)
}
