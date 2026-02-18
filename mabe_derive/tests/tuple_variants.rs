// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe_derive::Error;

#[derive(Error)]
enum ErrorEnum {
    #[error("The error message for Tuple1. The placeholders are: {0}, {y}, {{0}}, {msg}, and {{{cause}}}.")]
    Tuple1(String),

    #[error("The error message for Tuple2. The placeholders are: {0}, {007}, {420}, {000}, and false.")]
    Tuple2(String),

    #[error("The error message for Tuple3. The placeholders are: {}, {{}}, {{{0}}}, {1.5}, and {0}.")]
    Tuple3(i32),

    #[error("The error message for Tuple4. The placeholders are: {0}, {-0}, {0}, and 0.")]
    Tuple4(i32),

    #[error("The error message for Tuple5. The placeholders are: {0}, {{0}}, {2}, {1}, {2}, and {{{--1}}}.")]
    Tuple5(String, usize, f64),

    #[error("The error message for Tuple6. The placeholders are: {1}, {{{0}}}, {2}, and {{1}}.")]
    Tuple6(String, usize, f64),
}

#[test]
fn test() {
    let error1 = ErrorEnum::Tuple1("x".to_string());
    assert_eq!(format!("{:?}", error1), "ErrorEnum::Tuple1(x)");
    assert_eq!(error1.to_string(), "The error message for Tuple1. The placeholders are: x, y, {0}, msg, and {cause}.");

    let error2 = ErrorEnum::Tuple2("x".to_string());
    assert_eq!(format!("{:?}", error2), "ErrorEnum::Tuple2(x)");
    assert_eq!(error2.to_string(), "The error message for Tuple2. The placeholders are: x, 007, 420, 000, and false.");

    let error3 = ErrorEnum::Tuple3(-53);
    assert_eq!(format!("{:?}", error3), "ErrorEnum::Tuple3(-53)");
    assert_eq!(error3.to_string(), "The error message for Tuple3. The placeholders are: , {}, {-53}, 1.5, and -53.");

    let error4 = ErrorEnum::Tuple4(-53);
    assert_eq!(format!("{:?}", error4), "ErrorEnum::Tuple4(-53)");
    assert_eq!(error4.to_string(), "The error message for Tuple4. The placeholders are: -53, -0, -53, and 0.");

    let error5 = ErrorEnum::Tuple5("msg".to_string(), 100487, 0.777);
    assert_eq!(format!("{:?}", error5), "ErrorEnum::Tuple5(msg, 100487, 0.777)");
    assert_eq!(
        error5.to_string(),
        "The error message for Tuple5. The placeholders are: msg, {0}, 0.777, 100487, 0.777, and {--1}."
    );

    let error6 = ErrorEnum::Tuple6("msg".to_string(), 100487, 0.777);
    assert_eq!(format!("{:?}", error6), "ErrorEnum::Tuple6(msg, 100487, 0.777)");
    assert_eq!(error6.to_string(), "The error message for Tuple6. The placeholders are: 100487, {msg}, 0.777, and {1}.");
}
