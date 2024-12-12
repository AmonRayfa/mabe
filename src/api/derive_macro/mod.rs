// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

mod helpers;
use helpers::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[cfg(debug_assertions)]
use crate::utils::debug::log_macro_output;

/// The helper function that generates the raw implementations (i.e. a [`TokenStream`](proc_macro::TokenStream)) for the `Mabe`
/// derive macro.
pub fn mabe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = &input.ident;

    let mut state_match_arms = Vec::<proc_macro2::TokenStream>::new();
    let mut error_match_arms = Vec::<proc_macro2::TokenStream>::new();
    let mut cause_match_arms = Vec::<proc_macro2::TokenStream>::new();
    let mut debug_match_arms = Vec::<proc_macro2::TokenStream>::new();

    if let Data::Enum(enum_data) = &input.data {
        if enum_data.variants.is_empty() {
            panic!("The `Mabe` derive macro cannot be used on empty enums.");
        }

        // Iterates over all the variants of the enum to generate the appropriate match arms for each of them.
        for variant in &enum_data.variants {
            let variant_ident = &variant.ident;

            let mut state_msg = format!("{}::{}", enum_ident, variant_ident);
            let (error_msg, error_args) = format_msg(get_msg("error", variant));
            let (cause_msg, cause_args) = format_msg(get_msg("cause", variant));
            let (debug_msg, debug_args) = format_msg(get_msg("debug", variant));

            // Generates the match arms for the variant based on the type of fields it contains.
            match &variant.fields {
                Fields::Unit => {
                    let fields = Vec::<String>::new();

                    let (_, error_keyword_args) = map_args(&error_args, &fields, true);
                    let (_, cause_keyword_args) = map_args(&cause_args, &fields, true);
                    let (_, debug_keyword_args) = map_args(&debug_args, &fields, true);

                    state_match_arms.push(quote! {
                        Self::#variant_ident => format!(#state_msg),
                    });
                    error_match_arms.push(quote! {
                        Self::#variant_ident => format!(#error_msg, #(#error_keyword_args),*),
                    });
                    cause_match_arms.push(quote! {
                        Self::#variant_ident => format!(#cause_msg, #(#cause_keyword_args),*),
                    });
                    debug_match_arms.push(quote! {
                        Self::#variant_ident  => format!(#debug_msg, #(#debug_keyword_args),*),
                    });
                }
                Fields::Unnamed(fields) => {
                    let fields = (0..fields.unnamed.iter().len()).map(|i| i.to_string()).collect::<Vec<String>>();
                    state_msg.push('(');

                    for (i, f) in fields.iter().enumerate() {
                        if !error_args.contains(f) && !cause_args.contains(f) && !debug_args.contains(f) {
                            panic!(
                                "The `{}` field of the `{}` variant is not used in the error, cause, or debug message. Make sure to include it in at least one of the messages.",
                                f, variant_ident
                            );
                        }

                        if i == fields.len() - 1 {
                            state_msg.push_str(&format!("{{{}}}", f));
                        } else {
                            state_msg.push_str(&format!("{{{}}}, ", f));
                        }
                    }

                    state_msg.push(')');
                    let (state_msg, state_args) = format_msg(&state_msg);

                    let (state_pattern_bindings, state_keyword_args) = map_args(&state_args, &fields, true);
                    let (error_pattern_bindings, error_keyword_args) = map_args(&error_args, &fields, true);
                    let (cause_pattern_bindings, cause_keyword_args) = map_args(&cause_args, &fields, true);
                    let (debug_pattern_bindings, debug_keyword_args) = map_args(&debug_args, &fields, true);

                    state_match_arms.push(quote! {
                        Self::#variant_ident(#(#state_pattern_bindings),*) => format!(#state_msg, #(#state_keyword_args),*),
                    });
                    error_match_arms.push(quote! {
                        Self::#variant_ident(#(#error_pattern_bindings),*) => format!(#error_msg, #(#error_keyword_args),*),
                    });
                    cause_match_arms.push(quote! {
                        Self::#variant_ident(#(#cause_pattern_bindings),*) => format!(#cause_msg, #(#cause_keyword_args),*),
                    });
                    debug_match_arms.push(quote! {
                        Self::#variant_ident(#(#debug_pattern_bindings),*) => format!(#debug_msg, #(#debug_keyword_args),*),
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
                    state_msg.push_str(" {{ ");

                    for (i, f) in fields.iter().enumerate() {
                        if !error_args.contains(f) && !cause_args.contains(f) && !debug_args.contains(f) {
                            panic!(
                                "The `{}` field of the `{}` variant is not used in the error, cause, or debug message. Make sure to include it in at least one of the messages.",
                                f, variant_ident
                            );
                        }

                        if i == fields.len() - 1 {
                            state_msg.push_str(&format!("{}: {{{}}} ", f, f));
                        } else {
                            state_msg.push_str(&format!("{}: {{{}}}, ", f, f));
                        }
                    }

                    state_msg.push_str("}}");
                    let (state_msg, state_args) = format_msg(&state_msg);

                    let (state_pattern_bindings, state_keyword_args) = map_args(&state_args, &fields, false);
                    let (error_pattern_bindings, error_keyword_args) = map_args(&error_args, &fields, false);
                    let (cause_pattern_bindings, cause_keyword_args) = map_args(&cause_args, &fields, false);
                    let (debug_pattern_bindings, debug_keyword_args) = map_args(&debug_args, &fields, false);

                    state_match_arms.push(quote! {
                        Self::#variant_ident { #(#state_pattern_bindings),* } => format!(#state_msg, #(#state_keyword_args),*),
                    });
                    error_match_arms.push(quote! {
                        Self::#variant_ident { #(#error_pattern_bindings),* } => format!(#error_msg, #(#error_keyword_args),*),
                    });
                    cause_match_arms.push(quote! {
                        Self::#variant_ident { #(#cause_pattern_bindings),* } => format!(#cause_msg, #(#cause_keyword_args),*),
                    });
                    debug_match_arms.push(quote! {
                        Self::#variant_ident { #(#debug_pattern_bindings),* } => format!(#debug_msg, #(#debug_keyword_args),*),
                    });
                }
            }
        }
    } else {
        panic!("The `Mabe` derive macro can only be used with enums.");
    }

    let write_debug = quote! { write!(f, "{}", self.state()) };

    let error_prefix = style_prefix("error");
    let cause_prefix = style_prefix("cause");
    let debug_prefix = style_prefix("debug");

    let write_display = quote! {
        let mut error = match self.error().as_str() {
            "" => "".to_string(),
            e => format!("\n{} {}", #error_prefix, e),
        };
        let mut cause = match self.cause().as_str() {
            "" => "".to_string(),
            r => format!("\n{} {}", #cause_prefix, r),
        };
        let mut debug = match self.debug().as_str() {
            "" => "".to_string(),
            s => format!("\n{} {}", #debug_prefix, s),
        };

        write!(f, "{}{}{}", error, cause, debug)
    };

    let implementations = quote! {
        impl #enum_ident {
            pub fn state(&self) -> String { match self { #(#state_match_arms)* } }

            pub fn error(&self) -> String { match self { #(#error_match_arms)* } }

            pub fn cause(&self) -> String { match self { #(#cause_match_arms)* } }

            pub fn debug(&self) -> String { match self { #(#debug_match_arms)* } }
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

    TokenStream::from(implementations)
}
