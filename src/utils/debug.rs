// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use std::env::var;
use std::fs::OpenOptions;
use std::io::Write;

/// Logs the output of a macro (i.e. the generated code) to a file.
pub fn log_macro_output(macro_output: &proc_macro2::TokenStream, file: &str) {
    if var("CARGO_PKG_NAME").as_deref() == Ok("mabe") {
        let mut f = OpenOptions::new().create(true).append(true).open(file).unwrap();
        writeln!(f, "{}", macro_output).unwrap();
    }
}
