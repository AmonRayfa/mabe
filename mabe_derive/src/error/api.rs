// Copyright 2026 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use syn::Ident;

/// The custom error type for the `api` module.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error<'a> {
    AttrParsingFailed(&'a Ident),
    EmptyAttr(&'a Ident),
    EmptyEnum,
    ErrAttrNotFound(&'a Ident),
    ExcessAttr(&'a Ident),
    IdentRetrievalFailed(&'a Ident),
    InvalidAttr(&'a String),
    NotAnEnum,
    UnexpectedAttrArgs(&'a Ident, usize),
    UnsupportedAttrArg(&'a Ident),
    UnusedVariantField(&'a Ident, &'a String),
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AttrParsingFailed(var_ident) => write!(
                f,
                "Failed to parse the `error` attribute of the `{}` variant. This error should not be possible, please report the issue at https://github.com/AmonRayfa/mabe/issues.",
                var_ident
            ),
            Self::EmptyAttr(var_ident) => {
                write!(f, "The `error` attribute cannot be empty, but it is for the `{}` variant.", var_ident)
            }
            Self::EmptyEnum => write!(f, "The `Error` derive macro cannot be used on empty enums."),
            Self::ErrAttrNotFound(var_ident) => {
                write!(f, "The `{}` variant is missing the `error` attribute.", var_ident)
            }
            Self::ExcessAttr(var_ident) => write!(
                f,
                "The `error` attribute can only be used once on the same variant, but the `{}` variant has multiple instances of it.",
                var_ident
            ),
            Self::IdentRetrievalFailed(var_ident) => write!(
                f,
                "Failed to retrieve the identifier of a field of the `{}` variant. This error should not be possible, please report the issue at https://github.com/AmonRayfa/mabe/issues.",
                var_ident
            ),
            Self::InvalidAttr(attr) => write!(
                f,
                "The `Error` derive macro only supports the `error` attribute, but `{}` was found. This error should not be possible, please report the issue at https://github.com/AmonRayfa/mabe/issues.",
                attr
            ),
            Self::NotAnEnum => write!(f, "The `Error` derive macro can only be used on enums."),
            Self::UnexpectedAttrArgs(var_ident, args_count) => write!(
                f,
                "The `error` attribute can only take 1 argument, but `{}` were found for the `{}` variant.",
                args_count, var_ident
            ),
            Self::UnsupportedAttrArg(var_ident) => write!(
                f,
                "The argument of the `error` attribute must be a string literal, but a different type was found for the `{}` variant.",
                var_ident
            ),
            Self::UnusedVariantField(var_ident, field) => {
                write!(f, "The `{}` field of the `{}` variant is not used in the `error` message.", field, var_ident)
            }
        }
    }
}

impl<'a> std::error::Error for Error<'a> {}
