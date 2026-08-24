// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

//! This module contains helper tools for the [`mabe_attr`](https://mabe.readthedocs.io/en/stable/mabe_attr/index.html) and [`mabe_derive`](https://mabe.readthedocs.io/en/stable/mabe_derive/index.html) dependency crates.

// [X] Header
#[cfg(feature = "colorize")]
pub const X: &str = "\x1b[34m[\x1b[0m\x1b[31mX\x1b[0m\x1b[34m]\x1b[0m";
#[cfg(not(feature = "colorize"))]
pub const X: &str = "[X]";

// Branches
#[cfg(feature = "colorize")]
pub const L_BRANCH: &str = " \x1b[34m└─\x1b[0m ";
#[cfg(not(feature = "colorize"))]
pub const L_BRANCH: &str = " └─ ";

#[cfg(feature = "colorize")]
pub const T_BRANCH: &str = " \x1b[34m├─\x1b[0m ";
#[cfg(not(feature = "colorize"))]
pub const T_BRANCH: &str = " ├─ ";

// Pipe
#[cfg(feature = "colorize")]
pub const PIPE: &str = " \x1b[34m│\x1b[0m  ";
#[cfg(not(feature = "colorize"))]
pub const PIPE: &str = " │  ";

/// Splits text into lines based on the terminal's width, and preserves the existing `\n`.
pub fn wrap(text: String) -> Vec<String> {
    let width = match terminal_size::terminal_size() {
        // Retrieves the width of the terminal at runtime.
        Some((terminal_size::Width(w), _)) => (w as usize).saturating_sub(5), // safety check against very small terminals
        None => 75,                                                           // default fallback
    };

    let mut lines = Vec::new();

    for raw_line in text.lines() {
        let mut current_line = String::new();
        let mut current_len = 0; // The width is measured in characters, not bytes, to handle non-ASCII text correctly.

        for word in raw_line.split_whitespace() {
            // Calculates the required length: current + word + space (if not start of line)
            let space_needed = if current_line.is_empty() { 0 } else { 1 };
            let word_len = word.chars().count();

            // If adding this word exceeds width...
            if current_len + space_needed + word_len > width {
                // ...push the current line (if it has content)
                if !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = String::new();
                    current_len = 0;
                }
                // Note: If 'word' is longer than 'width' all by itself,
                // it will just start the new current_line and overflow.
            }

            if !current_line.is_empty() {
                current_line.push(' ');
                current_len += 1;
            }
            current_line.push_str(word);
            current_len += word_len;
        }

        // Pushes the last segment
        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }
    lines
}
