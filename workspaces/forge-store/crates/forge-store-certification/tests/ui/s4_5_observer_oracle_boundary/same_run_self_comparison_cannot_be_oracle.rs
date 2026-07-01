use forge_store_physical_certification::ReusablePhysicalOracleFamily;

struct SameRunSelfComparison<'a> {
    before: &'a str,
    after: &'a str,
}

fn main() {
    let comparison = SameRunSelfComparison {
        before: "digest-a",
        after: "digest-a",
    };
    let _ = ReusablePhysicalOracleFamily::s5_readiness_shape().oracle(comparison);
}
