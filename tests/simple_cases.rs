// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe::Mabe;

#[derive(Debug, Mabe)]
enum MyError {
    Error1,

    #[error("The error message for Error2.")]
    Error2,

    #[reason("The reason message for Error3.")]
    Error3,

    #[solution("The solution message for Error4.")]
    Error4,

    #[error("The error message for Error5.")]
    #[reason("The reason message for Error5.")]
    #[solution("The solution message for Error5.")]
    Error5,
}

#[test]
fn test() {
    let error1 = MyError::Error1;
    assert!(error1.error() == "");
    assert!(error1.reason() == "");
    assert!(error1.solution() == "");

    let error2 = MyError::Error2;
    assert!(error2.error() == "The error message for Error2.");
    assert!(error2.reason() == "");
    assert!(error2.solution() == "");

    let error3 = MyError::Error3;
    assert!(error3.error() == "");
    assert!(error3.reason() == "The reason message for Error3.");
    assert!(error3.solution() == "");

    let error4 = MyError::Error4;
    assert!(error4.error() == "");
    assert!(error4.reason() == "");
    assert!(error4.solution() == "The solution message for Error4.");

    let error5 = MyError::Error5;
    assert!(error5.error() == "The error message for Error5.");
    assert!(error5.reason() == "The reason message for Error5.");
    assert!(error5.solution() == "The solution message for Error5.");
}
