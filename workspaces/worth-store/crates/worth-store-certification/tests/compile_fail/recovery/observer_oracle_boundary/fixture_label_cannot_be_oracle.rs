use worth_store_physical_certification::ReusablePhysicalOracleFamily;

struct FixtureLabel(&'static str);

fn main() {
    let _ = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape().oracle(FixtureLabel("happy-path"));
}
