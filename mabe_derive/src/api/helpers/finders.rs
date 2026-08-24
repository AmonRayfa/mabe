// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

/// Returns the byte index of the first active left curly brace, which is either a single left curly brace or the last left
/// curly brace in an odd-length sequence of consecutive left curly braces. If no such brace is found, `None` is returned. The
/// scan works on bytes, which is safe because curly braces are ASCII characters and can never appear inside the multi-byte
/// sequences of UTF-8 encoded text.
pub(super) fn find_active_left_brace(msg: &str, start: usize) -> Option<usize> {
    let bytes = msg.as_bytes();
    let mut i = start;

    while i < bytes.len() {
        if bytes[i] == b'{' {
            let sequence_start = i;
            while i < bytes.len() && bytes[i] == b'{' {
                i += 1;
            }

            if (i - sequence_start) % 2 != 0 {
                return Some(i - 1);
            }
        } else {
            i += 1;
        }
    }

    None
}

/// Returns the byte index of the first active right curly brace, which is either a single right curly brace or the first right
/// curly brace in an odd-length sequence of consecutive right curly braces. If no such brace is found, `None` is returned. The
/// scan works on bytes, which is safe because curly braces are ASCII characters and can never appear inside the multi-byte
/// sequences of UTF-8 encoded text.
pub(super) fn find_active_right_brace(msg: &str, start: usize) -> Option<usize> {
    let bytes = msg.as_bytes();
    let mut i = start;

    while i < bytes.len() {
        if bytes[i] == b'}' {
            let sequence_start = i;
            while i < bytes.len() && bytes[i] == b'}' {
                i += 1;
            }

            if (i - sequence_start) % 2 != 0 {
                return Some(sequence_start);
            }
        } else {
            i += 1;
        }
    }

    None
}

/// Returns the index of the first element that matches the target in a vector. If no such string is found, `None` is returned.
pub(super) fn find_target<T: ToString, E: ToString>(target: T, vec: &[E]) -> Option<usize> {
    vec.iter().position(|e| e.to_string() == target.to_string())
}
