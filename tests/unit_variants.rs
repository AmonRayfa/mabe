// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe::Mabe;

#[derive(Mabe)]
enum Error {
    #[error("The error message for Unit1. This is a {test}.")]
    Unit1,

    #[error("The error message for Unit2. This is a {{test}}.")]
    #[cause("The cause message for Unit2. This is a {{{test}}}.")]
    Unit2,

    #[cause("The cause message for Unit3. This is a {test.")]
    #[error("The error message for Unit3. This is a test}.")]
    Unit3,

    #[error("The error message for Unit4. This is a {{test.")]
    #[debug("The debug message for Unit4. This is a test}}.")]
    Unit4,

    #[debug("The debug message for Unit5. This is a {{{test.")]
    #[error("The error message for Unit5. This is a test}}}.")]
    Unit5,

    #[error("The error message for Unit6. This is a {{test}.")]
    #[cause("The cause message for Unit6. This is a {test}}.")]
    #[debug("The debug message for Unit6. This is a {{{test}.")]
    Unit6,

    #[debug("The debug message for Unit7. This is a {test}}}.")]
    #[error("The error message for Unit7. This is a {{test}}}.")]
    #[cause("The cause message for Unit7. This is a {{{test}}.")]
    Unit7,

    #[cause("The cause message for Unit8. This is a { test }.")]
    #[debug("The debug message for Unit8. This is a {{ test }}.")]
    #[error("The error message for Unit8. This is a {{{ test }}}.")]
    Unit8,

    #[error("The error message for Unit9. This is a { test{} }.")]
    #[debug("The debug message for Unit9. This is a { {test}{} }.")]
    #[cause("The cause message for Unit9. This is a {{ {test}{} }}.")]
    Unit9,

    #[cause("The cause message for Unit10. This is a {{{test}{{}}}}.")]
    #[error("The error message for Unit10. This is a {{test{}}}.")]
    #[debug("The debug message for Unit10. This is a {{test{{{}}}.")]
    Unit10,

    #[debug("The debug message for Unit11. This is a {{{test}{}.")]
    #[cause("The cause message for Unit11. This is a {test{{}}}}}.")]
    #[error("The error message for Unit11. This is a {{{}{}{}{{}}test{{{}}}.")]
    Unit11,
}

#[test]
fn test() {
    let unit1_error = Error::Unit1;
    assert_eq!(unit1_error.state(), "Error::Unit1");
    assert_eq!(unit1_error.error(), "The error message for Unit1. This is a test.");
    assert_eq!(unit1_error.cause(), "");
    assert_eq!(unit1_error.debug(), "");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(unit1_error.to_string(), "\n[error] The error message for Unit1. This is a test.");

    #[cfg(feature = "colorize")]
    println!("{}", unit1_error);

    let unit2_error = Error::Unit2;
    assert_eq!(unit2_error.state(), "Error::Unit2");
    assert_eq!(unit2_error.error(), "The error message for Unit2. This is a {test}.");
    assert_eq!(unit2_error.cause(), "The cause message for Unit2. This is a {test}.");
    assert_eq!(unit2_error.debug(), "");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(
        unit2_error.to_string(),
        "\n[error] The error message for Unit2. This is a {test}.\n[cause] The cause message for Unit2. This is a {test}."
    );

    #[cfg(feature = "colorize")]
    println!("{}", unit2_error);

    let unit3_error = Error::Unit3;
    assert_eq!(unit3_error.state(), "Error::Unit3");
    assert_eq!(unit3_error.error(), "The error message for Unit3. This is a test}.");
    assert_eq!(unit3_error.cause(), "The cause message for Unit3. This is a {test.");
    assert_eq!(unit3_error.debug(), "");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(
        unit3_error.to_string(),
        "\n[error] The error message for Unit3. This is a test}.\n[cause] The cause message for Unit3. This is a {test."
    );

    #[cfg(feature = "colorize")]
    println!("{}", unit3_error);

    let unit4_error = Error::Unit4;
    assert_eq!(unit4_error.state(), "Error::Unit4");
    assert_eq!(unit4_error.error(), "The error message for Unit4. This is a {test.");
    assert_eq!(unit4_error.cause(), "");
    assert_eq!(unit4_error.debug(), "The debug message for Unit4. This is a test}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(
        unit4_error.to_string(),
        "\n[error] The error message for Unit4. This is a {test.\n[debug] The debug message for Unit4. This is a test}."
    );

    #[cfg(feature = "colorize")]
    println!("{}", unit4_error);

    let unit5_error = Error::Unit5;
    assert_eq!(unit5_error.state(), "Error::Unit5");
    assert_eq!(unit5_error.error(), "The error message for Unit5. This is a test}}.");
    assert_eq!(unit5_error.cause(), "");
    assert_eq!(unit5_error.debug(), "The debug message for Unit5. This is a {{test.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(
        unit5_error.to_string(),
        "\n[error] The error message for Unit5. This is a test}}.\n[debug] The debug message for Unit5. This is a {{test."
    );

    #[cfg(feature = "colorize")]
    println!("{}", unit5_error);

    let unit6_error = Error::Unit6;
    assert_eq!(unit6_error.state(), "Error::Unit6");
    assert_eq!(unit6_error.error(), "The error message for Unit6. This is a {test}.");
    assert_eq!(unit6_error.cause(), "The cause message for Unit6. This is a {test}.");
    assert_eq!(unit6_error.debug(), "The debug message for Unit6. This is a {test.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(unit6_error.to_string(), "\n[error] The error message for Unit6. This is a {test}.\n[cause] The cause message for Unit6. This is a {test}.\n[debug] The debug message for Unit6. This is a {test.");

    #[cfg(feature = "colorize")]
    println!("{}", unit6_error);

    let unit7_error = Error::Unit7;
    assert_eq!(unit7_error.state(), "Error::Unit7");
    assert_eq!(unit7_error.error(), "The error message for Unit7. This is a {test}}.");
    assert_eq!(unit7_error.cause(), "The cause message for Unit7. This is a {{test}.");
    assert_eq!(unit7_error.debug(), "The debug message for Unit7. This is a test}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(unit7_error.to_string(), "\n[error] The error message for Unit7. This is a {test}}.\n[cause] The cause message for Unit7. This is a {{test}.\n[debug] The debug message for Unit7. This is a test}.");

    #[cfg(feature = "colorize")]
    println!("{}", unit7_error);

    let unit8_error = Error::Unit8;
    assert_eq!(unit8_error.state(), "Error::Unit8");
    assert_eq!(unit8_error.error(), "The error message for Unit8. This is a { test }.");
    assert_eq!(unit8_error.cause(), "The cause message for Unit8. This is a  test .");
    assert_eq!(unit8_error.debug(), "The debug message for Unit8. This is a { test }.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(unit8_error.to_string(), "\n[error] The error message for Unit8. This is a { test }.\n[cause] The cause message for Unit8. This is a  test .\n[debug] The debug message for Unit8. This is a { test }.");

    #[cfg(feature = "colorize")]
    println!("{}", unit8_error);

    let unit9_error = Error::Unit9;
    assert_eq!(unit9_error.state(), "Error::Unit9");
    assert_eq!(unit9_error.error(), "The error message for Unit9. This is a  test{ }.");
    assert_eq!(unit9_error.cause(), "The cause message for Unit9. This is a { test }.");
    assert_eq!(unit9_error.debug(), "The debug message for Unit9. This is a  {test }.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(unit9_error.to_string(), "\n[error] The error message for Unit9. This is a  test{ }.\n[cause] The cause message for Unit9. This is a { test }.\n[debug] The debug message for Unit9. This is a  {test }.");

    #[cfg(feature = "colorize")]
    println!("{}", unit9_error);

    let unit10_error = Error::Unit10;
    assert_eq!(unit10_error.state(), "Error::Unit10");
    assert_eq!(unit10_error.error(), "The error message for Unit10. This is a {test}.");
    assert_eq!(unit10_error.cause(), "The cause message for Unit10. This is a {test{}}.");
    assert_eq!(unit10_error.debug(), "The debug message for Unit10. This is a {test{}.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(unit10_error.to_string(), "\n[error] The error message for Unit10. This is a {test}.\n[cause] The cause message for Unit10. This is a {test{}}.\n[debug] The debug message for Unit10. This is a {test{}.");

    #[cfg(feature = "colorize")]
    println!("{}", unit10_error);

    let unit11_error = Error::Unit11;
    assert_eq!(unit11_error.state(), "Error::Unit11");
    assert_eq!(unit11_error.error(), "The error message for Unit11. This is a {{}test{}.");
    assert_eq!(unit11_error.cause(), "The cause message for Unit11. This is a test{{}}.");
    assert_eq!(unit11_error.debug(), "The debug message for Unit11. This is a {test.");

    #[cfg(not(feature = "colorize"))]
    assert_eq!(unit11_error.to_string(), "\n[error] The error message for Unit11. This is a {{}test{}.\n[cause] The cause message for Unit11. This is a test{{}}.\n[debug] The debug message for Unit11. This is a {test.");

    #[cfg(feature = "colorize")]
    println!("{}", unit11_error);
}
