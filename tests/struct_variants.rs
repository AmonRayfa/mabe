// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe::Mabe;

#[derive(Mabe)]
enum Error {
    #[error("The error message for Struct1. The placeholders are: {0}, {msg}, {{0}}, {None}, and {{{None}}}.")]
    #[debug("The debug message for Struct1. The placeholders are: {007}, {420}, {000}, and false.")]
    Struct1 { msg: String },

    #[error("The error message for Struct2. The placeholders are: {}, {{}}, {{{msg}}}, {speed}, and {height}.")]
    #[debug("The debug message for Struct2. The placeholders are: {comment}, {-0}, {0}, and 0.")]
    Struct2 { speed: i32 },

    #[error("The error message for Struct3. The placeholders are: {{0}}, {ratio}, {height}, {2}, and {{{--1}}}.")]
    #[debug("The debug message for Struct3. The placeholders are: {comment}, {{{0}}}, {ratio}, and {{ratio}}.")]
    Struct3 { comment: String, height: usize, ratio: f64 },
}

#[test]
fn test() {
    let error1 = Error::Struct1 { msg: "Something...".to_string() };
    assert_eq!(error1.state(), "Error::Struct1 { msg: Something... }");
    assert_eq!(error1.error(), "The error message for Struct1. The placeholders are: 0, Something..., {0}, None, and {None}.");
    assert_eq!(error1.debug(), "The debug message for Struct1. The placeholders are: 007, 420, 000, and false.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error1.to_string(), "\n[error] The error message for Struct1. The placeholders are: 0, Something..., {0}, None, and {None}.\n[debug] The debug message for Struct1. The placeholders are: 007, 420, 000, and false.");

    println!("{}", error1);

    let error2 = Error::Struct2 { speed: 100 };
    assert_eq!(error2.state(), "Error::Struct2 { speed: 100 }");
    assert_eq!(error2.error(), "The error message for Struct2. The placeholders are: , {}, {msg}, 100, and height.");
    assert_eq!(error2.debug(), "The debug message for Struct2. The placeholders are: comment, -0, 0, and 0.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error2.to_string(), "\n[error] The error message for Struct2. The placeholders are: , {}, {msg}, 100, and height.\n[debug] The debug message for Struct2. The placeholders are: comment, -0, 0, and 0.");

    println!("{}", error2);

    let error3 = Error::Struct3 { comment: "msg".to_string(), height: 100487, ratio: 3.1415 };
    assert_eq!(error3.state(), "Error::Struct3 { comment: msg, height: 100487, ratio: 3.1415 }");
    assert_eq!(error3.error(), "The error message for Struct3. The placeholders are: {0}, 3.1415, 100487, 2, and {--1}.");
    assert_eq!(error3.debug(), "The debug message for Struct3. The placeholders are: msg, {0}, 3.1415, and {ratio}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error3.to_string(), "\n[error] The error message for Struct3. The placeholders are: {0}, 3.1415, 100487, 2, and {--1}.\n[debug] The debug message for Struct3. The placeholders are: msg, {0}, 3.1415, and {ratio}.");

    println!("{}", error3);
}
