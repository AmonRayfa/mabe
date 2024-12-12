// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe::Mabe;

#[derive(Mabe)]
enum Error {
    #[error("The error message for Struct1. The placeholders are: {0}, {msg} and {{0}}.")]
    #[cause("The cause message for Struct1. The placeholders are: {None}, {{{None}}} and false.")]
    #[debug("The debug message for Struct1. The placeholders are: {007}, {420} and {000}.")]
    Struct1 { msg: String },

    #[error("The error message for Struct2. The placeholders are: {}, {{}} and {{{msg}}}.")]
    #[cause("The cause message for Struct2. The placeholders are: {speed}, {height} and 0.")]
    #[debug("The debug message for Struct2. The placeholders are: {comment}, {-0} and {0}.")]
    Struct2 { speed: i32 },

    #[error("The error message for Struct3. The placeholders are: {{0}}, {ratio} and {height}.")]
    #[cause("The cause message for Struct3. The placeholders are: {2}, {{{--1}}} and {{ratio}}.")]
    #[debug("The debug message for Struct3. The placeholders are: {comment}, {{{0}}} and {ratio}.")]
    Struct3 { comment: String, height: usize, ratio: f64 },
}

#[test]
fn test() {
    let struct1_error = Error::Struct1 { msg: "Something...".to_string() };
    assert_eq!(struct1_error.state(), "Error::Struct1 { msg: Something... }");
    assert_eq!(struct1_error.error(), "The error message for Struct1. The placeholders are: 0, Something... and {0}.");
    assert_eq!(struct1_error.cause(), "The cause message for Struct1. The placeholders are: None, {None} and false.");
    assert_eq!(struct1_error.debug(), "The debug message for Struct1. The placeholders are: 007, 420 and 000.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(struct1_error.to_string(), "\n[error] The error message for Struct1. The placeholders are: 0, Something... and {0}.\n[cause] The cause message for Struct1. The placeholders are: None, {None} and false.\n[debug] The debug message for Struct1. The placeholders are: 007, 420 and 000.");

    #[cfg(feature = "colorize")]
    println!("{}", struct1_error);

    let struct2_error = Error::Struct2 { speed: 100 };
    assert_eq!(struct2_error.state(), "Error::Struct2 { speed: 100 }");
    assert_eq!(struct2_error.error(), "The error message for Struct2. The placeholders are: , {} and {msg}.");
    assert_eq!(struct2_error.cause(), "The cause message for Struct2. The placeholders are: 100, height and 0.");
    assert_eq!(struct2_error.debug(), "The debug message for Struct2. The placeholders are: comment, -0 and 0.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(struct2_error.to_string(), "\n[error] The error message for Struct2. The placeholders are: , {} and {msg}.\n[cause] The cause message for Struct2. The placeholders are: 100, height and 0.\n[debug] The debug message for Struct2. The placeholders are: comment, -0 and 0.");

    #[cfg(feature = "colorize")]
    println!("{}", struct2_error);

    let struct3_error = Error::Struct3 { comment: "msg".to_string(), height: 100487, ratio: 3.1415 };
    assert_eq!(struct3_error.state(), "Error::Struct3 { comment: msg, height: 100487, ratio: 3.1415 }");
    assert_eq!(struct3_error.error(), "The error message for Struct3. The placeholders are: {0}, 3.1415 and 100487.");
    assert_eq!(struct3_error.cause(), "The cause message for Struct3. The placeholders are: 2, {--1} and {ratio}.");
    assert_eq!(struct3_error.debug(), "The debug message for Struct3. The placeholders are: msg, {0} and 3.1415.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(struct3_error.to_string(), "\n[error] The error message for Struct3. The placeholders are: {0}, 3.1415 and 100487.\n[cause] The cause message for Struct3. The placeholders are: 2, {--1} and {ratio}.\n[debug] The debug message for Struct3. The placeholders are: msg, {0} and 3.1415.");

    #[cfg(feature = "colorize")]
    println!("{}", struct3_error);
}
