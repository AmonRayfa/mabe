// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

#[cfg(feature = "color")]
use colored::Colorize;

use mabe::Mabe;

#[derive(Debug, Mabe)]
enum MyError {
    #[error("The error message for Error1. The placeholders are: {0}, {y} and {{0}}.")]
    #[reason("The reason message for Error1. The placeholders are: {msg}, {{{reason}}} and false.")]
    #[solution("The solution message for Error1. The placeholders are: {007}, {420} and {000}.")]
    Error1(String),

    #[error("The error message for Error2. The placeholders are: {}, {{}} and {{{0}}}.")]
    #[reason("The reason message for Error2. The placeholders are: {1.5}, {0} and 0.")]
    #[solution("The solution message for Error2. The placeholders are: {0}, {-0} and {0}.")]
    Error2(i32),

    #[error("The error message for Error3. The placeholders are: {{0}}, {2} and {1}.")]
    #[reason("The reason message for Error3. The placeholders are: {2}, {{{--1}}} and {{1}}.")]
    #[solution("The solution message for Error3. The placeholders are: {1}, {{{0}}} and {2}.")]
    Error3(String, usize, f64),
}

#[test]
fn test() {
    let error1 = MyError::Error1("x".to_string());
    assert_eq!(error1.error(), "The error message for Error1. The placeholders are: x, y and {0}.");
    assert_eq!(error1.reason(), "The reason message for Error1. The placeholders are: msg, {reason} and false.");
    assert_eq!(error1.solution(), "The solution message for Error1. The placeholders are: 007, 420 and 000.");
    println!("{}", error1);

    let error2 = MyError::Error2(-53);
    assert_eq!(error2.error(), "The error message for Error2. The placeholders are: , {} and {-53}.");
    assert_eq!(error2.reason(), "The reason message for Error2. The placeholders are: 1.5, -53 and 0.");
    assert_eq!(error2.solution(), "The solution message for Error2. The placeholders are: -53, -0 and -53.");
    println!("{}", error2);

    let error3 = MyError::Error3("msg".to_string(), 100487, 3.1415);
    assert_eq!(error3.error(), "The error message for Error3. The placeholders are: {0}, 3.1415 and 100487.");
    assert_eq!(error3.reason(), "The reason message for Error3. The placeholders are: 3.1415, {--1} and {1}.");
    assert_eq!(error3.solution(), "The solution message for Error3. The placeholders are: 100487, {msg} and 3.1415.");
    println!("{}", error3);
}
