// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

mod finders;
use finders::*;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, Lit, Meta, NestedMeta, Variant};

/// A tool that returns the message of the attribute of a variant. The supported attributes are `error`, `reason`, and
/// `solution`. The function will panic in the following cases: if the attribute doesn't have exactly one argument, if the
/// attribute is empty, if the argument of the attribute is not a `&str`, if the `error` attribute is not found, or if the
/// attribute is found more than once.
pub fn get_msg<A: ToString>(attribute: A, variant: &Variant) -> String {
    let attribute = attribute.to_string();

    if attribute != "error" && attribute != "reason" && attribute != "solution" {
        panic!(
            "The `utils::helpers::get_msg` function only supports the `error`, `reason`, and `solution` attributes, found `{}`. This error should not be able to occur in production code. Try reloading the window. If the problem persists, report the issue to the crate's [GitHub repository](https://github.com/AmonRayfa/mabe).",
            attribute
        );
    }

    let filtered_attrs = variant
        .attrs
        .iter()
        .filter_map(|attr| {
            let mut msg = String::new();

            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                if let Some(attr_ident) = meta_list.path.get_ident() {
                    if *attr_ident == attribute {
                        if meta_list.nested.len() != 1 {
                            panic!(
                                "Expected 1 argument for the `{}` attribute of the `{}` variant, found {}.",
                                attribute,
                                variant.ident,
                                meta_list.nested.len()
                            );
                        }

                        if let NestedMeta::Lit(Lit::Str(lit_str)) = &meta_list.nested[0] {
                            if lit_str.value().is_empty() {
                                panic!(
                                    "The `{}` attribute of the `{}` variant cannot be empty. When an attribute is present, it must contain a message.",
                                    attribute, variant.ident
                                );
                            }

                            msg = lit_str.value();
                        } else {
                            panic!(
                                "Expected a `&str` argument for the `{}` attribute of the `{}` variant.",
                                attribute, variant.ident
                            );
                        }
                    }
                }
            } else {
                panic!(
                    "Failed to parse the attributes of the `{}` variant, make sure the attributes are correctly formatted.",
                    variant.ident
                );
            }

            if msg.is_empty() {
                None
            } else {
                Some(msg)
            }
        })
        .collect::<Vec<String>>();

    if filtered_attrs.is_empty() && attribute == "error" {
        panic!("The `error` attribute is required for the `{}` variant.", variant.ident);
    }

    if filtered_attrs.len() > 1 {
        panic!("The `{}` attribute of the `{}` variant can only be used once.", attribute, variant.ident);
    }

    if filtered_attrs.is_empty() {
        String::new()
    } else {
        filtered_attrs[0].clone()
    }
}

/// A tool that returns a tuple containing the formatted message and the extracted arguments as `String` and `Vec<String>` types
/// respectively. In the formatted message, all single curly braces (i.e. curly braces that are not part of a placeholder), and
/// all placeholders are replaced with generic placeholders (e.g. `{placeholder0}`, `{placeholder1}`, etc.). The replaced
/// elements are stored in the extracted arguments vector.
pub fn format_msg<M: ToString>(msg: M) -> (String, Vec<String>) {
    let msg = msg.to_string();
    let mut formatted_msg = String::new();
    let mut extracted_args = Vec::<String>::new();
    let mut placeholder_position = 0;
    let mut checkpoint = 0;

    while checkpoint < msg.len() {
        let index_left_brace = find_active_left_brace(&msg, checkpoint);
        let index_right_brace = find_active_right_brace(&msg, checkpoint);
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

/// A tool that returns a tuple containing the pattern bindings and the keyword arguments as
/// [`Vec<TokenStream>`](proc_macro2::TokenStream) types. The pattern bindings are the fields of an enum variant and the keyword
/// arguments are the placeholders and the extracted arguments. The `dunder` parameter is a boolean that determines whether the
/// pattern bindings should have underscores at the beginning and end.
pub fn map_args<A: ToString, F: ToString>(args: &[A], fields: &[F], dunder: bool) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let args = args.iter().map(|a| a.to_string()).collect::<Vec<String>>();
    let fields = fields.iter().map(|f| f.to_string()).collect::<Vec<String>>();

    let pattern_bindings = fields
        .iter()
        .map(|f| {
            let pattern = match dunder {
                true => Ident::new(format!("_{}_", f).as_str(), Span::call_site()),
                false => Ident::new(f.as_str(), Span::call_site()),
            };
            quote! { #pattern }
        })
        .collect::<Vec<TokenStream>>();

    let keyword_args = args
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            let keyword = Ident::new(format!("placeholder{}", i).as_str(), Span::call_site());
            match find_target(arg, &fields) {
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

// A tool that returns a styled prefix for the error, reason, and solution attributes using ANSI escape codes. The `colorize`
// feature must be enabled for this function to work.
pub fn style_prefix<A: ToString>(attribute: A) -> String {
    let attribute = attribute.to_string();

    if attribute != "error" && attribute != "reason" && attribute != "solution" {
        panic!("The `utils::helper::style_prefix` function only supports the `error`, `reason`, and `solution` attributes, found `{}`. This error should not be able to occur in production code. Try reloading the window. If the problem persists, report the issue to the crate's [GitHub repository](https://github.com/AmonRayfa/mabe).",
            attribute
        );
    }

    #[cfg(feature = "colorize")]
    match attribute.as_str() {
        "error" => return "\u{1b}[1;31m[error]\u{1b}[0m".to_string(), // ANSI escape code for red and bold text.
        "reason" => return "\u{1b}[1;33m[reason]\u{1b}[0m".to_string(), // ANSI escape code for yellow and bold text.
        "solution" => return "\u{1b}[1;32m[solution]\u{1b}[0m".to_string(), // ANSI escape code for green and bold text.
        _ => return String::new(),                                    // This should never be reached.
    };

    #[cfg(not(feature = "colorize"))]
    return format!("[{}]", attribute);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_msg() {
        // Example 1: Even number of curly braces.
        let msg1 = "The placeholders are: {x}, {{y}} and {z}.".to_string();
        let (formatted_msg1, extracted_args_msg1) = format_msg(msg1);
        assert_eq!(formatted_msg1, "The placeholders are: {placeholder0}, {{y}} and {placeholder1}.");
        assert_eq!(extracted_args_msg1, vec!["x".to_string(), "z".to_string()]);

        // Example 2: Uneven number of curly braces.
        let msg2 = "The placeholders are: {x}, {{{y}}} and {z}}}.".to_string();
        let (formatted_msg2, extracted_args_msg2) = format_msg(msg2);
        assert_eq!(formatted_msg2, "The placeholders are: {placeholder0}, {{{placeholder1}}} and {placeholder2}}}.");
        assert_eq!(extracted_args_msg2, vec!["x".to_string(), "y".to_string(), "z".to_string()]);

        // Example 3: Even number of curly braces.
        let msg3 = "The placeholders are: {{x}}, {{{{y}}}} and {{{{z}}.".to_string();
        let (formatted_msg3, extracted_args_msg3) = format_msg(msg3);
        assert_eq!(formatted_msg3, "The placeholders are: {{x}}, {{{{y}}}} and {{{{z}}.");
        assert_eq!(extracted_args_msg3, Vec::<String>::new());

        // Example 4: Empty curly braces.
        let msg4 = "The placeholders are: {}, {{}}}} and {{{ }.".to_string();
        let (formatted_msg4, extracted_args_msg4) = format_msg(msg4);
        assert_eq!(formatted_msg4, "The placeholders are: {placeholder0}, {{}}}} and {{{placeholder1}.");
        assert_eq!(extracted_args_msg4, vec!["".to_string(), " ".to_string()]);

        // Example 5: Unbalanced curly braces.
        let msg5 = "The placeholders are: {x}}, {y} and z.".to_string();
        let (formatted_msg5, extracted_args_msg5) = format_msg(msg5);
        assert_eq!(formatted_msg5, "The placeholders are: {placeholder0} and z.");
        assert_eq!(extracted_args_msg5, vec!["x}}, {y".to_string()]);
    }

    #[test]
    fn test_map_args() {
        // Example 1: Empty `args` and `fields` vector.
        let args1 = Vec::<String>::new();
        let fields1 = Vec::<String>::new();
        let (pattern_bindings1, keyword_args1) = map_args(&args1, &fields1, true);
        assert_eq!(quote! {[#(#pattern_bindings1),*]}.to_string(), "[]");
        assert_eq!(quote! {[#(#keyword_args1),*]}.to_string(), "[]");

        // Example 2: Empty `args` vector.
        let args2 = Vec::<String>::new();
        let fields2 = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        let (pattern_bindings2, keyword_args2) = map_args(&args2, &fields2, true);
        assert_eq!(quote! {[#(#pattern_bindings2),*]}.to_string(), "[_x_ , _y_ , _z_]");
        assert_eq!(quote! {[#(#keyword_args2),*]}.to_string(), "[]");

        // Example 3: Empty `fields` vector.
        let args3 = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        let fields3 = Vec::<String>::new();
        let (pattern_bindings3, keyword_args3) = map_args(&args3, &fields3, true);
        assert_eq!(quote! {[#(#pattern_bindings3),*]}.to_string(), "[]");
        assert_eq!(
            quote! {[#(#keyword_args3),*]}.to_string(),
            "[placeholder0 = \"x\" , placeholder1 = \"y\" , placeholder2 = \"z\"]"
        );

        // Example 4: Non-empty `args` and `fields` vector.
        let args4 = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        let fields4 = vec!["x".to_string(), "y".to_string()];
        let (pattern_bindings4, keyword_args4) = map_args(&args4, &fields4, false);
        assert_eq!(quote! {[#(#pattern_bindings4),*]}.to_string(), "[x , y]");
        assert_eq!(quote! {[#(#keyword_args4),*]}.to_string(), "[placeholder0 = x , placeholder1 = y , placeholder2 = \"z\"]");
    }
}
