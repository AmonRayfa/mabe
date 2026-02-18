// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe_derive::Error;

#[derive(Error)]
enum ErrorEnum {
    #[error("The error message for Struct1. The placeholders are: {0}, {msg}, {{0}}, {None}, and {{{None}}}.")]
    Struct1 { msg: String },

    #[error("The error message for Struct2. The placeholders are: {msg}, {007}, {420}, {000}, and false.")]
    Struct2 { msg: String },

    #[error("The error message for Struct3. The placeholders are: {}, {{}}, {{{msg}}}, {speed}, and {height}.")]
    Struct3 { speed: i32 },

    #[error("The error message for Struct4. The placeholders are: {speed}, {comment}, {-0}, {0}, and 0.")]
    Struct4 { speed: i32 },

    #[error("The error message for Struct5. The placeholders are: {comment}, {{0}}, {ratio}, {height}, {2}, and {{{--1}}}.")]
    Struct5 { comment: String, height: usize, ratio: f64 },

    #[error("The error message for Struct6. The placeholders are: {height}, {comment}, {{{0}}}, {ratio}, and {{ratio}}.")]
    Struct6 { comment: String, height: usize, ratio: f64 },
}

#[test]
fn test() {
    let error1 = ErrorEnum::Struct1 { msg: "Something...".to_string() };
    assert_eq!(format!("{:?}", error1), "ErrorEnum::Struct1 { msg: Something... }");
    assert_eq!(
        error1.to_string(),
        "The error message for Struct1. The placeholders are: 0, Something..., {0}, None, and {None}."
    );

    let error2 = ErrorEnum::Struct2 { msg: "Something...".to_string() };
    assert_eq!(format!("{:?}", error2), "ErrorEnum::Struct2 { msg: Something... }");
    assert_eq!(
        error2.to_string(),
        "The error message for Struct2. The placeholders are: Something..., 007, 420, 000, and false."
    );

    let error3 = ErrorEnum::Struct3 { speed: 100 };
    assert_eq!(format!("{:?}", error3), "ErrorEnum::Struct3 { speed: 100 }");
    assert_eq!(error3.to_string(), "The error message for Struct3. The placeholders are: , {}, {msg}, 100, and height.");

    let error4 = ErrorEnum::Struct4 { speed: 100 };
    assert_eq!(format!("{:?}", error4), "ErrorEnum::Struct4 { speed: 100 }");
    assert_eq!(error4.to_string(), "The error message for Struct4. The placeholders are: 100, comment, -0, 0, and 0.");

    let error5 = ErrorEnum::Struct5 { comment: "msg".to_string(), height: 100487, ratio: 1.333 };
    assert_eq!(format!("{:?}", error5), "ErrorEnum::Struct5 { comment: msg, height: 100487, ratio: 1.333 }");
    assert_eq!(
        error5.to_string(),
        "The error message for Struct5. The placeholders are: msg, {0}, 1.333, 100487, 2, and {--1}."
    );

    let error6 = ErrorEnum::Struct6 { comment: "msg".to_string(), height: 100487, ratio: 1.333 };
    assert_eq!(format!("{:?}", error6), "ErrorEnum::Struct6 { comment: msg, height: 100487, ratio: 1.333 }");
    assert_eq!(
        error6.to_string(),
        "The error message for Struct6. The placeholders are: 100487, msg, {0}, 1.333, and {ratio}."
    );
}
