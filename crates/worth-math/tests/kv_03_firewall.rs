//! KV-03: CertifiedTriSign compile-time firewall (Doctrine D3).
//!
//! Uses `trybuild` to verify that external crates cannot construct
//! `CertifiedTriSign` directly — they must go through certified predicates.

#[test]
fn kv03_certified_sign_firewall() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/construct_certified_sign.rs");
}
