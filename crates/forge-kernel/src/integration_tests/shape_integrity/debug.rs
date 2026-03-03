#[test]
fn debug_dodecahedron() {
    let _ = crate::integration_tests::harness::shapes::dodecahedron([0.0; 3], 1.0).unwrap();
}
