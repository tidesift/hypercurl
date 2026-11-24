#[test]
fn the_version_is_exposed() {
  assert_eq!(hypercurl::VERSION, env!("CARGO_PKG_VERSION"));
}
