use forge_query::facade::runtime::{
    forge_query_graph_read_access_compile_fail_boundary_digest,
    forge_query_graph_read_access_compile_fail_target_count,
    forge_query_graph_read_access_compile_fail_targets,
};

const EXPECTED_TARGET_COUNT: usize = 42;
const EXPECTED_BOUNDARY_DIGEST: &str =
    "ae7974c1948ec542940dcd999cd291b1164d99bf4a6be344c6250807952ace1e";

#[test]
fn graph_read_access_public_boundaries_reject_forged_artifacts() {
    let targets = forge_query_graph_read_access_compile_fail_targets();
    assert_eq!(targets.len(), EXPECTED_TARGET_COUNT);
    assert_eq!(
        forge_query_graph_read_access_compile_fail_target_count(),
        EXPECTED_TARGET_COUNT
    );
    assert_eq!(
        forge_query_graph_read_access_compile_fail_boundary_digest(),
        EXPECTED_BOUNDARY_DIGEST
    );

    let t = trybuild::TestCases::new();
    for target in targets {
        t.compile_fail(target);
    }
}
