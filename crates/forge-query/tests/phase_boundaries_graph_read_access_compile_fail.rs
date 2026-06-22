use forge_query::facade::runtime::{
    forge_query_graph_read_access_compile_fail_boundary_digest,
    forge_query_graph_read_access_compile_fail_target_count,
    forge_query_graph_read_access_compile_fail_targets,
    forge_query_graph_read_proof_transition_manifest,
    forge_query_graph_read_proof_transition_manifest_count,
    forge_query_graph_read_proof_transition_manifest_digest,
    ForgeQueryGraphReadProofBoundaryEvidenceKind,
};

const EXPECTED_TARGET_COUNT: usize = 45;
const EXPECTED_TRANSITION_MANIFEST_COUNT: usize = 12;
const EXPECTED_BOUNDARY_DIGEST: &str =
    "0b69dbc593d39a75fc6890a7d2416367c0d20f3645cab77f071b966879bafc90";
const EXPECTED_TRANSITION_MANIFEST_DIGEST: &str =
    "a8dd05cb21891fc4864d28ad52c763989bd5335c73edf7bd311c184d2a19356c";

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

    let transition_manifest = forge_query_graph_read_proof_transition_manifest();
    assert_eq!(
        forge_query_graph_read_proof_transition_manifest_count(),
        EXPECTED_TRANSITION_MANIFEST_COUNT
    );
    assert_eq!(
        transition_manifest.len(),
        EXPECTED_TRANSITION_MANIFEST_COUNT
    );
    assert_eq!(
        forge_query_graph_read_proof_transition_manifest_digest(),
        EXPECTED_TRANSITION_MANIFEST_DIGEST
    );
    for row in &transition_manifest {
        assert!(
            targets.contains(&row.compile_fail_target()),
            "{} must be backed by a compile-fail target",
            row.artifact()
        );
        assert!(!row.phase().is_empty());
        assert!(!row.artifact().is_empty());
        assert!(matches!(
            row.evidence_kind(),
            ForgeQueryGraphReadProofBoundaryEvidenceKind::ConstructorPrivate
                | ForgeQueryGraphReadProofBoundaryEvidenceKind::PhaseInputRequired
                | ForgeQueryGraphReadProofBoundaryEvidenceKind::RawValueRejected
        ));
    }

    let t = trybuild::TestCases::new();
    for target in targets {
        t.compile_fail(target);
    }
}
