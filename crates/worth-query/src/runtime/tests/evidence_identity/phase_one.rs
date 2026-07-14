use super::super::support::*;
use super::proof_support::*;
use crate::runtime::evidence_identities::{
    runtime_state_snapshot_basis_label_identity, runtime_state_snapshot_result_shape_label_identity,
};

#[test]
fn runtime_evidence_identity_surface_is_exported_through_public_facades() {
    let identity = crate::WorthQueryEvidenceIdentityScheme::V1;
    let runtime_identity = crate::facade::runtime::WorthQueryEvidenceIdentityScheme::V1;

    assert_eq!(identity, runtime_identity);
    assert_eq!(
        crate::WorthQueryEvidenceScope::RuntimeStateSnapshot,
        crate::facade::runtime::WorthQueryEvidenceScope::RuntimeStateSnapshot
    );
    let manual = crate::WorthQueryEvidenceIdentity::compose(
        crate::WorthQueryEvidenceScope::RuntimeStateSnapshot,
    )
    .field_shape(crate::WorthQueryEvidenceTag::new("kind"), "ready")
    .seal();
    assert_canonical_evidence_identity_token(manual.as_str());
}

#[test]
fn support_matrix_and_state_snapshot_emit_canonical_evidence_tokens() {
    let runtime = bridge_backed_runtime_with_support(intent_support_profile());
    let workspace =
        WorthQueryWorkspace::new("evidence-identity-support", runtime).expect("workspace builds");
    let matrix = workspace.public_support_matrix();
    let snapshot = WorthQueryRuntimeStateSnapshot::ready(
        runtime_state_snapshot_basis_label_identity(
            &crate::runtime::evidence_identities::runtime_state_snapshot_test_subject_identity(
                "basis|digest",
            ),
        ),
        runtime_state_snapshot_result_shape_label_identity(
            &crate::runtime::evidence_identities::runtime_state_snapshot_test_subject_identity(
                "result:shape",
            ),
        ),
        WorthQueryAuthorityLane::PreviewTruth,
        "state explanation with | and : punctuation",
    );

    assert_canonical_evidence_identity_token(
        matrix.matrix_digest().terminal_projection_for_reporting(),
    );
    for row in matrix.rows() {
        assert_canonical_evidence_identity_token(
            row.row_digest().terminal_projection_for_reporting(),
        );
        assert_eq!(
            row.row_digest().as_str(),
            compose_public_support_matrix_row_identity(row).as_str()
        );
    }
    assert_eq!(
        matrix.matrix_digest().as_str(),
        compose_public_support_matrix_identity(&matrix).as_str()
    );
    assert_canonical_evidence_identity_token(
        snapshot.state_digest().terminal_projection_for_reporting(),
    );
    assert_eq!(
        snapshot.state_digest().as_str(),
        compose_state_snapshot_identity(&snapshot).as_str()
    );
}

#[test]
fn state_snapshot_with_optional_postures_recomposes_exactly() {
    let ordinary_runtime_posture = crate::ordinary_outcome::WorthQueryOrdinaryRuntimePosture::new(
        crate::ordinary_outcome::WorthQueryOrdinaryRuntimePostureKind::Revalidating,
        crate::ordinary_outcome::WorthQueryOrdinaryRuntimeCausePostureKind::MixedCause,
        Some(crate::ordinary_outcome::WorthQueryOrdinaryRuntimeAsyncPostureKind::Revalidating),
        crate::ordinary_outcome::WorthQueryOrdinaryRuntimeBasisPostureKind::GenerationDrift,
        Some(
            crate::ordinary_outcome::WorthQueryOrdinaryRuntimeRemaskPostureKind::SchemaContextDrift,
        ),
        "support|evidence|digest",
    );
    let async_result_state = WorthQueryRuntimeAsyncResultState::new(
        WorthQueryRuntimeAsyncResultStateKind::Revalidating,
        &crate::runtime::async_result_state::runtime_async_causality_from_label("causality|digest"),
        &crate::runtime::async_result_state::runtime_async_checkpoint_label_identity(
            "basis|digest",
        ),
        &crate::runtime::async_result_state::runtime_async_checkpoint_label_identity(
            "generation|digest",
        ),
    );
    let remask_projection = WorthQueryRuntimeRemaskProjection::remasked(
        WorthQueryRuntimeRemaskReasonKind::SchemaContextDrift,
        "policy|digest",
        "tenant-truth|digest",
        "tenant-schema|digest",
        "relationship-proof|digest",
        "schema-context|digest",
    );
    let remask_posture = WorthQueryRuntimeRemaskPosture::from_activation_projection(
        &remask_projection,
        &runtime_state_snapshot_basis_label_identity(
            &crate::runtime::evidence_identities::runtime_state_snapshot_test_subject_identity(
                "support|evidence|digest",
            ),
        ),
        &runtime_state_snapshot_basis_label_identity(
            &crate::runtime::evidence_identities::runtime_state_snapshot_test_subject_identity(
                "basis|digest",
            ),
        ),
    );
    let snapshot = WorthQueryRuntimeStateSnapshot::ready(
        runtime_state_snapshot_basis_label_identity(
            &crate::runtime::evidence_identities::runtime_state_snapshot_test_subject_identity(
                "basis|digest",
            ),
        ),
        runtime_state_snapshot_result_shape_label_identity(
            &crate::runtime::evidence_identities::runtime_state_snapshot_test_subject_identity(
                "result:shape",
            ),
        ),
        WorthQueryAuthorityLane::BridgeExternalState,
        "state explanation with optional posture pressure",
    )
    .with_ordinary_runtime_posture(ordinary_runtime_posture)
    .with_async_result_state(async_result_state)
    .with_remask_posture(remask_posture);

    assert_canonical_evidence_identity_token(
        snapshot.state_digest().terminal_projection_for_reporting(),
    );
    assert_eq!(
        snapshot.state_digest().as_str(),
        compose_state_snapshot_identity(&snapshot).as_str()
    );
}

#[test]
fn basis_admissions_emit_canonical_evidence_tokens() {
    let authority = crate::runtime::WorthQueryRuntimeEvidenceAuthority::new();
    let preview = crate::runtime::WorthQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview basis | punctuation"),
        WorthQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::WorthQueryBasisAdmissionEvidenceRow::rows_from_values([
            "basis|one",
            "basis:two",
        ]),
    );
    let branch = crate::runtime::WorthQueryBranchBasisAdmission::new(
        &authority,
        test_session_label("branch basis | punctuation"),
        WorthQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::WorthQueryBasisAdmissionEvidenceRow::rows_from_values([
            "branch|one",
            "branch:two",
        ]),
    );

    assert_canonical_evidence_identity_token(
        preview
            .admission_identity()
            .terminal_projection_for_reporting(),
    );
    assert_canonical_evidence_identity_token(
        branch
            .admission_identity()
            .terminal_projection_for_reporting(),
    );

    let manual_preview_identity = compose_basis_admission_identity(
        crate::WorthQueryEvidenceScope::PreviewBasisAdmission,
        preview.session_label(),
        WorthQueryEffectPolicy::SandboxedWriteIntent,
        WorthQueryAuthorityLane::PreviewTruth,
        ["basis|one", "basis:two"],
    );
    assert_eq!(
        preview.admission_digest().as_str(),
        manual_preview_identity.as_str()
    );
}

#[test]
fn preview_and_branch_receipts_compose_from_basis_admissions() {
    let mut runtime = WorthQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(TestSnapshotIdentityAdapter)
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(TestIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");

    let mut preview = runtime
        .preview_with_options(
            test_session_label("preview identity punctuation | : test"),
            WorthQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview session should be admitted");
    let preview_basis_admission_identity = preview.basis_admission().admission_identity().clone();
    let admitted_receipt = preview
        .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
            "preview|receipt:test",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task|1"), ("title", "preview: title")]),
        ))
        .expect("sandboxed preview intent should be admitted");

    assert_canonical_evidence_identity_token(
        admitted_receipt
            .admission_identity()
            .terminal_projection_for_reporting(),
    );
    assert_canonical_evidence_identity_token(admitted_receipt.receipt_digest());

    let manual_preview_admission = crate::WorthQueryEvidenceIdentity::compose(
        crate::WorthQueryEvidenceScope::PreviewIntentAdmission,
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("intent_name"),
        admitted_receipt.intent_name(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("strategy_identity"),
        admitted_receipt.strategy_identity(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("strategy_version"),
        admitted_receipt.strategy_version(),
    )
    .field_value(
        crate::WorthQueryEvidenceTag::new("canonical_input_digest"),
        admitted_receipt.canonical_input_digest(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("source_lane"),
        admitted_receipt.source_lane().as_str(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("target_lane"),
        admitted_receipt.target_lane().as_str(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("effect_policy"),
        admitted_receipt.effect_policy().as_str(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("admitted_action"),
        WorthQueryEffectAction::WriteIntent.as_str(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("admitted_lane"),
        WorthQueryAuthorityLane::PreviewTruth.as_str(),
    )
    .field_evidence_identity(
        crate::WorthQueryEvidenceTag::new("basis_admission_identity"),
        &preview_basis_admission_identity,
    )
    .seal();
    assert_eq!(
        admitted_receipt.admission_identity().as_str(),
        manual_preview_admission.as_str()
    );
    let manual_preview_receipt = crate::WorthQueryEvidenceIdentity::compose(
        crate::WorthQueryEvidenceScope::PreviewIntentReceipt,
    )
    .field_evidence_identity(
        crate::WorthQueryEvidenceTag::new("admission_identity"),
        admitted_receipt.admission_identity(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("posture"),
        "preview-local-staged-no-authoritative-execution",
    )
    .seal();
    assert_eq!(
        admitted_receipt.receipt_digest(),
        manual_preview_receipt.as_str()
    );

    let mut branch = runtime
        .branch_with_options(
            test_session_label("branch identity composition"),
            WorthQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("branch session should be admitted");
    let branch_basis_admission_identity = branch.basis_admission().admission_identity().clone();
    let branch_receipt = branch
        .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
            "branch|receipt:test",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task|2"), ("title", "branch: title")]),
        ))
        .expect("branch intent should be admitted");

    assert_canonical_evidence_identity_token(
        branch_receipt
            .admission_identity()
            .terminal_projection_for_reporting(),
    );
    assert_canonical_evidence_identity_token(branch_receipt.receipt_digest());
    let manual_branch_admission = crate::WorthQueryEvidenceIdentity::compose(
        crate::WorthQueryEvidenceScope::BranchIntentAdmission,
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("intent_name"),
        branch_receipt.intent_name(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("strategy_identity"),
        branch_receipt.strategy_identity(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("strategy_version"),
        branch_receipt.strategy_version(),
    )
    .field_value(
        crate::WorthQueryEvidenceTag::new("canonical_input_digest"),
        branch_receipt.canonical_input_digest(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("source_lane"),
        branch_receipt.source_lane().as_str(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("target_lane"),
        branch_receipt.target_lane().as_str(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("effect_policy"),
        branch_receipt.effect_policy().as_str(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("admitted_action"),
        WorthQueryEffectAction::WriteIntent.as_str(),
    )
    .field_shape(
        crate::WorthQueryEvidenceTag::new("admitted_lane"),
        WorthQueryAuthorityLane::BranchLocalTruth.as_str(),
    )
    .field_evidence_identity(
        crate::WorthQueryEvidenceTag::new("basis_admission_identity"),
        &branch_basis_admission_identity,
    )
    .field_evidence_identity(
        crate::WorthQueryEvidenceTag::new("basis_snapshot_identity"),
        &branch_receipt.basis_snapshot_identity().evidence_identity(),
    )
    .seal();
    assert_eq!(
        branch_receipt.admission_identity().as_str(),
        manual_branch_admission.as_str()
    );
    let manual_branch_receipt = compose_receipt_identity(
        crate::WorthQueryEvidenceScope::BranchIntentReceipt,
        branch_receipt.admission_identity(),
        "branch-local-staged-no-authoritative-execution",
    );
    assert_eq!(
        branch_receipt.receipt_digest(),
        manual_branch_receipt.as_str()
    );

    let denied = {
        let mut preview = runtime
            .preview(test_session_label("derive-only denial punctuation"))
            .expect("preview session should be admitted");
        preview
            .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
                "preview|denial:test",
                "strategy.intent.reconcile",
                "1.0",
                "intent.reconcile.input.v1",
                test_intent_input([("entity", "task|1")]),
            ))
            .expect_err("derive-only preview must deny write intents")
    };

    match denied {
        WorthQueryRuntimeError::IntentCommitDenied { evidence, .. } => {
            assert_canonical_evidence_identity_token(
                evidence.denial_digest().terminal_projection_for_reporting(),
            );
            assert_eq!(
                evidence.denial_digest().as_str(),
                compose_denial_evidence_identity(&evidence).as_str()
            );
        }
        other => panic!("expected intent denial, got {other:?}"),
    }
}

#[test]
fn runtime_surface_evidence_identities_resist_joined_string_folklore_collisions() {
    let authority = crate::runtime::WorthQueryRuntimeEvidenceAuthority::new();
    let left = crate::runtime::WorthQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview|basis"),
        WorthQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::WorthQueryBasisAdmissionEvidenceRow::rows_from_values([
            "alpha",
            "beta|gamma",
        ]),
    );
    let right = crate::runtime::WorthQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview"),
        WorthQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::WorthQueryBasisAdmissionEvidenceRow::rows_from_values([
            "basis|alpha",
            "beta|gamma",
        ]),
    );
    let branch = crate::runtime::WorthQueryBranchBasisAdmission::new(
        &authority,
        test_session_label("preview|basis"),
        WorthQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::WorthQueryBasisAdmissionEvidenceRow::rows_from_values([
            "alpha",
            "beta|gamma",
        ]),
    );

    assert_ne!(left.admission_identity(), right.admission_identity());
    assert_ne!(left.admission_identity(), branch.admission_identity());
}

#[test]
fn phase_one_covered_surfaces_have_no_digest_folklore_residue() {
    use crate::application::{format_digest_folklore_pattern_in, source_for_format_digest_path};

    for path in [
        "runtime/support_matrix.rs",
        "runtime/state_snapshot.rs",
        "runtime/intent/preview.rs",
        "runtime/intent/denial.rs",
    ] {
        let source = source_for_format_digest_path(path).expect("embedded source");
        assert!(
            format_digest_folklore_pattern_in(source).is_none(),
            "phase-1-covered surface must not retain digest folklore: {path}"
        );
    }
}
