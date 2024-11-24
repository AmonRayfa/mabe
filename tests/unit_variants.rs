// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe::Mabe;

#[derive(Mabe)]
enum MyError {
    Error1,

    #[error("The error message for Error2. This is a {test}.")]
    Error2,

    #[reason("The reason message for Error3. This is a {{test}}.")]
    Error3,

    #[solution("The solution message for Error4. This is a {{{test}}}")]
    Error4,

    #[error("The error message for Error5. This is a {test.")]
    #[reason("The reason message for Error5. This is a test}.")]
    Error5,

    #[error("The error message for Error6. This is a {{test.")]
    #[solution("The solution message for Error6. This is a test}}.")]
    Error6,

    #[reason("The reason message for Error7. This is a {{{test.")]
    #[solution("The solution message for Error7. This is a test}}}.")]
    Error7,

    #[error("The error message for Error8. This is a {{test}.")]
    #[reason("The reason message for Error8. This is a {test}}.")]
    #[solution("The solution message for Error8. This is a {{{test}.")]
    Error8,

    #[solution("The solution message for Error9. This is a {test}}}.")]
    #[error("The error message for Error9. This is a {{test}}}.")]
    #[reason("The reason message for Error9. This is a {{{test}}.")]
    Error9,

    #[reason("The reason message for Error10. This is a { test }.")]
    #[solution("The solution message for Error10. This is a {{ test }}.")]
    #[error("The error message for Error10. This is a {{{ test }}}.")]
    Error10,

    #[error("The error message for Error11. This is a { test{} }.")]
    #[solution("The solution message for Error11. This is a { {test}{} }.")]
    #[reason("The reason message for Error11. This is a {{ {test}{} }}.")]
    Error11,

    #[reason("The reason message for Error12. This is a {{{test}{{}}}}.")]
    #[error("The error message for Error12. This is a {{test{}}}.")]
    #[solution("The solution message for Error12. This is a {{test{{{}}}.")]
    Error12,

    #[solution("The solution message for Error13. This is a {{{test}{}.")]
    #[reason("The reason message for Error13. This is a {test{{}}}}}.")]
    #[error("The error message for Error13. This is a {{{}{}{}{{}}test{{{}}}.")]
    Error13,
}

#[test]
fn test() {
    let error1 = MyError::Error1;
    assert_eq!(error1.debug(), "MyError::Error1");
    assert_eq!(error1.error(), "");
    assert_eq!(error1.reason(), "");
    assert_eq!(error1.solution(), "");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error1.to_string(), "\n[error] \n[reason] \n[solution] ");

    #[cfg(feature = "colorize")]
    println!("{}", error1);

    let error2 = MyError::Error2;
    assert_eq!(error2.debug(), "MyError::Error2");
    assert_eq!(error2.error(), "The error message for Error2. This is a test.");
    assert_eq!(error2.reason(), "");
    assert_eq!(error2.solution(), "");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error2.to_string(), "\n[error] The error message for Error2. This is a test.\n[reason] \n[solution] ");

    #[cfg(feature = "colorize")]
    println!("{}", error2);

    let error3 = MyError::Error3;
    assert_eq!(error3.debug(), "MyError::Error3");
    assert_eq!(error3.error(), "");
    assert_eq!(error3.reason(), "The reason message for Error3. This is a {test}.");
    assert_eq!(error3.solution(), "");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error3.to_string(), "\n[error] \n[reason] The reason message for Error3. This is a {test}.\n[solution] ");

    #[cfg(feature = "colorize")]
    println!("{}", error3);

    let error4 = MyError::Error4;
    assert_eq!(error4.debug(), "MyError::Error4");
    assert_eq!(error4.error(), "");
    assert_eq!(error4.reason(), "");
    assert_eq!(error4.solution(), "The solution message for Error4. This is a {test}");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error4.to_string(), "\n[error] \n[reason] \n[solution] The solution message for Error4. This is a {test}");

    #[cfg(feature = "colorize")]
    println!("{}", error4);

    let error5 = MyError::Error5;
    assert_eq!(error5.debug(), "MyError::Error5");
    assert_eq!(error5.error(), "The error message for Error5. This is a {test.");
    assert_eq!(error5.reason(), "The reason message for Error5. This is a test}.");
    assert_eq!(error5.solution(), "");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error5.to_string(), "\n[error] The error message for Error5. This is a {test.\n[reason] The reason message for Error5. This is a test}.\n[solution] ");

    #[cfg(feature = "colorize")]
    println!("{}", error5);

    let error6 = MyError::Error6;
    assert_eq!(error6.debug(), "MyError::Error6");
    assert_eq!(error6.error(), "The error message for Error6. This is a {test.");
    assert_eq!(error6.reason(), "");
    assert_eq!(error6.solution(), "The solution message for Error6. This is a test}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error6.to_string(), "\n[error] The error message for Error6. This is a {test.\n[reason] \n[solution] The solution message for Error6. This is a test}.");

    #[cfg(feature = "colorize")]
    println!("{}", error6);

    let error7 = MyError::Error7;
    assert_eq!(error7.debug(), "MyError::Error7");
    assert_eq!(error7.error(), "");
    assert_eq!(error7.reason(), "The reason message for Error7. This is a {{test.");
    assert_eq!(error7.solution(), "The solution message for Error7. This is a test}}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error7.to_string(), "\n[error] \n[reason] The reason message for Error7. This is a {{test.\n[solution] The solution message for Error7. This is a test}}.");

    #[cfg(feature = "colorize")]
    println!("{}", error7);

    let error8 = MyError::Error8;
    assert_eq!(error8.debug(), "MyError::Error8");
    assert_eq!(error8.error(), "The error message for Error8. This is a {test}.");
    assert_eq!(error8.reason(), "The reason message for Error8. This is a {test}.");
    assert_eq!(error8.solution(), "The solution message for Error8. This is a {test.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error8.to_string(), "\n[error] The error message for Error8. This is a {test}.\n[reason] The reason message for Error8. This is a {test}.\n[solution] The solution message for Error8. This is a {test.");

    #[cfg(feature = "colorize")]
    println!("{}", error8);

    let error9 = MyError::Error9;
    assert_eq!(error9.debug(), "MyError::Error9");
    assert_eq!(error9.error(), "The error message for Error9. This is a {test}}.");
    assert_eq!(error9.reason(), "The reason message for Error9. This is a {{test}.");
    assert_eq!(error9.solution(), "The solution message for Error9. This is a test}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error9.to_string(), "\n[error] The error message for Error9. This is a {test}}.\n[reason] The reason message for Error9. This is a {{test}.\n[solution] The solution message for Error9. This is a test}.");

    #[cfg(feature = "colorize")]
    println!("{}", error9);

    let error10 = MyError::Error10;
    assert_eq!(error10.debug(), "MyError::Error10");
    assert_eq!(error10.error(), "The error message for Error10. This is a { test }.");
    assert_eq!(error10.reason(), "The reason message for Error10. This is a  test .");
    assert_eq!(error10.solution(), "The solution message for Error10. This is a { test }.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error10.to_string(), "\n[error] The error message for Error10. This is a { test }.\n[reason] The reason message for Error10. This is a  test .\n[solution] The solution message for Error10. This is a { test }.");

    #[cfg(feature = "colorize")]
    println!("{}", error10);

    let error11 = MyError::Error11;
    assert_eq!(error11.debug(), "MyError::Error11");
    assert_eq!(error11.error(), "The error message for Error11. This is a  test{ }.");
    assert_eq!(error11.reason(), "The reason message for Error11. This is a { test }.");
    assert_eq!(error11.solution(), "The solution message for Error11. This is a  {test }.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error11.to_string(), "\n[error] The error message for Error11. This is a  test{ }.\n[reason] The reason message for Error11. This is a { test }.\n[solution] The solution message for Error11. This is a  {test }.");

    #[cfg(feature = "colorize")]
    println!("{}", error11);

    let error12 = MyError::Error12;
    assert_eq!(error12.debug(), "MyError::Error12");
    assert_eq!(error12.error(), "The error message for Error12. This is a {test}.");
    assert_eq!(error12.reason(), "The reason message for Error12. This is a {test{}}.");
    assert_eq!(error12.solution(), "The solution message for Error12. This is a {test{}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error12.to_string(), "\n[error] The error message for Error12. This is a {test}.\n[reason] The reason message for Error12. This is a {test{}}.\n[solution] The solution message for Error12. This is a {test{}.");

    #[cfg(feature = "colorize")]
    println!("{}", error12);

    let error13 = MyError::Error13;
    assert_eq!(error13.debug(), "MyError::Error13");
    assert_eq!(error13.error(), "The error message for Error13. This is a {{}test{}.");
    assert_eq!(error13.reason(), "The reason message for Error13. This is a test{{}}.");
    assert_eq!(error13.solution(), "The solution message for Error13. This is a {test.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error13.to_string(), "\n[error] The error message for Error13. This is a {{}test{}.\n[reason] The reason message for Error13. This is a test{{}}.\n[solution] The solution message for Error13. This is a {test.");

    #[cfg(feature = "colorize")]
    println!("{}", error13);
}
