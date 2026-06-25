use super::super::{
    WorthGraphReadAccessScopeExpectation, WorthGraphReadAccessScopeFamily,
    WorthGraphReadAccessScopeKind, WorthGraphReadAccessScopePlanReport,
};

pub(crate) fn assert_exact_scope_plan(scope_plan: &WorthGraphReadAccessScopePlanReport) {
    let expected = expected_scope_plan();
    assert_eq!(scope_plan.entries().len(), expected.len());
    for expected_scope in expected {
        let entry = scope_plan
            .entry_for_source_path(expected_scope.source_path)
            .unwrap_or_else(|| {
                panic!(
                    "missing scope plan entry for {}",
                    expected_scope.source_path
                )
            });
        let binding = entry.scope_binding();
        assert_eq!(entry.source_path(), expected_scope.source_path);
        assert_eq!(binding.scope_kind(), expected_scope.scope_kind);
        assert_eq!(binding.scope_family(), expected_scope.scope_family);
        assert_eq!(
            binding.scope_expectation(),
            expected_scope.scope_expectation
        );
        assert_eq!(
            binding.selected_obligation_index(),
            expected_scope.selected_obligation_index
        );
        assert_eq!(binding.authority_digest(), expected_scope.authority_digest);
        assert_eq!(
            binding.touch_descriptor_digest(),
            expected_scope.touch_descriptor_digest
        );
        assert_eq!(
            binding.execution_proof_digest(),
            expected_scope.execution_proof_digest
        );
        assert_eq!(
            binding.selected_registration_digest(),
            expected_scope.selected_registration_digest
        );
        assert_eq!(
            binding.adoption_manifest_digest(),
            expected_scope.adoption_manifest_digest
        );
        assert_eq!(
            binding.certification_boundary(),
            expected_scope.certification_boundary.as_deref()
        );
    }
}

#[derive(Clone)]
struct ExpectedScopePlanEntry {
    source_path: &'static str,
    scope_kind: WorthGraphReadAccessScopeKind,
    scope_family: WorthGraphReadAccessScopeFamily,
    scope_expectation: WorthGraphReadAccessScopeExpectation,
    selected_obligation_index: Option<usize>,
    authority_digest: Option<&'static str>,
    touch_descriptor_digest: Option<&'static str>,
    execution_proof_digest: Option<&'static str>,
    selected_registration_digest: Option<&'static str>,
    adoption_manifest_digest: Option<&'static str>,
    certification_boundary: Option<String>,
}

fn expected_scope_plan() -> [ExpectedScopePlanEntry; 12] {
    [
        topology_read_proof("crates/worth-topo/src/projection/read_views/domain"),
        touched_authority("crates/worth-topo/src/projection/runtime_boundary/read_execution"),
        certification("crates/worth-topo/src/projection/read_views/domain/read_proof"),
        selected_obligation(
            "crates/worth-spatial/src/workload_platform/evidence_ledger",
            1,
            WorthGraphReadAccessScopeFamily::SpatialEvidenceLookup,
        ),
        spatial_continuation(
            "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction",
        ),
        spatial_continuation("crates/worth-spatial/src/workload_platform/planar_boolean_events"),
        deleted_graph_read_source("crates/worth-kernel/src/query_adoption/graph_read_access"),
        selected_obligation(
            "crates/worth-kernel/src/workload_composition",
            0,
            WorthGraphReadAccessScopeFamily::KernelWorkloadComposition,
        ),
        touch_descriptor("crates/worth-kernel/src/binding"),
        certification("crates/worth-topo/src/certification/projection_closeout/tests/topology_reads"),
        certification(
            "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction/test_support",
        ),
        certification("crates/worth-kernel/src/binding/tests"),
    ]
}

fn topology_read_proof(source_path: &'static str) -> ExpectedScopePlanEntry {
    indexed_scope(
        source_path,
        WorthGraphReadAccessScopeKind::TopologyReadProof,
        WorthGraphReadAccessScopeFamily::TopologyReadLedger,
        WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput,
        0,
        None,
    )
}

fn touched_authority(source_path: &'static str) -> ExpectedScopePlanEntry {
    indexed_scope(
        source_path,
        WorthGraphReadAccessScopeKind::TouchedAuthorityDigest,
        WorthGraphReadAccessScopeFamily::TopologyRuntimeReadExecution,
        WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput,
        0,
        None,
    )
}

fn selected_obligation(
    source_path: &'static str,
    index: usize,
    family: WorthGraphReadAccessScopeFamily,
) -> ExpectedScopePlanEntry {
    indexed_scope(
        source_path,
        WorthGraphReadAccessScopeKind::SelectedObligation,
        family,
        WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput,
        index,
        Some(if index == 0 {
            "registration-a"
        } else {
            "registration-b"
        }),
    )
}

fn touch_descriptor(source_path: &'static str) -> ExpectedScopePlanEntry {
    indexed_scope(
        source_path,
        WorthGraphReadAccessScopeKind::TouchDescriptorDigest,
        WorthGraphReadAccessScopeFamily::KernelBindingNeighborhood,
        WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput,
        1,
        None,
    )
}

fn spatial_continuation(source_path: &'static str) -> ExpectedScopePlanEntry {
    indexed_scope(
        source_path,
        WorthGraphReadAccessScopeKind::SpatialContinuationProof,
        WorthGraphReadAccessScopeFamily::PlanarBooleanContinuation,
        WorthGraphReadAccessScopeExpectation::QueryAccessRequirementCandidateInput,
        1,
        None,
    )
}

fn indexed_scope(
    source_path: &'static str,
    scope_kind: WorthGraphReadAccessScopeKind,
    scope_family: WorthGraphReadAccessScopeFamily,
    scope_expectation: WorthGraphReadAccessScopeExpectation,
    index: usize,
    selected_registration_digest: Option<&'static str>,
) -> ExpectedScopePlanEntry {
    ExpectedScopePlanEntry {
        source_path,
        scope_kind,
        scope_family,
        scope_expectation,
        selected_obligation_index: Some(index),
        authority_digest: Some(if index == 0 {
            "authority-a"
        } else {
            "authority-b"
        }),
        touch_descriptor_digest: Some(if index == 0 { "touch-a" } else { "touch-b" }),
        execution_proof_digest: Some(if index == 0 {
            "execution-a"
        } else {
            "execution-b"
        }),
        selected_registration_digest,
        adoption_manifest_digest: None,
        certification_boundary: None,
    }
}

fn deleted_graph_read_source(source_path: &'static str) -> ExpectedScopePlanEntry {
    ExpectedScopePlanEntry {
        source_path,
        scope_kind: WorthGraphReadAccessScopeKind::DeletedGraphReadSource,
        scope_family: WorthGraphReadAccessScopeFamily::DeletedGraphReadSource,
        scope_expectation: WorthGraphReadAccessScopeExpectation::DeletionOnlyResidue,
        selected_obligation_index: None,
        authority_digest: None,
        touch_descriptor_digest: None,
        execution_proof_digest: None,
        selected_registration_digest: None,
        adoption_manifest_digest: Some("adoption-a"),
        certification_boundary: None,
    }
}

fn certification(source_path: &'static str) -> ExpectedScopePlanEntry {
    ExpectedScopePlanEntry {
        source_path,
        scope_kind: WorthGraphReadAccessScopeKind::CertificationOnlyBoundary,
        scope_family: WorthGraphReadAccessScopeFamily::CertificationBoundary,
        scope_expectation: WorthGraphReadAccessScopeExpectation::CertificationOnlyBoundary,
        selected_obligation_index: None,
        authority_digest: None,
        touch_descriptor_digest: None,
        execution_proof_digest: None,
        selected_registration_digest: None,
        adoption_manifest_digest: None,
        certification_boundary: Some(format!("certification-boundary:{source_path}")),
    }
}
