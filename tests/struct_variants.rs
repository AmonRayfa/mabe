// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe::Mabe;

#[derive(Mabe)]
enum MyError {
    #[error("The error message for Error1. The placeholders are: {0}, {msg} and {{0}}.")]
    #[cause("The cause message for Error1. The placeholders are: {None}, {{{None}}} and false.")]
    #[debug("The debug message for Error1. The placeholders are: {007}, {420} and {000}.")]
    Error1 { msg: String },

    #[error("The error message for Error2. The placeholders are: {}, {{}} and {{{msg}}}.")]
    #[cause("The cause message for Error2. The placeholders are: {speed}, {height} and 0.")]
    #[debug("The debug message for Error2. The placeholders are: {comment}, {-0} and {0}.")]
    Error2 { speed: i32 },

    #[error("The error message for Error3. The placeholders are: {{0}}, {ratio} and {height}.")]
    #[cause("The cause message for Error3. The placeholders are: {2}, {{{--1}}} and {{ratio}}.")]
    #[debug("The debug message for Error3. The placeholders are: {comment}, {{{0}}} and {ratio}.")]
    Error3 { comment: String, height: usize, ratio: f64 },
}

#[test]
fn test() {
    let error1 = MyError::Error1 { msg: "Something...".to_string() };
    assert_eq!(error1.state(), "MyError::Error1 { msg: Something... }");
    assert_eq!(error1.error(), "The error message for Error1. The placeholders are: 0, Something... and {0}.");
    assert_eq!(error1.cause(), "The cause message for Error1. The placeholders are: None, {None} and false.");
    assert_eq!(error1.debug(), "The debug message for Error1. The placeholders are: 007, 420 and 000.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error1.to_string(), "\n[error] The error message for Error1. The placeholders are: 0, Something... and {0}.\n[cause] The cause message for Error1. The placeholders are: None, {None} and false.\n[debug] The debug message for Error1. The placeholders are: 007, 420 and 000.");

    #[cfg(feature = "colorize")]
    println!("{}", error1);

    let error2 = MyError::Error2 { speed: 100 };
    assert_eq!(error2.state(), "MyError::Error2 { speed: 100 }");
    assert_eq!(error2.error(), "The error message for Error2. The placeholders are: , {} and {msg}.");
    assert_eq!(error2.cause(), "The cause message for Error2. The placeholders are: 100, height and 0.");
    assert_eq!(error2.debug(), "The debug message for Error2. The placeholders are: comment, -0 and 0.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error2.to_string(), "\n[error] The error message for Error2. The placeholders are: , {} and {msg}.\n[cause] The cause message for Error2. The placeholders are: 100, height and 0.\n[debug] The debug message for Error2. The placeholders are: comment, -0 and 0.");

    #[cfg(feature = "colorize")]
    println!("{}", error2);

    let error3 = MyError::Error3 { comment: "msg".to_string(), height: 100487, ratio: 3.1415 };
    assert_eq!(error3.state(), "MyError::Error3 { comment: msg, height: 100487, ratio: 3.1415 }");
    assert_eq!(error3.error(), "The error message for Error3. The placeholders are: {0}, 3.1415 and 100487.");
    assert_eq!(error3.cause(), "The cause message for Error3. The placeholders are: 2, {--1} and {ratio}.");
    assert_eq!(error3.debug(), "The debug message for Error3. The placeholders are: msg, {0} and 3.1415.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error3.to_string(), "\n[error] The error message for Error3. The placeholders are: {0}, 3.1415 and 100487.\n[cause] The cause message for Error3. The placeholders are: 2, {--1} and {ratio}.\n[debug] The debug message for Error3. The placeholders are: msg, {0} and 3.1415.");

    #[cfg(feature = "colorize")]
    println!("{}", error3);
}
