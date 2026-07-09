use worth_store_physical_certification::ReusablePhysicalOracleFamily;

struct FixtureLabel(&'static str);

fn main() {
    let _ = ReusablePhysicalOracleFamily::s5_readiness_shape().oracle(FixtureLabel("happy-path"));
}
