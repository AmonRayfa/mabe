// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

mod braces;
pub use braces::*;

/// Returns a tuple containing the formatted message and the extracted arguments.
/// In the formatted message, all single curly braces (i.e. curly braces that are not part of a placeholder),
/// and all placeholders are replaced with generic placeholders (e.g. {placeholder0}, {placeholder1}, etc.).
/// The replaced elements are stored in the extracted arguments vector.
///
/// # Examples
/// ```
/// use mabe_utils::generic_format;
///
/// // Example 1: No curly braces.
/// let msg1 = "The placeholders are: x, y and z.".to_string();
/// let (formatted_msg1, extracted_args_msg1) = generic_format(msg1);
/// assert_eq!(formatted_msg1, "The placeholders are: x, y and z.");
/// assert_eq!(extracted_args_msg1, Vec::<String>::new());
///
/// // Example 2: Uneven number of curly braces.
/// let msg2 = "The placeholders are: {x}, {{{y}}} and {z}}}.".to_string();
/// let (formatted_msg2, extracted_args_msg2) = generic_format(msg2);
/// assert_eq!(formatted_msg2, "The placeholders are: {placeholder0}, {{{placeholder1}}} and {placeholder2}}}.");
/// assert_eq!(extracted_args_msg2, vec!["x".to_string(), "y".to_string(), "z".to_string()]);
///
/// // Example 3: Even number of curly braces.
/// let msg3 = "The placeholders are: {{x}}, {{{{y}}}} and {{{{z}}.".to_string();
/// let (formatted_msg3, extracted_args_msg3) = generic_format(msg3);
/// assert_eq!(formatted_msg3, "The placeholders are: {{x}}, {{{{y}}}} and {{{{z}}.");
/// assert_eq!(extracted_args_msg3, Vec::<String>::new());
///
/// // Example 4: Empty curly braces.
/// let msg4 = "The placeholders are: {}, {{}}}} and {{{ }.".to_string();
/// let (formatted_msg4, extracted_args_msg4) = generic_format(msg4);
/// assert_eq!(formatted_msg4, "The placeholders are: {placeholder0}, {{}}}} and {{{placeholder1}.");
/// assert_eq!(extracted_args_msg4, vec!["".to_string(), " ".to_string()]);
///
/// // Example 5: Unbalanced curly braces.
/// let msg5 = "The placeholders are: {x}}, {y} and z.".to_string();
/// let (formatted_msg5, extracted_args_msg5) = generic_format(msg5);
/// assert_eq!(formatted_msg5, "The placeholders are: {placeholder0} and z.");
/// assert_eq!(extracted_args_msg5, vec!["x}}, {y".to_string()]);
/// ```
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
