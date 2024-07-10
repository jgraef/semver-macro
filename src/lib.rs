use proc_macro_error::{abort, proc_macro_error};
use quote::quote_spanned;
use semver::Version;
use syn::{parse_macro_input, LitStr};

#[proc_macro_error]
#[proc_macro]
pub fn version(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let span = input.span();

    let version: Version = input
        .value()
        .parse()
        .unwrap_or_else(|e| abort!(span, "{}", e));

    if !version.pre.is_empty() {
        abort!(span, "Version can't have a pre-release identifier.");
    }
    if !version.build.is_empty() {
        abort!(span, "Version can't have a build metadata identifier.");
    }

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
