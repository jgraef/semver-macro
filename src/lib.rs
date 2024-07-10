//! A proc-macro for [`semver::Version`]
//!
//! This crate contains macros to parse [`semver::Version`]s at compile-time.
//! Currently only versions without pre-release and build metadata identifier
//! are supported.
//!
//! # Example
//!
//! ```
//! use semver::Version;
//! use semver_macro::{version, env_version};
//!
//! // parse from string literal
//! const FROM_LITERAL: Version = version!("0.1.0");
//!
//! // parse from environment variable
//! const FROM_ENV: Version = env_version!("CARGO_PKG_VERSION");
//! ```

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote_spanned;
use semver::Version;
use syn::{parse_macro_input, LitStr};

/// Macro that parses a [`semver::Version`] from string and outputs it as a
/// constant expression.
///
/// # Example
///
/// ```
/// use semver::Version;
/// use semver_macro::version;
///
/// const MY_VERSION: Version = version!("0.1.0");
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn version(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let span = input.span();

    parse_and_emit(&input.value(), span)
}

/// Macro that parses a [`semver::Version`] from an environment variable and
/// outputs it as a constant expression.
///
/// # Example
///
/// ```
/// use semver::Version;
/// use semver_macro::env_version;
///
/// const MY_VERSION: Version = env_version!("CARGO_PKG_VERSION");
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn env_version(input: proc_macro::TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let span = input.span();

    let input = std::env::var(input.value()).unwrap_or_else(|e| abort!(span, "{}", e));

    parse_and_emit(&input, span)
}

fn parse_and_emit(input: &str, span: Span) -> TokenStream {
    let version: Version = input.parse().unwrap_or_else(|e| abort!(span, "{}", e));

    if !version.pre.is_empty() {
        abort!(span, "Version can't have a pre-release identifier.");
    }
    if !version.build.is_empty() {
        abort!(span, "Version can't have a build metadata identifier.");
    }

    emit(&version, span)
}

fn emit(version: &Version, span: Span) -> TokenStream {
    let major = version.major;
    let minor = version.minor;
    let patch = version.patch;

    quote_spanned! {
        span =>
        ::semver::Version {
            major: #major,
            minor: #minor,
            patch: #patch,
            pre: ::semver::Prerelease::EMPTY,
            build: ::semver::BuildMetadata::EMPTY,
        }
    }
    .into()
}
