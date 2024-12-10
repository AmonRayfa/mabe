// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe::Mabe;

#[derive(Mabe)]
enum MyError {
    #[error("The error message for Error1. This is a {test}.")]
    Error1,

    #[error("The error message for Error2. This is a {{test}}.")]
    #[cause("The cause message for Error2. This is a {{{test}}}.")]
    Error2,

    #[cause("The cause message for Error3. This is a {test.")]
    #[error("The error message for Error3. This is a test}.")]
    Error3,

    #[error("The error message for Error4. This is a {{test.")]
    #[debug("The debug message for Error4. This is a test}}.")]
    Error4,

    #[debug("The debug message for Error5. This is a {{{test.")]
    #[error("The error message for Error5. This is a test}}}.")]
    Error5,

    #[error("The error message for Error6. This is a {{test}.")]
    #[cause("The cause message for Error6. This is a {test}}.")]
    #[debug("The debug message for Error6. This is a {{{test}.")]
    Error6,

    #[debug("The debug message for Error7. This is a {test}}}.")]
    #[error("The error message for Error7. This is a {{test}}}.")]
    #[cause("The cause message for Error7. This is a {{{test}}.")]
    Error7,

    #[cause("The cause message for Error8. This is a { test }.")]
    #[debug("The debug message for Error8. This is a {{ test }}.")]
    #[error("The error message for Error8. This is a {{{ test }}}.")]
    Error8,

    #[error("The error message for Error9. This is a { test{} }.")]
    #[debug("The debug message for Error9. This is a { {test}{} }.")]
    #[cause("The cause message for Error9. This is a {{ {test}{} }}.")]
    Error9,

    #[cause("The cause message for Error10. This is a {{{test}{{}}}}.")]
    #[error("The error message for Error10. This is a {{test{}}}.")]
    #[debug("The debug message for Error10. This is a {{test{{{}}}.")]
    Error10,

    #[debug("The debug message for Error11. This is a {{{test}{}.")]
    #[cause("The cause message for Error11. This is a {test{{}}}}}.")]
    #[error("The error message for Error11. This is a {{{}{}{}{{}}test{{{}}}.")]
    Error11,
}

#[test]
fn test() {
    let error1 = MyError::Error1;
    assert_eq!(error1.state(), "MyError::Error1");
    assert_eq!(error1.error(), "The error message for Error1. This is a test.");
    assert_eq!(error1.cause(), "");
    assert_eq!(error1.debug(), "");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error1.to_string(), "\n[error] The error message for Error1. This is a test.");

    #[cfg(feature = "colorize")]
    println!("{}", error1);

    let error2 = MyError::Error2;
    assert_eq!(error2.state(), "MyError::Error2");
    assert_eq!(error2.error(), "The error message for Error2. This is a {test}.");
    assert_eq!(error2.cause(), "The cause message for Error2. This is a {test}.");
    assert_eq!(error2.debug(), "");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(
        error2.to_string(),
        "\n[error] The error message for Error2. This is a {test}.\n[cause] The cause message for Error2. This is a {test}."
    );

    #[cfg(feature = "colorize")]
    println!("{}", error2);

    let error3 = MyError::Error3;
    assert_eq!(error3.state(), "MyError::Error3");
    assert_eq!(error3.error(), "The error message for Error3. This is a test}.");
    assert_eq!(error3.cause(), "The cause message for Error3. This is a {test.");
    assert_eq!(error3.debug(), "");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(
        error3.to_string(),
        "\n[error] The error message for Error3. This is a test}.\n[cause] The cause message for Error3. This is a {test."
    );

    #[cfg(feature = "colorize")]
    println!("{}", error3);

    let error4 = MyError::Error4;
    assert_eq!(error4.state(), "MyError::Error4");
    assert_eq!(error4.error(), "The error message for Error4. This is a {test.");
    assert_eq!(error4.cause(), "");
    assert_eq!(error4.debug(), "The debug message for Error4. This is a test}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(
        error4.to_string(),
        "\n[error] The error message for Error4. This is a {test.\n[debug] The debug message for Error4. This is a test}."
    );

    #[cfg(feature = "colorize")]
    println!("{}", error4);

    let error5 = MyError::Error5;
    assert_eq!(error5.state(), "MyError::Error5");
    assert_eq!(error5.error(), "The error message for Error5. This is a test}}.");
    assert_eq!(error5.cause(), "");
    assert_eq!(error5.debug(), "The debug message for Error5. This is a {{test.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(
        error5.to_string(),
        "\n[error] The error message for Error5. This is a test}}.\n[debug] The debug message for Error5. This is a {{test."
    );

    #[cfg(feature = "colorize")]
    println!("{}", error5);

    let error6 = MyError::Error6;
    assert_eq!(error6.state(), "MyError::Error6");
    assert_eq!(error6.error(), "The error message for Error6. This is a {test}.");
    assert_eq!(error6.cause(), "The cause message for Error6. This is a {test}.");
    assert_eq!(error6.debug(), "The debug message for Error6. This is a {test.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error6.to_string(), "\n[error] The error message for Error6. This is a {test}.\n[cause] The cause message for Error6. This is a {test}.\n[debug] The debug message for Error6. This is a {test.");

    #[cfg(feature = "colorize")]
    println!("{}", error6);

    let error7 = MyError::Error7;
    assert_eq!(error7.state(), "MyError::Error7");
    assert_eq!(error7.error(), "The error message for Error7. This is a {test}}.");
    assert_eq!(error7.cause(), "The cause message for Error7. This is a {{test}.");
    assert_eq!(error7.debug(), "The debug message for Error7. This is a test}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error7.to_string(), "\n[error] The error message for Error7. This is a {test}}.\n[cause] The cause message for Error7. This is a {{test}.\n[debug] The debug message for Error7. This is a test}.");

    #[cfg(feature = "colorize")]
    println!("{}", error7);

    let error8 = MyError::Error8;
    assert_eq!(error8.state(), "MyError::Error8");
    assert_eq!(error8.error(), "The error message for Error8. This is a { test }.");
    assert_eq!(error8.cause(), "The cause message for Error8. This is a  test .");
    assert_eq!(error8.debug(), "The debug message for Error8. This is a { test }.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error8.to_string(), "\n[error] The error message for Error8. This is a { test }.\n[cause] The cause message for Error8. This is a  test .\n[debug] The debug message for Error8. This is a { test }.");

    #[cfg(feature = "colorize")]
    println!("{}", error8);

    let error9 = MyError::Error9;
    assert_eq!(error9.state(), "MyError::Error9");
    assert_eq!(error9.error(), "The error message for Error9. This is a  test{ }.");
    assert_eq!(error9.cause(), "The cause message for Error9. This is a { test }.");
    assert_eq!(error9.debug(), "The debug message for Error9. This is a  {test }.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error9.to_string(), "\n[error] The error message for Error9. This is a  test{ }.\n[cause] The cause message for Error9. This is a { test }.\n[debug] The debug message for Error9. This is a  {test }.");

    #[cfg(feature = "colorize")]
    println!("{}", error9);

    let error10 = MyError::Error10;
    assert_eq!(error10.state(), "MyError::Error10");
    assert_eq!(error10.error(), "The error message for Error10. This is a {test}.");
    assert_eq!(error10.cause(), "The cause message for Error10. This is a {test{}}.");
    assert_eq!(error10.debug(), "The debug message for Error10. This is a {test{}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error10.to_string(), "\n[error] The error message for Error10. This is a {test}.\n[cause] The cause message for Error10. This is a {test{}}.\n[debug] The debug message for Error10. This is a {test{}.");

    #[cfg(feature = "colorize")]
    println!("{}", error10);

    let error11 = MyError::Error11;
    assert_eq!(error11.state(), "MyError::Error11");
    assert_eq!(error11.error(), "The error message for Error11. This is a {{}test{}.");
    assert_eq!(error11.cause(), "The cause message for Error11. This is a test{{}}.");
    assert_eq!(error11.debug(), "The debug message for Error11. This is a {test.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error11.to_string(), "\n[error] The error message for Error11. This is a {{}test{}.\n[cause] The cause message for Error11. This is a test{{}}.\n[debug] The debug message for Error11. This is a {test.");

    #[cfg(feature = "colorize")]
    println!("{}", error11);
}
