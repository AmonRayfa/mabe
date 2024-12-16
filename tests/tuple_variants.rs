// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe::Mabe;

#[derive(Mabe)]
enum Error {
    #[error("The error message for Tuple1. The placeholders are: {0}, {y}, {{0}}, {msg}, and {{{cause}}}.")]
    #[debug("The debug message for Tuple1. The placeholders are: {007}, {420}, {000}, and false.")]
    Tuple1(String),

    #[error("The error message for Tuple2. The placeholders are: {}, {{}}, {{{0}}}, {1.5}, and {0}.")]
    #[debug("The debug message for Tuple2. The placeholders are: {0}, {-0}, {0}, and 0.")]
    Tuple2(i32),

    #[error("The error message for Tuple3. The placeholders are: {{0}}, {2}, {1}, {2}, and {{{--1}}}.")]
    #[debug("The debug message for Tuple3. The placeholders are: {1}, {{{0}}}, {2}, and {{1}}.")]
    Tuple3(String, usize, f64),
}

#[test]
fn test() {
    let error1 = Error::Tuple1("x".to_string());
    assert_eq!(error1.state(), "Error::Tuple1(x)");
    assert_eq!(error1.error(), "The error message for Tuple1. The placeholders are: x, y, {0}, msg, and {cause}.");
    assert_eq!(error1.debug(), "The debug message for Tuple1. The placeholders are: 007, 420, 000, and false.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error1.to_string(), "\n[error] The error message for Tuple1. The placeholders are: x, y, {0}, msg, and {cause}.\n[debug] The debug message for Tuple1. The placeholders are: 007, 420, 000, and false.");

    #[cfg(feature = "colorize")]
    println!("{}", error1);

    let error2 = Error::Tuple2(-53);
    assert_eq!(error2.state(), "Error::Tuple2(-53)");
    assert_eq!(error2.error(), "The error message for Tuple2. The placeholders are: , {}, {-53}, 1.5, and -53.");
    assert_eq!(error2.debug(), "The debug message for Tuple2. The placeholders are: -53, -0, -53, and 0.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error2.to_string(), "\n[error] The error message for Tuple2. The placeholders are: , {}, {-53}, 1.5, and -53.\n[debug] The debug message for Tuple2. The placeholders are: -53, -0, -53, and 0.");

    #[cfg(feature = "colorize")]
    println!("{}", error2);

    let error3 = Error::Tuple3("msg".to_string(), 100487, 3.1415);
    assert_eq!(error3.state(), "Error::Tuple3(msg, 100487, 3.1415)");
    assert_eq!(error3.error(), "The error message for Tuple3. The placeholders are: {0}, 3.1415, 100487, 3.1415, and {--1}.");
    assert_eq!(error3.debug(), "The debug message for Tuple3. The placeholders are: 100487, {msg}, 3.1415, and {1}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error3.to_string(), "\n[error] The error message for Tuple3. The placeholders are: {0}, 3.1415, 100487, 3.1415, and {--1}.\n[debug] The debug message for Tuple3. The placeholders are: 100487, {msg}, 3.1415, and {1}.");

    #[cfg(feature = "colorize")]
    println!("{}", error3);
}
