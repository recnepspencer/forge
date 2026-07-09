use worth_store_readiness::S6MaterializedCertificationCloseoutEvidence;

struct LocalScalarCounts;

fn main() {
    fn requires_materialized_evidence(_: impl S6MaterializedCertificationCloseoutEvidence) {}

    requires_materialized_evidence(LocalScalarCounts);
}
