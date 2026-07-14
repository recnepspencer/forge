use worth_query::facade::certification::{
    worth_query_graph_read_access_compile_fail_boundary_digest,
    worth_query_graph_read_access_compile_fail_target_count,
    worth_query_graph_read_access_compile_fail_targets,
    worth_query_graph_read_proof_transition_manifest,
    worth_query_graph_read_proof_transition_manifest_count,
    worth_query_graph_read_proof_transition_manifest_digest,
};
use worth_query::facade::runtime::WorthQueryGraphReadProofBoundaryEvidenceKind;

const EXPECTED_TARGET_COUNT: usize = 45;
const EXPECTED_TRANSITION_MANIFEST_COUNT: usize = 12;
const EXPECTED_BOUNDARY_DIGEST: &str =
    "d4fb2a33136966d4d264d1cc554342cee79fcc3434c79a3e9a24aff89bd6ae4a";
const EXPECTED_TRANSITION_MANIFEST_DIGEST: &str =
    "96121afc8a536e023a50c97a1d813d3b94f0c6e88fbc1ce4824576128dc6646b";

#[test]
fn graph_read_access_public_boundaries_reject_worthd_artifacts() {
    let targets = worth_query_graph_read_access_compile_fail_targets();
    assert_eq!(targets.len(), EXPECTED_TARGET_COUNT);
    assert_eq!(
        worth_query_graph_read_access_compile_fail_target_count(),
        EXPECTED_TARGET_COUNT
    );
    assert_eq!(
        worth_query_graph_read_access_compile_fail_boundary_digest(),
        EXPECTED_BOUNDARY_DIGEST
    );

    let transition_manifest = worth_query_graph_read_proof_transition_manifest();
    assert_eq!(
        worth_query_graph_read_proof_transition_manifest_count(),
        EXPECTED_TRANSITION_MANIFEST_COUNT
    );
    assert_eq!(
        transition_manifest.len(),
        EXPECTED_TRANSITION_MANIFEST_COUNT
    );
    assert_eq!(
        worth_query_graph_read_proof_transition_manifest_digest(),
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
            WorthQueryGraphReadProofBoundaryEvidenceKind::ConstructorPrivate
                | WorthQueryGraphReadProofBoundaryEvidenceKind::PhaseInputRequired
                | WorthQueryGraphReadProofBoundaryEvidenceKind::RawValueRejected
        ));
    }

    let t = trybuild::TestCases::new();
    for target in targets {
        t.compile_fail(target);
    }
}
