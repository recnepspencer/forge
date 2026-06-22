mod contract_bundle_support;
mod local_rebuild_fixture;
mod readiness_workload_fixture;

pub(crate) use readiness_workload_fixture::certified_boolean_readiness_workload_receipt;

pub(crate) fn assert_planar_boolean_query_digest(digest: &str) {
    assert!(!digest.trim().is_empty());
    assert!(!digest.contains("phase-1"));
    assert!(!digest.contains("planar boolean"));
}
