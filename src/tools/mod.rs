// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

mod helpers;
use helpers::*;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

/// A tool that returns a tuple containing the formatted message and the extracted arguments.
/// In the formatted message, all single curly braces (i.e. curly braces that are not part of a placeholder),
/// and all placeholders are replaced with generic placeholders (e.g. {placeholder0}, {placeholder1}, etc.).
/// The replaced elements are stored in the extracted arguments vector.
pub fn generic_format(msg: String) -> (String, Vec<String>) {
    let mut formatted_msg = String::new();
    let mut extracted_args = Vec::new();
    let mut placeholder_position = 0;
    let mut checkpoint = 0;

    while checkpoint < msg.len() {
        let index_left_brace = active_left_brace(&msg, checkpoint);
        let index_right_brace = active_right_brace(&msg, checkpoint);
        let generic_placeholder = format!("{{placeholder{}}}", placeholder_position);

        if index_left_brace.is_some() && index_right_brace.is_some() {
            formatted_msg.push_str(&msg[checkpoint..index_left_brace.unwrap()]);
            formatted_msg.push_str(generic_placeholder.as_str());
            extracted_args.push(msg[index_left_brace.unwrap() + 1..index_right_brace.unwrap()].to_string());
            placeholder_position += 1;
            checkpoint = index_right_brace.unwrap() + 1;
        } else if index_left_brace.is_some() && index_right_brace.is_none() {
            formatted_msg.push_str(&msg[checkpoint..index_left_brace.unwrap()]);
            formatted_msg.push_str(generic_placeholder.as_str());
            extracted_args.push("{".to_string());
            placeholder_position += 1;
            checkpoint = index_left_brace.unwrap() + 1;
        } else if index_left_brace.is_none() && index_right_brace.is_some() {
            formatted_msg.push_str(&msg[checkpoint..index_right_brace.unwrap()]);
            formatted_msg.push_str(generic_placeholder.as_str());
            extracted_args.push("}".to_string());
            placeholder_position += 1;
            checkpoint = index_right_brace.unwrap() + 1;
        } else {
            formatted_msg.push_str(&msg[checkpoint..]);
            checkpoint = msg.len();
        }
    }

    (formatted_msg, extracted_args)
}

/// A tool that returns a tuple containing the pattern bindings and the keyword arguments.
/// The pattern bindings are the fields of an enum variant and the keyword arguments are the placeholders and the extracted
/// arguments.
pub fn pattern_map(args: &[String], fields: &[String], dunder: bool) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let pattern_bindings = fields
        .iter()
        .map(|f| {
            let pattern = match dunder {
                true => Ident::new(format!("_{}_", f).as_str(), Span::call_site()),
                false => Ident::new(f.to_string().as_str(), Span::call_site()),
            };
            quote! { #pattern }
        })
        .collect::<Vec<TokenStream>>();

    let keyword_args = args
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            let keyword = Ident::new(format!("placeholder{}", i).as_str(), Span::call_site());
            match find_index(fields, arg) {
                Some(index) => {
                    let pattern = &pattern_bindings[index];
                    quote! { #keyword = #pattern }
                }
                None => quote! { #keyword = #arg },
            }
        })
        .collect::<Vec<TokenStream>>();

    (pattern_bindings, keyword_args)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_format() {
        // Example 1: Even number of curly braces.
        let msg1 = "The placeholders are: {x}, {{y}} and {z}.".to_string();
        let (formatted_msg1, extracted_args_msg1) = generic_format(msg1);
        assert_eq!(formatted_msg1, "The placeholders are: {placeholder0}, {{y}} and {placeholder1}.");
        assert_eq!(extracted_args_msg1, vec!["x".to_string(), "z".to_string()]);

        // Example 2: Uneven number of curly braces.
        let msg2 = "The placeholders are: {x}, {{{y}}} and {z}}}.".to_string();
        let (formatted_msg2, extracted_args_msg2) = generic_format(msg2);
        assert_eq!(formatted_msg2, "The placeholders are: {placeholder0}, {{{placeholder1}}} and {placeholder2}}}.");
        assert_eq!(extracted_args_msg2, vec!["x".to_string(), "y".to_string(), "z".to_string()]);

        // Example 3: Even number of curly braces.
        let msg3 = "The placeholders are: {{x}}, {{{{y}}}} and {{{{z}}.".to_string();
        let (formatted_msg3, extracted_args_msg3) = generic_format(msg3);
        assert_eq!(formatted_msg3, "The placeholders are: {{x}}, {{{{y}}}} and {{{{z}}.");
        assert_eq!(extracted_args_msg3, Vec::<String>::new());

        // Example 4: Empty curly braces.
        let msg4 = "The placeholders are: {}, {{}}}} and {{{ }.".to_string();
        let (formatted_msg4, extracted_args_msg4) = generic_format(msg4);
        assert_eq!(formatted_msg4, "The placeholders are: {placeholder0}, {{}}}} and {{{placeholder1}.");
        assert_eq!(extracted_args_msg4, vec!["".to_string(), " ".to_string()]);

        // Example 5: Unbalanced curly braces.
        let msg5 = "The placeholders are: {x}}, {y} and z.".to_string();
        let (formatted_msg5, extracted_args_msg5) = generic_format(msg5);
        assert_eq!(formatted_msg5, "The placeholders are: {placeholder0} and z.");
        assert_eq!(extracted_args_msg5, vec!["x}}, {y".to_string()]);
    }

    #[test]
    fn test_pattern_map() {
        // Example 1: Empty `args` and `fields` vector.
        let args1 = Vec::<String>::new();
        let fields1 = Vec::<String>::new();
        let (pattern_bindings1, keyword_args1) = pattern_map(&args1, &fields1, true);
        assert_eq!(quote! {[#(#pattern_bindings1),*]}.to_string(), "[]");
        assert_eq!(quote! {[#(#keyword_args1),*]}.to_string(), "[]");

        // Example 2: Empty `args` vector.
        let args2 = Vec::<String>::new();
        let fields2 = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        let (pattern_bindings2, keyword_args2) = pattern_map(&args2, &fields2, true);
        assert_eq!(quote! {[#(#pattern_bindings2),*]}.to_string(), "[_x_ , _y_ , _z_]");
        assert_eq!(quote! {[#(#keyword_args2),*]}.to_string(), "[]");

        // Example 3: Empty `fields` vector.
        let args3 = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        let fields3 = Vec::<String>::new();
        let (pattern_bindings3, keyword_args3) = pattern_map(&args3, &fields3, true);
        assert_eq!(quote! {[#(#pattern_bindings3),*]}.to_string(), "[]");
        assert_eq!(
            quote! {[#(#keyword_args3),*]}.to_string(),
            "[placeholder0 = \"x\" , placeholder1 = \"y\" , placeholder2 = \"z\"]"
        );

        // Example 4: Non-empty `args` and `fields` vector.
        let args4 = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        let fields4 = vec!["x".to_string(), "y".to_string()];
        let (pattern_bindings4, keyword_args4) = pattern_map(&args4, &fields4, false);
        assert_eq!(quote! {[#(#pattern_bindings4),*]}.to_string(), "[x , y]");
        assert_eq!(quote! {[#(#keyword_args4),*]}.to_string(), "[placeholder0 = x , placeholder1 = y , placeholder2 = \"z\"]");
    }
}
