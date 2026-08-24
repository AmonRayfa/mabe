// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use mabe_derive::Error;

#[derive(Error)]
enum ErrorEnum {
    #[error("The café is closed. Try the {{café}} around the corner.")]
    NonAscii1,

    #[error("Ambiguïté détectée — {{code}} inconnu pour {name}.")]
    NonAscii2 { name: String },

    #[error("温度が {0}°C を超えました。")]
    NonAscii3(i32),

    #[error("Résumé: {{{status}}} at {pct}% 🚀.")]
    NonAscii4 { status: String, pct: u8 },
}

#[test]
fn test() {
    let error1 = ErrorEnum::NonAscii1;
    assert_eq!(format!("{:?}", error1), "ErrorEnum::NonAscii1");
    assert_eq!(error1.to_string(), "The café is closed. Try the {café} around the corner.");

    let error2 = ErrorEnum::NonAscii2 { name: "Máni".to_string() };
    assert_eq!(format!("{:?}", error2), "ErrorEnum::NonAscii2 { name: Máni }");
    assert_eq!(error2.to_string(), "Ambiguïté détectée — {code} inconnu pour Máni.");

    let error3 = ErrorEnum::NonAscii3(-40);
    assert_eq!(format!("{:?}", error3), "ErrorEnum::NonAscii3(-40)");
    assert_eq!(error3.to_string(), "温度が -40°C を超えました。");

    let error4 = ErrorEnum::NonAscii4 { status: "prêt".to_string(), pct: 99 };
    assert_eq!(format!("{:?}", error4), "ErrorEnum::NonAscii4 { status: prêt, pct: 99 }");
    assert_eq!(error4.to_string(), "Résumé: {prêt} at 99% 🚀.");
}
