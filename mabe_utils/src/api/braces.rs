// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

/// Returns the index of the first active left curly brace i.e. a single left curly brace or a left curly brace that is part of
/// a sequence of consecutive left curly braces with an odd number of braces.
pub fn active_left_brace(msg: &String, checkpoint: usize) -> Option<usize> {
    let mut consec_left_braces = 0;

    match msg[checkpoint..].find('{') {
        Some(mut i) => {
            consec_left_braces += 1;
            i += checkpoint;
            while i < msg.len() - 1 && msg.chars().nth(i + 1).unwrap() == '{' {
                consec_left_braces += 1;
                i += 1;
            }

            if consec_left_braces % 2 != 0 {
                Some(i)
            } else {
                active_left_brace(msg, i + 1)
            }
        }
        None => None,
    }
}

/// Returns the index of the first active right curly brace i.e. a single right curly brace or a right curly brace that is part
/// of a sequence of consecutive right curly braces with an odd number of braces.
pub fn active_right_brace(msg: &String, checkpoint: usize) -> Option<usize> {
    let mut consec_right_braces = 0;

    match msg[checkpoint..].find('}') {
        Some(mut i) => {
            consec_right_braces += 1;
            i += checkpoint;
            let index = i;
            while i < msg.len() - 1 && msg.chars().nth(i + 1).unwrap() == '}' {
                consec_right_braces += 1;
                i += 1;
            }

            if consec_right_braces % 2 != 0 {
                Some(index)
            } else {
                active_right_brace(msg, i + 1)
            }
        }
        None => None,
    }
}
