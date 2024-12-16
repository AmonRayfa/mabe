// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe::Mabe;

#[derive(Mabe)]
enum Error {
    #[error("The error message for Unit1. This is a {test}.")]
    Unit1,

    #[error("The error message for Unit2. This is a {{test}}.")]
    #[debug("The debug message for Unit2. This is a {{{test}}}.")]
    Unit2,

    #[debug("The debug message for Unit3. This is a {test.")]
    #[error("The error message for Unit3. This is a test}.")]
    Unit3,

    #[error("The error message for Unit4. This is a {{test.")]
    #[debug("The debug message for Unit4. This is a test}}.")]
    Unit4,

    #[debug("The debug message for Unit5. This is a {{{test.")]
    #[error("The error message for Unit5. This is a test}}}.")]
    Unit5,

    #[error("The error message for Unit6. This is a {{test}.")]
    #[debug("The debug message for Unit6. This is a {test}} and a {{{test}.")]
    Unit6,

    #[debug("The debug message for Unit7. This is a {test}}}.")]
    #[error("The error message for Unit7. This is a {{{test}} and a {{test}}}.")]
    Unit7,

    #[debug("The debug message for Unit8. This is a { test } and a {{ test }}.")]
    #[error("The error message for Unit8. This is a {{{ test }}}.")]
    Unit8,

    #[error("The error message for Unit9. This is a { test{} }.")]
    #[debug("The debug message for Unit9. This is a { {test}{} } and a {{ {test}{} }}.")]
    Unit9,

    #[error("The error message for Unit10. This is a {{{test}{{}}}} and a {{test{}}}.")]
    #[debug("The debug message for Unit10. This is a {{test{{{}}}.")]
    Unit10,

    #[debug("The debug message for Unit11. This is a {{{test}{} and a {test{{}}}}}.")]
    #[error("The error message for Unit11. This is a {{{}{}{}{{}}test{{{}}}.")]
    Unit11,
}

#[test]
fn test() {
    let error1 = Error::Unit1;
    assert_eq!(error1.state(), "Error::Unit1");
    assert_eq!(error1.error(), "The error message for Unit1. This is a test.");
    assert_eq!(error1.debug(), "");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error1.to_string(), "\n[error] The error message for Unit1. This is a test.");

    println!("{}", error1);

    let error2 = Error::Unit2;
    assert_eq!(error2.state(), "Error::Unit2");
    assert_eq!(error2.error(), "The error message for Unit2. This is a {test}.");
    assert_eq!(error2.debug(), "The debug message for Unit2. This is a {test}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(
        error2.to_string(),
        "\n[error] The error message for Unit2. This is a {test}.\n[debug] The debug message for Unit2. This is a {test}."
    );

    println!("{}", error2);

    let error3 = Error::Unit3;
    assert_eq!(error3.state(), "Error::Unit3");
    assert_eq!(error3.error(), "The error message for Unit3. This is a test}.");
    assert_eq!(error3.debug(), "The debug message for Unit3. This is a {test.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(
        error3.to_string(),
        "\n[error] The error message for Unit3. This is a test}.\n[debug] The debug message for Unit3. This is a {test."
    );

    println!("{}", error3);

    let error4 = Error::Unit4;
    assert_eq!(error4.state(), "Error::Unit4");
    assert_eq!(error4.error(), "The error message for Unit4. This is a {test.");
    assert_eq!(error4.debug(), "The debug message for Unit4. This is a test}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(
        error4.to_string(),
        "\n[error] The error message for Unit4. This is a {test.\n[debug] The debug message for Unit4. This is a test}."
    );

    println!("{}", error4);

    let error5 = Error::Unit5;
    assert_eq!(error5.state(), "Error::Unit5");
    assert_eq!(error5.error(), "The error message for Unit5. This is a test}}.");
    assert_eq!(error5.debug(), "The debug message for Unit5. This is a {{test.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(
        error5.to_string(),
        "\n[error] The error message for Unit5. This is a test}}.\n[debug] The debug message for Unit5. This is a {{test."
    );

    println!("{}", error5);

    let error6 = Error::Unit6;
    assert_eq!(error6.state(), "Error::Unit6");
    assert_eq!(error6.error(), "The error message for Unit6. This is a {test}.");
    assert_eq!(error6.debug(), "The debug message for Unit6. This is a test}} and a {{{test.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(
        error6.to_string(),
        "\n[error] The error message for Unit6. This is a {test}.\n[debug] The debug message for Unit6. This is a test}} and a {{{test."
    );

    println!("{}", error6);

    let error7 = Error::Unit7;
    assert_eq!(error7.state(), "Error::Unit7");
    assert_eq!(error7.error(), "The error message for Unit7. This is a {test}} and a {{test}.");
    assert_eq!(error7.debug(), "The debug message for Unit7. This is a test}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error7.to_string(), "\n[error] The error message for Unit7. This is a {test}} and a {{test}.\n[debug] The debug message for Unit7. This is a test}.");

    println!("{}", error7);

    let error8 = Error::Unit8;
    assert_eq!(error8.state(), "Error::Unit8");
    assert_eq!(error8.error(), "The error message for Unit8. This is a { test }.");
    assert_eq!(error8.debug(), "The debug message for Unit8. This is a  test  and a { test }.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error8.to_string(), "\n[error] The error message for Unit8. This is a { test }.\n[debug] The debug message for Unit8. This is a  test  and a { test }.");

    println!("{}", error8);

    let error9 = Error::Unit9;
    assert_eq!(error9.state(), "Error::Unit9");
    assert_eq!(error9.error(), "The error message for Unit9. This is a  test{ }.");
    assert_eq!(error9.debug(), "The debug message for Unit9. This is a  {test } and a { test }.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error9.to_string(), "\n[error] The error message for Unit9. This is a  test{ }.\n[debug] The debug message for Unit9. This is a  {test } and a { test }.");

    println!("{}", error9);

    let error10 = Error::Unit10;
    assert_eq!(error10.state(), "Error::Unit10");
    assert_eq!(error10.error(), "The error message for Unit10. This is a {test{}} and a {test}.");
    assert_eq!(error10.debug(), "The debug message for Unit10. This is a {test{}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error10.to_string(), "\n[error] The error message for Unit10. This is a {test{}} and a {test}.\n[debug] The debug message for Unit10. This is a {test{}.");

    println!("{}", error10);

    let error11 = Error::Unit11;
    assert_eq!(error11.state(), "Error::Unit11");
    assert_eq!(error11.error(), "The error message for Unit11. This is a {{}test{}.");
    assert_eq!(error11.debug(), "The debug message for Unit11. This is a {test and a test{{}}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(error11.to_string(), "\n[error] The error message for Unit11. This is a {{}test{}.\n[debug] The debug message for Unit11. This is a {test and a test{{}}.");

    println!("{}", error11);
}
