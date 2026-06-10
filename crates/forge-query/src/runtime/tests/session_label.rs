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

fn basis_digest(
    scope: ForgeQueryEvidenceScope,
    label: &ForgeQuerySessionLabel,
    effect_policy: ForgeQueryEffectPolicy,
    authority_lane: ForgeQueryAuthorityLane,
    evidence_rows: &[crate::runtime::ForgeQueryBasisAdmissionEvidenceRow],
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(scope)
        .field_identity(
            ForgeQueryEvidenceTag::new("session_label_identity"),
            label.identity_digest().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            effect_policy.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("authority_lane"),
            authority_lane.as_str(),
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("evidence_row"),
            evidence_rows
                .iter()
                .map(|row| row.row_digest().as_str()),
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
    let preview_manual = basis_digest(
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
        preview.basis_admission().admission_digest(),
        &preview_manual
    );
    drop(preview);

    let branch = runtime
        .branch_with_options(
            label.clone(),
            ForgeQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("branch should admit the same typed label in its own family");
    let branch_manual = basis_digest(
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
    assert_eq!(branch.basis_admission().admission_digest(), &branch_manual);
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
    let preview_manual = basis_digest(
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
        preview.basis_admission().admission_digest(),
        &preview_manual
    );
    drop(preview);

    let branch = workspace
        .branch_with_options(
            label.clone(),
            ForgeQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("workspace branch should admit same typed label in its own family");
    let branch_manual = basis_digest(
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
    assert_eq!(branch.basis_admission().admission_digest(), &branch_manual);
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

#[test]
fn display_colliding_preview_labels_produce_distinct_closeout_digests() {
    let mut runtime = session_entry_runtime();
    let left = ForgeQuerySessionLabel::scoped_strs("worth.kernel", ["preview"]).expect("label");
    let right = ForgeQuerySessionLabel::scoped_strs("worth", ["kernel", "preview"]).expect("label");

    let left_outcome = runtime
        .preview(left.clone())
        .expect("left preview should admit")
        .discard();
    let right_outcome = runtime
        .preview(right.clone())
        .expect("right preview should admit")
        .discard();

    assert_eq!(left.display(), right.display());
    assert_ne!(left.identity_digest(), right.identity_digest());
    assert_ne!(
        left_outcome.closeout_evidence().closeout_digest(),
        right_outcome.closeout_evidence().closeout_digest(),
        "preview closeout evidence must preserve canonical label identity"
    );
}

#[test]
fn display_colliding_preview_labels_produce_distinct_execution_digests() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>(
            "tasks.render-collision-execution",
            task_live_request(),
            task_schema(),
        )
        .expect("live view should declare");
    let left = ForgeQuerySessionLabel::scoped_strs("worth.kernel", ["preview"]).expect("label");
    let right = ForgeQuerySessionLabel::scoped_strs("worth", ["kernel", "preview"]).expect("label");

    let left_execution = {
        let mut preview = runtime
            .preview_with_options(left.clone(), ForgeQueryPreviewOptions::redirected_delivery())
            .expect("left preview should admit");
        preview.use_view(&live);
        preview
            .write(insert_command(
                "Task",
                [
                    ("identity.id", json!("left-render-collision")),
                    ("title.value", json!("Left render collision")),
                ],
            ))
            .expect("left preview write should stage");
        preview
            .preview_execution_evidence()
            .iter()
            .find(|evidence| evidence.kind() == ForgeQueryPreviewExecutionKind::LivePatch)
            .map(|evidence| evidence.execution_digest().to_string())
            .expect("left preview should record live-patch execution evidence")
    };
    let right_execution = {
        let mut preview = runtime
            .preview_with_options(
                right.clone(),
                ForgeQueryPreviewOptions::redirected_delivery(),
            )
            .expect("right preview should admit");
        preview.use_view(&live);
        preview
            .write(insert_command(
                "Task",
                [
                    ("identity.id", json!("right-render-collision")),
                    ("title.value", json!("Right render collision")),
                ],
            ))
            .expect("right preview write should stage");
        preview
            .preview_execution_evidence()
            .iter()
            .find(|evidence| evidence.kind() == ForgeQueryPreviewExecutionKind::LivePatch)
            .map(|evidence| evidence.execution_digest().to_string())
            .expect("right preview should record live-patch execution evidence")
    };

    assert_eq!(left.display(), right.display());
    assert_ne!(left.identity_digest(), right.identity_digest());
    assert_ne!(
        left_execution, right_execution,
        "preview execution evidence must preserve canonical label identity"
    );
}

#[test]
fn ordinary_runtime_entrypoints_require_typed_session_labels() {
    use crate::application::{
        normalize_source_text, ordinary_session_entrypoint_audit_violations,
    };

    let runtime_sessions = normalize_source_text(include_str!("../runtime_sessions.rs"));
    let workspace = normalize_source_text(include_str!("../workspace.rs"));
    let violations =
        ordinary_session_entrypoint_audit_violations(&runtime_sessions, &workspace);
    assert!(
        violations.is_empty(),
        "ordinary runtime session entrypoint audit failed: {violations:?}"
    );
}

#[test]
fn canonical_session_label_intake_phase_six_outputs_are_non_empty_and_stable() {
    use crate::application::{
        normalize_source_text, ordinary_session_entrypoint_audit_violations,
    };

    let mut runtime = session_entry_runtime();
    let preview_label = test_session_label("phase-six-preview");
    let branch_label = test_session_label("phase-six-branch");
    let render_collision_left =
        ForgeQuerySessionLabel::scoped_strs("worth.kernel", ["preview"]).expect("label");
    let render_collision_right =
        ForgeQuerySessionLabel::scoped_strs("worth", ["kernel", "preview"]).expect("label");

    let preview = runtime
        .preview_with_options(
            preview_label.clone(),
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview should admit typed label");
    let preview_session_basis_digest = preview.basis_admission().admission_digest().to_string();
    drop(preview);

    let branch = runtime
        .branch_with_options(
            branch_label.clone(),
            ForgeQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("branch should admit typed label");
    let branch_session_basis_digest = branch.basis_admission().admission_digest().to_string();
    drop(branch);

    runtime
        .preview(render_collision_left.clone())
        .expect("left render-collision label should admit")
        .discard();
    let render_collision_outcome = runtime
        .preview(render_collision_right.clone())
        .expect("right render-collision label should admit")
        .discard();
    let render_collision_admission_digest = render_collision_outcome
        .closeout_evidence()
        .closeout_digest()
        .to_string();

    let collision_error = match runtime.preview(preview_label.clone()) {
        Ok(_) => panic!("same-family replay should collide"),
        Err(error) => error,
    };
    let session_label_collision_stop_class = match collision_error.stop_class() {
        ForgeQueryStopClass::SessionLabelCollision {
            authority_lane,
            label,
        } => format!(
            "{}:{}",
            authority_lane.as_str(),
            label.identity_digest().as_str()
        ),
        other => panic!("expected session-label collision stop class, got {other:?}"),
    };

    let runtime_sessions = normalize_source_text(include_str!("../runtime_sessions.rs"));
    let workspace = normalize_source_text(include_str!("../workspace.rs"));
    let raw_string_entrypoint_audit = crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::ApplicationSessionLabelBoundaryClosure,
    )
    .field_identity_sequence(
        crate::ForgeQueryEvidenceTag::new("entrypoint_audit_violation"),
        ordinary_session_entrypoint_audit_violations(&runtime_sessions, &workspace),
    )
    .seal()
    .as_str()
    .to_string();

    assert!(!preview_session_basis_digest.is_empty());
    assert!(!branch_session_basis_digest.is_empty());
    assert!(!session_label_collision_stop_class.is_empty());
    assert!(!render_collision_admission_digest.is_empty());
    assert!(!raw_string_entrypoint_audit.is_empty());

    assert_ne!(preview_session_basis_digest, branch_session_basis_digest);
    assert_ne!(
        render_collision_left.identity_digest(),
        render_collision_right.identity_digest()
    );
    assert_eq!(
        render_collision_left.display(),
        render_collision_right.display()
    );
}
