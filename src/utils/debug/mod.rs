// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use std::fs::OpenOptions;
use std::io::Write;

/// Writes the implementations to a file.
pub fn write_implementations(implementations: &proc_macro2::TokenStream, file: &str) {
    let mut f = OpenOptions::new().create(true).append(true).open(file).unwrap();
    writeln!(f, "{}", implementations).unwrap();
}
