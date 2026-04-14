/// This file must FAIL to compile.
///
/// KV-03: Attempting to construct a `CertifiedTriSign` outside of `worth-math`
/// must be a compile error. The `new()` constructor is `pub(crate)`, so external
/// crates cannot call it directly.
///
/// This enforces Doctrine D3: all topology decisions must flow through
/// certified predicates, not raw float comparisons.
use worth_math::sign::{CertifiedTriSign, TriSign};

fn main() {
    // This MUST fail: CertifiedTriSign::new is pub(crate)
    let _fake = CertifiedTriSign::new(TriSign::Pos);
}
