// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

#[cfg(feature = "colored")]
use colored::Colorize;

use mabe::Mabe;

#[derive(Debug, Mabe)]
enum MyError {
    #[error("The error message for Error1. The placeholders are: {0}, {msg} and {{0}}.")]
    #[reason("The reason message for Error1. The placeholders are: {None}, {{{None}}} and false.")]
    #[solution("The solution message for Error1. The placeholders are: {007}, {420} and {000}.")]
    Error1 { msg: String },

    #[error("The error message for Error2. The placeholders are: {}, {{}} and {{{msg}}}.")]
    #[reason("The reason message for Error2. The placeholders are: {speed}, {height} and 0.")]
    #[solution("The solution message for Error2. The placeholders are: {comment}, {-0} and {0}.")]
    Error2 { speed: i32 },

    #[error("The error message for Error3. The placeholders are: {{0}}, {ratio} and {height}.")]
    #[reason("The reason message for Error3. The placeholders are: {2}, {{{--1}}} and {{ratio}}.")]
    #[solution("The solution message for Error3. The placeholders are: {comment}, {{{0}}} and {ratio}.")]
    Error3 { comment: String, height: usize, ratio: f64 },
}

#[test]
fn test() {
    let error1 = MyError::Error1 { msg: "Something...".to_string() };
    assert_eq!(error1.error(), "The error message for Error1. The placeholders are: 0, Something... and {0}.");
    assert_eq!(error1.reason(), "The reason message for Error1. The placeholders are: None, {None} and false.");
    assert_eq!(error1.solution(), "The solution message for Error1. The placeholders are: 007, 420 and 000.");
    println!("{}", error1);

    let error2 = MyError::Error2 { speed: 100 };
    assert_eq!(error2.error(), "The error message for Error2. The placeholders are: , {} and {msg}.");
    assert_eq!(error2.reason(), "The reason message for Error2. The placeholders are: 100, height and 0.");
    assert_eq!(error2.solution(), "The solution message for Error2. The placeholders are: comment, -0 and 0.");
    println!("{}", error2);

    let error3 = MyError::Error3 { comment: "msg".to_string(), height: 100487, ratio: 3.1415 };
    assert_eq!(error3.error(), "The error message for Error3. The placeholders are: {0}, 3.1415 and 100487.");
    assert_eq!(error3.reason(), "The reason message for Error3. The placeholders are: 2, {--1} and {ratio}.");
    assert_eq!(error3.solution(), "The solution message for Error3. The placeholders are: msg, {0} and 3.1415.");
    println!("{}", error3);
}
