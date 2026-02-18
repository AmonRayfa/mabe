// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// The attribute macro used on the entry point of your app to format error chains into a debug-friendly tree structure in the
/// terminal.
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);

    // Renames the user's `main` function.
    let original_name = input_fn.sig.ident;
    let wrapped_name = syn::Ident::new(&format!("__mabe_{}", original_name), original_name.span());
    input_fn.sig.ident = wrapped_name.clone();

    let expanded = quote! {
        #input_fn

        fn main() {
            if let Err(e) = #wrapped_name() {
                // HELPER: Cleans text by trimming whitespaces.
                let clean = |s: String| -> String {
                    s.trim().to_string()
                };

                // --- PRINT ROOT ERROR ---
                let root_msg = clean(e.to_string());
                let root_lines = ::mabe::internal::wrap(root_msg);

                for (i, line) in root_lines.iter().enumerate() {
                    if i == 0 { // First line gets the "[X]" prefix.
                        eprintln!("{} {}", ::mabe::internal::X, line);
                    } else { // Subsequent lines get 4 spaces indentation to align with "[X] ".
                        eprintln!("    {}", line);
                    }
                }

                // --- PRINT CAUSES ---
                let causes: Vec<_> = e.chain().skip(1).collect();
                let last_idx = causes.len().saturating_sub(1);

                for (i, cause) in causes.iter().enumerate() {
                    let is_last = i == last_idx;

                    // Defines the prefixes.
                    let branch = if is_last { ::mabe::internal::L_BRANCH } else { ::mabe::internal::T_BRANCH };
                    let pipe   = if is_last { "    " } else { ::mabe::internal::PIPE };

                    // Cleans and wrap the messages.
                    let raw_msg = clean(cause.to_string());
                    let lines = ::mabe::internal::wrap(raw_msg);

                    for (j, line) in lines.iter().enumerate() {
                        if j == 0 { // First line gets the branch symbol.
                            eprintln!("{}{}", branch, line);
                        } else { // Subsequent lines get the pipe symbol.
                            eprintln!("{}{}", pipe, line);
                        }
                    }
                }

                // --- EXIT ---
                std::process::exit(1);
            }
        }
    };

    TokenStream::from(expanded)
}
