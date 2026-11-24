use super::*;

#[test]
fn version_is_the_manifest_version() {
  assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
}
