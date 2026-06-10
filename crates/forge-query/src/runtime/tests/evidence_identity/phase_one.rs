use super::super::support::*;
use super::proof_support::*;
use serde_json::json;

#[test]
fn runtime_evidence_identity_surface_is_exported_through_public_facades() {
    let identity = crate::ForgeQueryEvidenceIdentityScheme::V1;
    let runtime_identity = crate::facade::runtime::ForgeQueryEvidenceIdentityScheme::V1;

    assert_eq!(identity, runtime_identity);
    assert_eq!(
        crate::ForgeQueryEvidenceScope::RuntimeStateSnapshot,
        crate::facade::runtime::ForgeQueryEvidenceScope::RuntimeStateSnapshot
    );
    let manual = crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::RuntimeStateSnapshot,
    )
    .field_shape(crate::ForgeQueryEvidenceTag::new("kind"), "ready")
    .seal();
    assert_canonical_evidence_identity_token(manual.as_str());
}

#[test]
fn support_matrix_and_state_snapshot_emit_canonical_evidence_tokens() {
    let runtime = bridge_backed_runtime_with_support(intent_support_profile());
    let workspace =
        ForgeQueryWorkspace::new("evidence-identity-support", runtime).expect("workspace builds");
    let matrix = workspace.public_support_matrix();
    let snapshot = ForgeQueryRuntimeStateSnapshot::ready(
        "basis|digest",
        "result:shape",
        ForgeQueryAuthorityLane::PreviewTruth,
        "state explanation with | and : punctuation",
    );

    assert_canonical_evidence_identity_token(matrix.matrix_digest());
    for row in matrix.rows() {
        assert_canonical_evidence_identity_token(row.row_digest());
        assert_eq!(
            row.row_digest().as_str(),
            compose_public_support_matrix_row_identity(row).as_str()
        );
    }
    assert_eq!(
        matrix.matrix_digest().as_str(),
        compose_public_support_matrix_identity(&matrix).as_str()
    );
    assert_canonical_evidence_identity_token(snapshot.state_digest());
    assert_eq!(
        snapshot.state_digest().as_str(),
        compose_state_snapshot_identity(&snapshot).as_str()
    );
}

#[test]
fn state_snapshot_with_optional_postures_recomposes_exactly() {
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
fn basis_admissions_emit_canonical_evidence_tokens() {
    let authority = crate::runtime::ForgeQueryRuntimeEvidenceAuthority::new();
    let preview = crate::runtime::ForgeQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview basis | punctuation"),
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        ["basis|one", "basis:two"],
    );
    let branch = crate::runtime::ForgeQueryBranchBasisAdmission::new(
        &authority,
        test_session_label("branch basis | punctuation"),
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        ["branch|one", "branch:two"],
    );

    assert_canonical_evidence_identity_token(preview.admission_digest());
    assert_canonical_evidence_identity_token(branch.admission_digest());

    let manual_preview_identity = compose_basis_admission_identity(
        crate::ForgeQueryEvidenceScope::PreviewBasisAdmission,
        preview.session_label(),
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        ForgeQueryAuthorityLane::PreviewTruth,
        ["basis|one", "basis:two"],
    );
    assert_eq!(preview.admission_digest().as_str(), manual_preview_identity.as_str());
}

#[test]
fn preview_and_branch_receipts_compose_from_basis_admissions() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
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
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview session should be admitted");
    let preview_basis_admission_digest = preview.basis_admission().admission_digest().to_string();
    let admitted_receipt = preview
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "preview|receipt:test",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({ "entity": "task|1", "title": "preview: title" }),
        ))
        .expect("sandboxed preview intent should be admitted");

    assert_canonical_evidence_identity_token(admitted_receipt.admission_digest());
    assert_canonical_evidence_identity_token(admitted_receipt.receipt_digest());

    let manual_preview_admission = crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::PreviewIntentAdmission,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("intent_name"),
        admitted_receipt.intent_name(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("strategy_identity"),
        admitted_receipt.strategy_identity(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("strategy_version"),
        admitted_receipt.strategy_version(),
    )
    .field_identity(
        crate::ForgeQueryEvidenceTag::new("canonical_input_digest"),
        admitted_receipt.canonical_input_digest(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("source_lane"),
        admitted_receipt.source_lane().as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("target_lane"),
        admitted_receipt.target_lane().as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("effect_policy"),
        admitted_receipt.effect_policy().as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("admitted_action"),
        ForgeQueryEffectAction::WriteIntent.as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("admitted_lane"),
        ForgeQueryAuthorityLane::PreviewTruth.as_str(),
    )
    .field_identity(
        crate::ForgeQueryEvidenceTag::new("basis_admission_digest"),
        &preview_basis_admission_digest,
    )
    .seal();
    assert_eq!(
        admitted_receipt.admission_digest().as_str(),
        manual_preview_admission.as_str()
    );
    let manual_preview_receipt = compose_receipt_identity(
        crate::ForgeQueryEvidenceScope::PreviewIntentReceipt,
        admitted_receipt.admission_digest(),
        "preview-local-staged-no-authoritative-execution",
    );
    assert_eq!(
        admitted_receipt.receipt_digest().as_str(),
        manual_preview_receipt.as_str()
    );

    let mut branch = runtime
        .branch_with_options(
            test_session_label("branch identity composition"),
            ForgeQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("branch session should be admitted");
    let branch_basis_admission_digest = branch.basis_admission().admission_digest().to_string();
    let branch_receipt = branch
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "branch|receipt:test",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({ "entity": "task|2", "title": "branch: title" }),
        ))
        .expect("branch intent should be admitted");

    assert_canonical_evidence_identity_token(branch_receipt.admission_digest());
    assert_canonical_evidence_identity_token(branch_receipt.receipt_digest());
    let manual_branch_admission = crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::BranchIntentAdmission,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("intent_name"),
        branch_receipt.intent_name(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("strategy_identity"),
        branch_receipt.strategy_identity(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("strategy_version"),
        branch_receipt.strategy_version(),
    )
    .field_identity(
        crate::ForgeQueryEvidenceTag::new("canonical_input_digest"),
        branch_receipt.canonical_input_digest(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("source_lane"),
        branch_receipt.source_lane().as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("target_lane"),
        branch_receipt.target_lane().as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("effect_policy"),
        branch_receipt.effect_policy().as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("admitted_action"),
        ForgeQueryEffectAction::WriteIntent.as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("admitted_lane"),
        ForgeQueryAuthorityLane::BranchLocalTruth.as_str(),
    )
    .field_identity(
        crate::ForgeQueryEvidenceTag::new("basis_admission_digest"),
        &branch_basis_admission_digest,
    )
    .field_identity(
        crate::ForgeQueryEvidenceTag::new("basis_snapshot_token"),
        branch_receipt.basis_snapshot_token(),
    )
    .seal();
    assert_eq!(
        branch_receipt.admission_digest().as_str(),
        manual_branch_admission.as_str()
    );
    let manual_branch_receipt = compose_receipt_identity(
        crate::ForgeQueryEvidenceScope::BranchIntentReceipt,
        branch_receipt.admission_digest(),
        "branch-local-staged-no-authoritative-execution",
    );
    assert_eq!(
        branch_receipt.receipt_digest().as_str(),
        manual_branch_receipt.as_str()
    );

    let denied = {
        let mut preview = runtime
            .preview(test_session_label("derive-only denial punctuation"))
            .expect("preview session should be admitted");
        preview
            .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
                "preview|denial:test",
                "strategy.intent.reconcile",
                "1.0",
                "intent.reconcile.input.v1",
                json!({ "entity": "task|1" }),
            ))
            .expect_err("derive-only preview must deny write intents")
    };

    match denied {
        ForgeQueryRuntimeError::IntentCommitDenied { evidence, .. } => {
            assert_canonical_evidence_identity_token(evidence.denial_digest());
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
    let authority = crate::runtime::ForgeQueryRuntimeEvidenceAuthority::new();
    let left = crate::runtime::ForgeQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview|basis"),
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        ["alpha", "beta|gamma"],
    );
    let right = crate::runtime::ForgeQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview"),
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        ["basis|alpha", "beta|gamma"],
    );
    let branch = crate::runtime::ForgeQueryBranchBasisAdmission::new(
        &authority,
        test_session_label("preview|basis"),
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        ["alpha", "beta|gamma"],
    );

    assert_ne!(left.admission_digest(), right.admission_digest());
    assert_ne!(left.admission_digest(), branch.admission_digest());
}
