// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe::Mabe;

#[derive(Mabe)]
enum MyError {
    #[error("The error message for Error1. The placeholders are: {0}, {y} and {{0}}.")]
    #[cause("The cause message for Error1. The placeholders are: {msg}, {{{cause}}} and false.")]
    #[debug("The debug message for Error1. The placeholders are: {007}, {420} and {000}.")]
    Error1(String),

    #[error("The error message for Error2. The placeholders are: {}, {{}} and {{{0}}}.")]
    #[cause("The cause message for Error2. The placeholders are: {1.5}, {0} and 0.")]
    #[debug("The debug message for Error2. The placeholders are: {0}, {-0} and {0}.")]
    Error2(i32),

    #[error("The error message for Error3. The placeholders are: {{0}}, {2} and {1}.")]
    #[cause("The cause message for Error3. The placeholders are: {2}, {{{--1}}} and {{1}}.")]
    #[debug("The debug message for Error3. The placeholders are: {1}, {{{0}}} and {2}.")]
    Error3(String, usize, f64),
}

#[test]
fn test() {
    let error1 = MyError::Error1("x".to_string());
    assert_eq!(error1.state(), "MyError::Error1(x)");
    assert_eq!(error1.error(), "The error message for Error1. The placeholders are: x, y and {0}.");
    assert_eq!(error1.cause(), "The cause message for Error1. The placeholders are: msg, {cause} and false.");
    assert_eq!(error1.debug(), "The debug message for Error1. The placeholders are: 007, 420 and 000.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error1.to_string(), "\n[error] The error message for Error1. The placeholders are: x, y and {0}.\n[cause] The cause message for Error1. The placeholders are: msg, {cause} and false.\n[debug] The debug message for Error1. The placeholders are: 007, 420 and 000.");

    #[cfg(feature = "colorize")]
    println!("{}", error1);

    let error2 = MyError::Error2(-53);
    assert_eq!(error2.state(), "MyError::Error2(-53)");
    assert_eq!(error2.error(), "The error message for Error2. The placeholders are: , {} and {-53}.");
    assert_eq!(error2.cause(), "The cause message for Error2. The placeholders are: 1.5, -53 and 0.");
    assert_eq!(error2.debug(), "The debug message for Error2. The placeholders are: -53, -0 and -53.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error2.to_string(), "\n[error] The error message for Error2. The placeholders are: , {} and {-53}.\n[cause] The cause message for Error2. The placeholders are: 1.5, -53 and 0.\n[debug] The debug message for Error2. The placeholders are: -53, -0 and -53.");

    #[cfg(feature = "colorize")]
    println!("{}", error2);

    let error3 = MyError::Error3("msg".to_string(), 100487, 3.1415);
    assert_eq!(error3.state(), "MyError::Error3(msg, 100487, 3.1415)");
    assert_eq!(error3.error(), "The error message for Error3. The placeholders are: {0}, 3.1415 and 100487.");
    assert_eq!(error3.cause(), "The cause message for Error3. The placeholders are: 3.1415, {--1} and {1}.");
    assert_eq!(error3.debug(), "The debug message for Error3. The placeholders are: 100487, {msg} and 3.1415.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error3.to_string(), "\n[error] The error message for Error3. The placeholders are: {0}, 3.1415 and 100487.\n[cause] The cause message for Error3. The placeholders are: 3.1415, {--1} and {1}.\n[debug] The debug message for Error3. The placeholders are: 100487, {msg} and 3.1415.");

    #[cfg(feature = "colorize")]
    println!("{}", error3);
}
