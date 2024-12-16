// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

/// Returns the index of the first active left curly brace, which is either a single left curly brace or the last left curly
/// brace in an odd-length sequence of consecutive left curly braces. If no such brace is found, `None` is returned.
pub fn find_active_left_brace<M: ToString>(msg: M, start: usize) -> Option<usize> {
    let msg = msg.to_string();
    let mut consec_left_braces = 0;

    match msg[start..].find('{') {
        Some(mut i) => {
            consec_left_braces += 1;
            i += start;
            while i < msg.len() - 1 && msg.chars().nth(i + 1).unwrap() == '{' {
                consec_left_braces += 1;
                i += 1;
            }

            if consec_left_braces % 2 != 0 {
                Some(i)
            } else {
                find_active_left_brace(msg, i + 1)
            }
        }
        None => None,
    }
}

/// Returns the index of the first active right curly brace, which is either a single right curly brace or the first right curly
/// brace in an odd-length sequence of consecutive right curly braces. If no such brace is found, `None` is returned.
pub fn find_active_right_brace<M: ToString>(msg: M, start: usize) -> Option<usize> {
    let msg = msg.to_string();
    let mut consec_right_braces = 0;

    match msg[start..].find('}') {
        Some(mut i) => {
            consec_right_braces += 1;
            i += start;
            let index = i;
            while i < msg.len() - 1 && msg.chars().nth(i + 1).unwrap() == '}' {
                consec_right_braces += 1;
                i += 1;
            }

            if consec_right_braces % 2 != 0 {
                Some(index)
            } else {
                find_active_right_brace(msg, i + 1)
            }
        }
        None => None,
    }
}

/// Returns the index of the first element that matches the target in a vector. If no such string is found, `None` is returned.
pub fn find_target<T: ToString, E: ToString>(target: T, vec: &[E]) -> Option<usize> {
    vec.iter().position(|e| e.to_string() == target.to_string())
}
