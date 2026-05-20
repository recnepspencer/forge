use super::{
    forge_query_lower_runtime_phase_artifact_manifest_digest,
    forge_query_lower_runtime_phase_manifest,
    forge_query_lower_runtime_typestate_transition_digest, ForgeQueryLowerRuntimePhaseArtifact,
};

#[test]
fn phase_manifest_names_every_closeout_artifact_in_order() {
    let manifest = forge_query_lower_runtime_phase_manifest();
    let artifact_order = manifest
        .rows()
        .iter()
        .map(|row| row.artifact())
        .collect::<Vec<_>>();

    assert_eq!(manifest.rows().len(), 17);
    assert_eq!(
        artifact_order,
        vec![
            ForgeQueryLowerRuntimePhaseArtifact::CrossingInventory,
            ForgeQueryLowerRuntimePhaseArtifact::CapabilityRequest,
            ForgeQueryLowerRuntimePhaseArtifact::CapabilityEligibility,
            ForgeQueryLowerRuntimePhaseArtifact::RoutePlanOrReadmissionHandoff,
            ForgeQueryLowerRuntimePhaseArtifact::BoundaryExecutionReceipt,
            ForgeQueryLowerRuntimePhaseArtifact::BoundaryEnvelope,
            ForgeQueryLowerRuntimePhaseArtifact::SupportMatrix,
            ForgeQueryLowerRuntimePhaseArtifact::CloseoutRegistry,
            ForgeQueryLowerRuntimePhaseArtifact::PublicSurfaceInventory,
            ForgeQueryLowerRuntimePhaseArtifact::BoundaryReconciliationReport,
            ForgeQueryLowerRuntimePhaseArtifact::NonBypassAudit,
            ForgeQueryLowerRuntimePhaseArtifact::ProofShapeAudit,
            ForgeQueryLowerRuntimePhaseArtifact::PerformanceSlopeReport,
            ForgeQueryLowerRuntimePhaseArtifact::AcceptanceSuite,
            ForgeQueryLowerRuntimePhaseArtifact::CertificationBundle,
            ForgeQueryLowerRuntimePhaseArtifact::NamedClosureTest,
            ForgeQueryLowerRuntimePhaseArtifact::StabilizationCloseoutReport,
        ]
    );
    assert_eq!(
        forge_query_lower_runtime_phase_artifact_manifest_digest(),
        manifest.manifest_digest()
    );
}

#[test]
fn phase_manifest_rows_bind_required_inputs_to_next_consumers() {
    let manifest = forge_query_lower_runtime_phase_manifest();

    for row in manifest.rows() {
        assert!(!row.producer().is_empty());
        assert!(!row.required_input().is_empty());
        assert!(!row.next_consumer().is_empty());
        assert!(!row.enforcement_proof().is_empty());
    }

    assert_eq!(
        manifest.rows()[1].next_consumer(),
        "ForgeQueryLowerRuntimeCapabilityEligibility::{admitted,deferred,unsupported,forbidden}"
    );
    assert_eq!(
        manifest.rows()[4].next_consumer(),
        "ForgeQueryLowerRuntimeBoundaryEnvelope::{from_route_plan,from_readmission_receipt}"
    );
    assert_eq!(
        manifest.rows()[6].next_consumer(),
        "forge_query_lower_runtime_closeout_registry / certify_lower_runtime_routing"
    );
    assert_eq!(
        manifest.rows()[7].artifact(),
        ForgeQueryLowerRuntimePhaseArtifact::CloseoutRegistry
    );
    assert_eq!(
        manifest.rows()[8].artifact(),
        ForgeQueryLowerRuntimePhaseArtifact::PublicSurfaceInventory
    );
    assert_eq!(
        manifest.rows()[9].artifact(),
        ForgeQueryLowerRuntimePhaseArtifact::BoundaryReconciliationReport
    );
    assert_eq!(
        manifest.rows()[10].artifact(),
        ForgeQueryLowerRuntimePhaseArtifact::NonBypassAudit
    );
    assert_eq!(
        manifest.rows()[10].next_consumer(),
        "certify_lower_runtime_routing"
    );
    assert_eq!(
        manifest.rows()[11].artifact(),
        ForgeQueryLowerRuntimePhaseArtifact::ProofShapeAudit
    );
    assert_eq!(
        manifest.rows()[12].artifact(),
        ForgeQueryLowerRuntimePhaseArtifact::PerformanceSlopeReport
    );
    assert_eq!(
        manifest.rows()[13].artifact(),
        ForgeQueryLowerRuntimePhaseArtifact::AcceptanceSuite
    );
    assert_eq!(
        manifest.rows()[13].next_consumer(),
        "certify_lower_runtime_routing"
    );
    assert_eq!(
        manifest.rows()[14].next_consumer(),
        "forge_query_lower_runtime_closure_test"
    );
    assert_eq!(
        manifest.rows()[14].enforcement_proof(),
        "phase_manifest_is_public_and_consumable_by_closeout_bundle"
    );
    assert_eq!(
        manifest.rows()[15].artifact(),
        ForgeQueryLowerRuntimePhaseArtifact::NamedClosureTest
    );
    assert_eq!(
        manifest.rows()[15].next_consumer(),
        "forge_query_lower_runtime_closeout_report"
    );
    assert_eq!(
        manifest.rows()[15].enforcement_proof(),
        "closure_test_binds_boundary_and_compile_lanes_to_certified_rows"
    );
    assert_eq!(
        manifest.rows()[16].artifact(),
        ForgeQueryLowerRuntimePhaseArtifact::StabilizationCloseoutReport
    );
    assert_eq!(
        manifest.rows()[16].next_consumer(),
        "runtime-api-public-stabilization gate"
    );
    assert_eq!(
        manifest.rows()[16].enforcement_proof(),
        "closeout_report_keeps_stabilization_inputs_in_sync"
    );
}

#[test]
fn phase_manifest_exposes_typestate_transition_digest() {
    let manifest = forge_query_lower_runtime_phase_manifest();
    let expected = crate::identity::hash_parts(
        &manifest
            .rows()
            .windows(2)
            .map(|pair| {
                format!(
                    "{}->{}|{}|{}",
                    pair[0].artifact().as_str(),
                    pair[1].artifact().as_str(),
                    pair[0].row_digest(),
                    pair[1].row_digest()
                )
            })
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        forge_query_lower_runtime_typestate_transition_digest(),
        manifest.typestate_transition_digest()
    );
    assert_eq!(manifest.typestate_transition_digest(), expected);
    assert!(!manifest.typestate_transition_digest().is_empty());
}
