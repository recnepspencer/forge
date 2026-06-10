use super::super::support::*;
use super::proof_support::*;

#[test]
fn support_matrix_and_state_snapshot_remain_in_phase_two_canonical_migration_coverage() {
    let runtime = bridge_backed_runtime_with_support(intent_support_profile());
    let workspace =
        ForgeQueryWorkspace::new("phase-two-evidence-identity", runtime).expect("workspace builds");
    let matrix = workspace.public_support_matrix();
    let snapshot = ForgeQueryRuntimeStateSnapshot::ready(
        "basis|digest",
        "result:shape",
        ForgeQueryAuthorityLane::PreviewTruth,
        "state explanation with | and : punctuation",
    );

    assert_canonical_evidence_identity_token(matrix.matrix_digest());
    for row in matrix.rows() {
        assert_eq!(
            row.row_digest().as_str(),
            compose_public_support_matrix_row_identity(row).as_str()
        );
    }
    assert_eq!(
        matrix.matrix_digest().as_str(),
        compose_public_support_matrix_identity(&matrix).as_str()
    );
    assert_eq!(
        snapshot.state_digest().as_str(),
        compose_state_snapshot_identity(&snapshot).as_str()
    );
}

#[test]
fn state_snapshot_with_optional_postures_remains_in_phase_two_canonical_migration_coverage() {
    let ordinary_runtime_posture = crate::ordinary_outcome::ForgeQueryOrdinaryRuntimePosture::new(
        crate::ordinary_outcome::ForgeQueryOrdinaryRuntimePostureKind::Revalidating,
        crate::ordinary_outcome::ForgeQueryOrdinaryRuntimeCausePostureKind::MixedCause,
        Some(crate::ordinary_outcome::ForgeQueryOrdinaryRuntimeAsyncPostureKind::Revalidating),
        crate::ordinary_outcome::ForgeQueryOrdinaryRuntimeBasisPostureKind::GenerationDrift,
        Some(
            crate::ordinary_outcome::ForgeQueryOrdinaryRuntimeRemaskPostureKind::SchemaContextDrift,
        ),
        "support|evidence|digest",
    );
    let async_result_state = ForgeQueryRuntimeAsyncResultState::new(
        ForgeQueryRuntimeAsyncResultStateKind::Revalidating,
        "causality|digest",
        "basis|digest",
        "generation|digest",
    );
    let remask_projection = ForgeQueryRuntimeRemaskProjection::remasked(
        ForgeQueryRuntimeRemaskReasonKind::SchemaContextDrift,
        "policy|digest",
        "tenant-truth|digest",
        "tenant-schema|digest",
        "relationship-proof|digest",
        "schema-context|digest",
    );
    let remask_posture = ForgeQueryRuntimeRemaskPosture::from_activation_projection(
        &remask_projection,
        "support|evidence|digest",
        "basis|digest",
    );
    let snapshot = ForgeQueryRuntimeStateSnapshot::ready(
        "basis|digest",
        "result:shape",
        ForgeQueryAuthorityLane::BridgeExternalState,
        "state explanation with optional posture pressure",
    )
    .with_ordinary_runtime_posture(ordinary_runtime_posture)
    .with_async_result_state(async_result_state)
    .with_remask_posture(remask_posture);

    assert_canonical_evidence_identity_token(snapshot.state_digest());
    assert_eq!(
        snapshot.state_digest().as_str(),
        compose_state_snapshot_identity(&snapshot).as_str()
    );
}

#[test]
fn public_api_contract_transcript_and_support_report_emit_canonical_evidence_tokens() {
    let runtime = stateful_bridge_task_runtime();
    let contract = runtime.public_api_contract();
    assert_canonical_evidence_identity_token(contract.contract_digest());
    assert_eq!(
        contract.contract_digest(),
        compose_public_api_contract_identity(&contract).as_str()
    );
    for family in contract.families() {
        assert_canonical_evidence_identity_token(family.contract_digest());
        assert_eq!(
            family.contract_digest(),
            compose_public_api_family_contract_identity(family).as_str()
        );
    }

    let transcript =
        crate::harness::RuntimeApiStabilizationAdapter::composed_runtime_hostile_transcript_evidence();
    assert_canonical_evidence_identity_token(transcript.transcript_digest());
    assert_eq!(
        transcript.transcript_digest(),
        compose_runtime_public_api_transcript_identity(&transcript).as_str()
    );
    assert!(
        transcript.support_gated_neighbor_denial_digests().len() >= 2,
        "ordinary runtime-backed transcript should carry multiple support-gated neighbor denials"
    );
    assert!(
        transcript.delivery_residue_count() >= 1,
        "ordinary runtime-backed transcript should prove delivery residue through the public transcript surface"
    );
    assert!(
        transcript.meaningful_assertion_count() >= 16,
        "ordinary runtime-backed transcript should remain a certification-grade proof artifact"
    );

    let report =
        crate::application::ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    assert_canonical_evidence_identity_token(report.report_digest());
    assert_eq!(
        report.report_digest(),
        compose_support_report_identity(&report).as_str()
    );

    let query_disabled_report = crate::application::ForgeQueryApplicationFacade::new(
        crate::application::ForgeQueryConfig::runtime_backed_default()
            .with_query(crate::application::ForgeQueryQueryConfig::disabled())
            .with_signal(crate::application::ForgeQuerySignalConfig::disabled())
            .with_runtime_bridge(crate::application::ForgeQueryRuntimeBridgeConfig::disabled())
            .with_relational(crate::application::ForgeQueryRelationalConfig::disabled()),
    )
    .expect("query-disabled facade config should remain valid")
    .support_report();
    assert_canonical_evidence_identity_token(query_disabled_report.report_digest());
    assert_eq!(
        query_disabled_report.report_digest(),
        compose_support_report_identity(&query_disabled_report).as_str()
    );
    assert_ne!(
        report.report_digest(),
        query_disabled_report.report_digest()
    );
}

#[test]
fn phase_two_covered_surfaces_have_no_digest_folklore_residue() {
    assert_phase_two_surface_has_no_digest_folklore(include_str!("../../public_api.rs"));
    assert_phase_two_surface_has_no_digest_folklore(include_str!("../../public_api_transcript.rs"));
    assert_phase_two_surface_has_no_digest_folklore(include_str!("../../support/profile.rs"));
    assert_phase_two_surface_has_no_digest_folklore(include_str!(
        "../../../application/support/report.rs"
    ));
    assert_phase_two_surface_has_no_digest_folklore(include_str!("../../support_matrix.rs"));
    assert_phase_two_surface_has_no_digest_folklore(include_str!("../../state_snapshot.rs"));
}
