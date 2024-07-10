use semver::Version;
use semver_macro::{env_version, version};

#[test]
fn it_parses_from_string_literals() {
    const V: Version = version!("1.2.3");
    assert_eq!(V, "1.2.3".parse().unwrap());
}

#[test]
fn it_parses_from_env_var() {
    const V: Version = env_version!("CARGO_PKG_VERSION");
    assert_eq!(V, std::env!("CARGO_PKG_VERSION").parse().unwrap());
}
