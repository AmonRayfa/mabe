// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe::Mabe;

#[derive(Mabe)]
enum Error {
    #[error("The error message for Tuple1. The placeholders are: {0}, {y} and {{0}}.")]
    #[cause("The cause message for Tuple1. The placeholders are: {msg}, {{{cause}}} and false.")]
    #[debug("The debug message for Tuple1. The placeholders are: {007}, {420} and {000}.")]
    Tuple1(String),

    #[error("The error message for Tuple2. The placeholders are: {}, {{}} and {{{0}}}.")]
    #[cause("The cause message for Tuple2. The placeholders are: {1.5}, {0} and 0.")]
    #[debug("The debug message for Tuple2. The placeholders are: {0}, {-0} and {0}.")]
    Tuple2(i32),

    #[error("The error message for Tuple3. The placeholders are: {{0}}, {2} and {1}.")]
    #[cause("The cause message for Tuple3. The placeholders are: {2}, {{{--1}}} and {{1}}.")]
    #[debug("The debug message for Tuple3. The placeholders are: {1}, {{{0}}} and {2}.")]
    Tuple3(String, usize, f64),
}

#[test]
fn test() {
    let tuple1_error = Error::Tuple1("x".to_string());
    assert_eq!(tuple1_error.state(), "Error::Tuple1(x)");
    assert_eq!(tuple1_error.error(), "The error message for Tuple1. The placeholders are: x, y and {0}.");
    assert_eq!(tuple1_error.cause(), "The cause message for Tuple1. The placeholders are: msg, {cause} and false.");
    assert_eq!(tuple1_error.debug(), "The debug message for Tuple1. The placeholders are: 007, 420 and 000.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(tuple1_error.to_string(), "\n[error] The error message for Tuple1. The placeholders are: x, y and {0}.\n[cause] The cause message for Tuple1. The placeholders are: msg, {cause} and false.\n[debug] The debug message for Tuple1. The placeholders are: 007, 420 and 000.");

    #[cfg(feature = "colorize")]
    println!("{}", tuple1_error);

    let tuple2_error = Error::Tuple2(-53);
    assert_eq!(tuple2_error.state(), "Error::Tuple2(-53)");
    assert_eq!(tuple2_error.error(), "The error message for Tuple2. The placeholders are: , {} and {-53}.");
    assert_eq!(tuple2_error.cause(), "The cause message for Tuple2. The placeholders are: 1.5, -53 and 0.");
    assert_eq!(tuple2_error.debug(), "The debug message for Tuple2. The placeholders are: -53, -0 and -53.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(tuple2_error.to_string(), "\n[error] The error message for Tuple2. The placeholders are: , {} and {-53}.\n[cause] The cause message for Tuple2. The placeholders are: 1.5, -53 and 0.\n[debug] The debug message for Tuple2. The placeholders are: -53, -0 and -53.");

    #[cfg(feature = "colorize")]
    println!("{}", tuple2_error);

    let tuple3_error = Error::Tuple3("msg".to_string(), 100487, 3.1415);
    assert_eq!(tuple3_error.state(), "Error::Tuple3(msg, 100487, 3.1415)");
    assert_eq!(tuple3_error.error(), "The error message for Tuple3. The placeholders are: {0}, 3.1415 and 100487.");
    assert_eq!(tuple3_error.cause(), "The cause message for Tuple3. The placeholders are: 3.1415, {--1} and {1}.");
    assert_eq!(tuple3_error.debug(), "The debug message for Tuple3. The placeholders are: 100487, {msg} and 3.1415.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(tuple3_error.to_string(), "\n[error] The error message for Tuple3. The placeholders are: {0}, 3.1415 and 100487.\n[cause] The cause message for Tuple3. The placeholders are: 3.1415, {--1} and {1}.\n[debug] The debug message for Tuple3. The placeholders are: 100487, {msg} and 3.1415.");

    #[cfg(feature = "colorize")]
    println!("{}", tuple3_error);
}
