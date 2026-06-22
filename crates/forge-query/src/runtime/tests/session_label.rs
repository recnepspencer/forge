use super::support::*;
use crate::facade::runtime::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryEvidenceIdentity,
    ForgeQueryEvidenceScope, ForgeQueryEvidenceTag, ForgeQuerySessionLabel,
};

fn session_entry_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
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
        .expect("session-entry runtime should build")
}

fn session_entry_workspace() -> ForgeQueryWorkspace {
    session_entry_runtime()
        .workspace("session-entry-workspace")
        .expect("session-entry workspace should build")
}

fn basis_identity(
    scope: ForgeQueryEvidenceScope,
    label: &ForgeQuerySessionLabel,
    effect_policy: ForgeQueryEffectPolicy,
    authority_lane: ForgeQueryAuthorityLane,
    evidence_rows: &[crate::runtime::ForgeQueryBasisAdmissionEvidenceRow],
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(scope)
        .field_value(
            ForgeQueryEvidenceTag::new("session_label_identity"),
            label.identity_digest().terminal_projection_for_reporting(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            effect_policy.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("authority_lane"),
            authority_lane.as_str(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("basis_evidence"),
            evidence_rows
                .iter()
                .map(|row| row.row_digest().terminal_projection_for_reporting()),
        )
        .seal()
}

fn assert_session_label_collision(
    error: ForgeQueryRuntimeError,
    authority_lane: ForgeQueryAuthorityLane,
    label: &ForgeQuerySessionLabel,
) {
    match error.stop_class() {
        ForgeQueryStopClass::SessionLabelCollision {
            authority_lane: observed_lane,
            label: collided,
        } => {
            assert_eq!(observed_lane, authority_lane);
            assert_eq!(collided, label);
        }
        other => panic!("expected typed session label collision, got {other:?}"),
    }
}

#[test]
fn preview_and_branch_basis_admissions_record_canonical_session_label_identity() {
    let mut runtime = session_entry_runtime();
    let label = test_session_label("typed-session-entry");

    let preview = runtime
        .preview_with_options(
            label.clone(),
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview should admit typed label");
    let preview_manual = basis_identity(
        ForgeQueryEvidenceScope::PreviewBasisAdmission,
        &label,
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        ForgeQueryAuthorityLane::PreviewTruth,
        preview.basis_admission().evidence_rows(),
    );
    assert_eq!(preview.basis_admission().session_label(), &label);
    assert_eq!(
        preview.basis_admission().label_identity(),
        label.identity_digest()
    );
    assert_eq!(
        preview.basis_admission().admission_identity(),
        &preview_manual
    );
    drop(preview);

    let branch = runtime
        .branch_with_options(
            label.clone(),
            ForgeQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("branch should admit the same typed label in its own family");
    let branch_manual = basis_identity(
        ForgeQueryEvidenceScope::BranchBasisAdmission,
        &label,
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        ForgeQueryAuthorityLane::BranchLocalTruth,
        branch.basis_admission().evidence_rows(),
    );
    assert_eq!(branch.basis_admission().session_label(), &label);
    assert_eq!(
        branch.basis_admission().label_identity(),
        label.identity_digest()
    );
    assert_eq!(
        branch.basis_admission().admission_identity(),
        &branch_manual
    );
}

#[test]
fn workspace_entrypoints_preserve_typed_session_label_identity_and_collision_posture() {
    let mut workspace = session_entry_workspace();
    let label = test_session_label("workspace-session-entry");

    let preview = workspace
        .preview_with_options(
            label.clone(),
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("workspace preview should admit typed label");
    let preview_manual = basis_identity(
        ForgeQueryEvidenceScope::PreviewBasisAdmission,
        &label,
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        ForgeQueryAuthorityLane::PreviewTruth,
        preview.basis_admission().evidence_rows(),
    );
    assert_eq!(preview.basis_admission().session_label(), &label);
    assert_eq!(
        preview.basis_admission().label_identity(),
        label.identity_digest()
    );
    assert_eq!(
        preview.basis_admission().admission_identity(),
        &preview_manual
    );
    drop(preview);

    let branch = workspace
        .branch_with_options(
            label.clone(),
            ForgeQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("workspace branch should admit same typed label in its own family");
    let branch_manual = basis_identity(
        ForgeQueryEvidenceScope::BranchBasisAdmission,
        &label,
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        ForgeQueryAuthorityLane::BranchLocalTruth,
        branch.basis_admission().evidence_rows(),
    );
    assert_eq!(branch.basis_admission().session_label(), &label);
    assert_eq!(
        branch.basis_admission().label_identity(),
        label.identity_digest()
    );
    assert_eq!(
        branch.basis_admission().admission_identity(),
        &branch_manual
    );
    drop(branch);

    let preview_collision = match workspace.preview(label.clone()) {
        Ok(_) => panic!("workspace preview replay should collide"),
        Err(error) => error,
    };
    assert_session_label_collision(
        preview_collision,
        ForgeQueryAuthorityLane::PreviewTruth,
        &label,
    );

    let branch_collision = match workspace.branch(label.clone()) {
        Ok(_) => panic!("workspace branch replay should collide"),
        Err(error) => error,
    };
    assert_session_label_collision(
        branch_collision,
        ForgeQueryAuthorityLane::BranchLocalTruth,
        &label,
    );

    let render_collision_left =
        ForgeQuerySessionLabel::scoped_strs("worth.kernel", ["preview"]).expect("label");
    let render_collision_right =
        ForgeQuerySessionLabel::scoped_strs("worth", ["kernel", "preview"]).expect("label");
    workspace
        .preview(render_collision_left.clone())
        .expect("workspace should admit left render-collision label")
        .discard();
    workspace
        .preview(render_collision_right.clone())
        .expect("workspace should admit right render-collision label")
        .discard();
    assert_eq!(
        render_collision_left.display(),
        render_collision_right.display()
    );
    assert_ne!(
        render_collision_left.identity_digest(),
        render_collision_right.identity_digest()
    );
}

#[test]
fn equivalent_preview_session_label_replay_stops_with_typed_collision_class() {
    let mut runtime = session_entry_runtime();
    let label = test_session_label("preview-collision");

    runtime
        .preview(label.clone())
        .expect("first preview label admission should succeed")
        .discard();

    let error = match runtime.preview(label.clone()) {
        Ok(_) => panic!("re-admitting equivalent preview label should collide"),
        Err(error) => error,
    };

    assert_session_label_collision(error, ForgeQueryAuthorityLane::PreviewTruth, &label);
}

#[test]
fn session_label_collision_is_scoped_per_family_and_not_by_rendered_display() {
    let mut runtime = session_entry_runtime();
    let preview_label =
        ForgeQuerySessionLabel::scoped_strs("worth.kernel", ["preview"]).expect("label");
    let render_collision =
        ForgeQuerySessionLabel::scoped_strs("worth", ["kernel", "preview"]).expect("label");

    runtime
        .preview(preview_label.clone())
        .expect("first preview label should admit")
        .discard();
    runtime
        .preview(render_collision.clone())
        .expect("display-colliding but identity-distinct preview label should admit")
        .discard();
    runtime
        .branch(preview_label.clone())
        .expect("branch family should admit same identity independently");

    let error = match runtime.branch(preview_label.clone()) {
        Ok(_) => panic!("re-admitting the same branch label should collide"),
        Err(error) => error,
    };
    assert_session_label_collision(
        error,
        ForgeQueryAuthorityLane::BranchLocalTruth,
        &preview_label,
    );
    assert_ne!(
        preview_label.identity_digest(),
        render_collision.identity_digest()
    );
    assert_eq!(preview_label.display(), render_collision.display());
}
