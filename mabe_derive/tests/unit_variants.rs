// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe_derive::Error;

#[derive(Error)]
enum ErrorEnum {
    #[error("The error message for Unit1. This is a {test}.")]
    Unit1,

    #[error("The error message for Unit2. This is a {{test}}.")]
    Unit2,

    #[error("The error message for Unit3. This is a {{{test}}}.")]
    Unit3,

    #[error("The error message for Unit4. This is a {test.")]
    Unit4,

    #[error("The error message for Unit5. This is a test}.")]
    Unit5,

    #[error("The error message for Unit6. This is a {{test.")]
    Unit6,

    #[error("The error message for Unit7. This is a test}}.")]
    Unit7,

    #[error("The error message for Unit8. This is a {{{test.")]
    Unit8,

    #[error("The error message for Unit9. This is a test}}}.")]
    Unit9,

    #[error("The error message for Unit10. This is a {{test}.")]
    Unit10,

    #[error("The error message for Unit11. This is a {test}} and a {{{test}.")]
    Unit11,

    #[error("The error message for Unit12. This is a {test}}}.")]
    Unit12,

    #[error("The error message for Unit13. This is a {{{test}} and a {{test}}}.")]
    Unit13,

    #[error("The error message for Unit14. This is a { test } and a {{ test }}.")]
    Unit14,

    #[error("The error message for Unit15. This is a {{{ test }}}.")]
    Unit15,

    #[error("The error message for Unit16. This is a { test{} }.")]
    Unit16,

    #[error("The error message for Unit17. This is a { {test}{} } and a {{ {test}{} }}.")]
    Unit17,

    #[error("The error message for Unit18. This is a {{{test}{{}}}} and a {{test{}}}.")]
    Unit18,

    #[error("The error message for Unit19. This is a {{test{{{}}}.")]
    Unit19,

    #[error("The error message for Unit20. This is a {{{test}{} and a {test{{}}}}}.")]
    Unit20,

    #[error("The error message for Unit21. This is a {{{}{}{}{{}}test{{{}}}.")]
    Unit21,
}

#[test]
fn test() {
    let error1 = ErrorEnum::Unit1;
    assert_eq!(format!("{:?}", error1), "ErrorEnum::Unit1");
    assert_eq!(error1.to_string(), "The error message for Unit1. This is a test.");

    let error2 = ErrorEnum::Unit2;
    assert_eq!(format!("{:?}", error2), "ErrorEnum::Unit2");
    assert_eq!(error2.to_string(), "The error message for Unit2. This is a {test}.");

    let error3 = ErrorEnum::Unit3;
    assert_eq!(format!("{:?}", error3), "ErrorEnum::Unit3");
    assert_eq!(error3.to_string(), "The error message for Unit3. This is a {test}.");

    let error4 = ErrorEnum::Unit4;
    assert_eq!(format!("{:?}", error4), "ErrorEnum::Unit4");
    assert_eq!(error4.to_string(), "The error message for Unit4. This is a {test.");

    let error5 = ErrorEnum::Unit5;
    assert_eq!(format!("{:?}", error5), "ErrorEnum::Unit5");
    assert_eq!(error5.to_string(), "The error message for Unit5. This is a test}.");

    let error6 = ErrorEnum::Unit6;
    assert_eq!(format!("{:?}", error6), "ErrorEnum::Unit6");
    assert_eq!(error6.to_string(), "The error message for Unit6. This is a {test.");

    let error7 = ErrorEnum::Unit7;
    assert_eq!(format!("{:?}", error7), "ErrorEnum::Unit7");
    assert_eq!(error7.to_string(), "The error message for Unit7. This is a test}.");

    let error8 = ErrorEnum::Unit8;
    assert_eq!(format!("{:?}", error8), "ErrorEnum::Unit8");
    assert_eq!(error8.to_string(), "The error message for Unit8. This is a {{test.");

    let error9 = ErrorEnum::Unit9;
    assert_eq!(format!("{:?}", error9), "ErrorEnum::Unit9");
    assert_eq!(error9.to_string(), "The error message for Unit9. This is a test}}.");

    let error10 = ErrorEnum::Unit10;
    assert_eq!(format!("{:?}", error10), "ErrorEnum::Unit10");
    assert_eq!(error10.to_string(), "The error message for Unit10. This is a {test}.");

    let error11 = ErrorEnum::Unit11;
    assert_eq!(format!("{:?}", error11), "ErrorEnum::Unit11");
    assert_eq!(error11.to_string(), "The error message for Unit11. This is a test}} and a {{{test.");

    let error12 = ErrorEnum::Unit12;
    assert_eq!(format!("{:?}", error12), "ErrorEnum::Unit12");
    assert_eq!(error12.to_string(), "The error message for Unit12. This is a test}.");

    let error13 = ErrorEnum::Unit13;
    assert_eq!(format!("{:?}", error13), "ErrorEnum::Unit13");
    assert_eq!(error13.to_string(), "The error message for Unit13. This is a {test}} and a {{test}.");

    let error14 = ErrorEnum::Unit14;
    assert_eq!(format!("{:?}", error14), "ErrorEnum::Unit14");
    assert_eq!(error14.to_string(), "The error message for Unit14. This is a  test  and a { test }.");

    let error15 = ErrorEnum::Unit15;
    assert_eq!(format!("{:?}", error15), "ErrorEnum::Unit15");
    assert_eq!(error15.to_string(), "The error message for Unit15. This is a { test }.");

    let error16 = ErrorEnum::Unit16;
    assert_eq!(format!("{:?}", error16), "ErrorEnum::Unit16");
    assert_eq!(error16.to_string(), "The error message for Unit16. This is a  test{ }.");

    let error17 = ErrorEnum::Unit17;
    assert_eq!(format!("{:?}", error17), "ErrorEnum::Unit17");
    assert_eq!(error17.to_string(), "The error message for Unit17. This is a  {test } and a { test }.");

    let error18 = ErrorEnum::Unit18;
    assert_eq!(format!("{:?}", error18), "ErrorEnum::Unit18");
    assert_eq!(error18.to_string(), "The error message for Unit18. This is a {test{}} and a {test}.");

    let error19 = ErrorEnum::Unit19;
    assert_eq!(format!("{:?}", error19), "ErrorEnum::Unit19");
    assert_eq!(error19.to_string(), "The error message for Unit19. This is a {test{}.");

    let error20 = ErrorEnum::Unit20;
    assert_eq!(format!("{:?}", error20), "ErrorEnum::Unit20");
    assert_eq!(error20.to_string(), "The error message for Unit20. This is a {test and a test{{}}.");

    let error21 = ErrorEnum::Unit21;
    assert_eq!(format!("{:?}", error21), "ErrorEnum::Unit21");
    assert_eq!(error21.to_string(), "The error message for Unit21. This is a {{}test{}.");
}
