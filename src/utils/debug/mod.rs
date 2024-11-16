// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.
use std::fs::File;
use std::io::Write;

/// Writes the implementations to a file.
pub fn write_implementations(implementations: &proc_macro2::TokenStream, file: &str) {
    let mut f = File::create(file).unwrap();
    writeln!(f, "{}", implementations).unwrap();
}
