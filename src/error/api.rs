// Copyright 2024 Amon Rayfa.
// SPDX-License-Identifier: Apache-2.0.

use syn::Ident;

/// The custom error type for the `api` module.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error<'a> {
    AttrParsingFailed(&'a Ident),
    EmptyAttr(&'a String, &'a Ident),
    EmptyEnum,
    ErrAttrNotFound(&'a Ident),
    ExcessAttr(&'a String, &'a Ident),
    IdentRetrievalFailed(&'a Ident),
    InvalidAttr(&'a String, &'a str),
    NotAnEnum,
    UnexpectedAttrArgs(&'a String, usize, &'a Ident),
    UnsupportedAttrArg(&'a String, &'a Ident),
    UnusedVariantField(&'a Ident, &'a String),
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AttrParsingFailed(var_ident) => write!(f, "[error] Failed to parse the attributes of the `{}` variant.\n[debug] This error should not be possible, try reloading the window. If the problem persists, report the issue to the crate's [GitHub repository](https://github.com/AmonRayfa/mabe).", var_ident),
            Self::EmptyAttr(attr, var_ident) => write!(f, "[error] `Mabe` attributes cannot be empty, but the `{}` attribute of the `{}` variant is.", attr, var_ident),
            Self::EmptyEnum => write!(f, "[error] The `Mabe` derive macro cannot be used on empty enums."),
            Self::ErrAttrNotFound(var_ident) => write!(f, "[error] The `{}` variant is missing the `error` attribute.", var_ident),
            Self::ExcessAttr(attr, var_ident)=> write!(f, "[error] `Mabe` attributes can only be used once on the same variant, but the `{}` variant has multiple `{}` attributes.", var_ident, attr),
            Self::IdentRetrievalFailed(var_ident) => write!(f, "[error] Failed to retrieve the identifier of a field of the `{}` variant.\n[debug] This error should not be possible, try reloading the window. If the problem persists, report the issue to the crate's [GitHub repository](https://github.com/AmonRayfa/mabe).", var_ident),
            Self::InvalidAttr(attr, func) => write!(f, "[error] The `api::derive_macro::{}` function only supports the `error`, `cause`, and `debug` attributes, but `{}` was found.\n[debug] This error should not be possible, try reloading the window. If the problem persists, report the issue to the crate's [GitHub repository](https://github.com/AmonRayfa/mabe).", func, attr),
            Self::NotAnEnum => write!(f, "[error] The `Mabe` derive macro can only be used on enums."),
            Self::UnexpectedAttrArgs(attr, args_count, var_ident) => write!(f, "[error] `Mabe` attributes can only take 1 argument, but `{}` were found for the `{}` attribute of the `{}` variant.", args_count, attr, var_ident),
            Self::UnsupportedAttrArg(attr, var_ident) => write!(f, "[error] The argument of a `Mabe` attribute must be a string literal, but a different type was found for the argument of the `{}` attribute of the `{}` variant.", attr, var_ident),
            Self::UnusedVariantField(var_ident, field) => write!(f, "[error] The `{}` field of the `{}` variant is not used in the error, cause, or debug message.", field, var_ident),
        }
    }
}

impl<'a> std::error::Error for Error<'a> {}
