// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use std::env::var;
use std::fs::OpenOptions;
use std::io::Write;

/// Logs the output of a macro (i.e. the generated code) to a file. The logging only happens when the `MABE_DEBUG` environment
/// variable is set (e.g. `MABE_DEBUG=1 cargo build`), so that the log file doesn't silently grow with every build. Note that
/// the file is appended to, not overwritten, so it should be deleted manually when it's no longer needed.
pub fn log_macro_output(macro_output: &proc_macro2::TokenStream, file: &str) {
    if var("MABE_DEBUG").is_ok() && var("CARGO_PKG_NAME").as_deref() == Ok("mabe") {
        let mut f = OpenOptions::new().create(true).append(true).open(file).unwrap();
        writeln!(f, "{}", macro_output).unwrap();
    }
}
